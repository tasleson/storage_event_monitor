// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate sdjournal;
extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::process::Command;

extern crate libudev;

use regex::Regex;
use std::ptr;
use libc::{POLLIN, POLLERR, POLLHUP, POLLNVAL};
use std::path::Path;
use std::io::Error;
use std::os::unix::io::AsRawFd;

pub static MSG_STORAGE_ID: &'static str = "3183267b90074a4595e91daef0e01462";

use libc::{c_void, c_int, c_short, c_ulong};

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

#[repr(C)]
struct sigset_t {
    __private: c_void
}

#[allow(non_camel_case_types)]
type nfds_t = c_ulong;

extern "C" {
    fn ppoll(fds: *mut pollfd, nfds: nfds_t, timeout_ts: *mut libc::timespec, sigmask: *const sigset_t) -> c_int;
}

fn udev_settle() {
    Command::new("/usr/bin/udevadm")
        .arg("settle")
        .spawn()
        .expect("Failed to do a udev settle");
}

enum IdLookup {
    DevNode = 1,
    PathId = 2,
}

fn fetch_durable_name(device: &libudev::Device) -> Option<String> {
    let mut wwid = device.property_value("ID_WWN");
    match wwid {
        Some(w) => return Some(String::from(w.to_str().unwrap())),
        None => {
            wwid = device.property_value("ID_SERIAL_SHORT");
            match wwid {
                Some(w) => return Some(String::from(w.to_str().unwrap())),
                None => ()
            }
        }
    }
    None
}

fn id_lookup(search_name: &str, mode: IdLookup) -> Option<String> {
    udev_settle();

    let context = libudev::Context::new().unwrap();

    let mut enumerator = libudev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("block").unwrap();
    enumerator.match_property("DEVTYPE", "disk").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        let mut found = false;

        match mode {
            IdLookup::PathId => {
                let str_path = device.syspath().to_str().unwrap();
                if str_path.contains(search_name) {
                    found = true;
                }
            }
            IdLookup::DevNode => {
                let dev_node = String::from(device.devnode().unwrap_or(Path::new("")).to_str().unwrap_or(""));
                if dev_node == search_name || dev_node.ends_with(search_name) {
                    found = true;
                }
            }
        }

        if found {
            // We may not have a very good durable name for some devices, what to do ...
            return fetch_durable_name(&device);
        }
    }

    None
}

fn id_for_devnode(dev_name: &str) -> Option<String> {
    id_lookup(dev_name, IdLookup::DevNode)
}

/// Given a path id eg. 0000:3e:00.0 find the WWID, SN or something fairly durable.
/// Note: Make sure to remove any leading identifiers if we are pulling this out of a journal entry
fn id_for_path_id(device_id: &str) -> Option<String> {
    id_lookup(device_id, IdLookup::PathId)
}

fn process_journal_entry(journal_entry: &HashMap<String, String>) {
    // Take a look at the message and filter for storage messages we are interested in.
    // There are lots of different way to search, lets start simple.
    //
    // This is horribly error prone
    // and we really need to add structured data to the error messages themselves.
    let message = "Storage error addendum";
    let source = "kernel";
    let mut source_man = "";
    let mut log = false;
    let mut device = String::from("");
    let mut device_id = String::new();
    let mut details = String::from("");
    let mut state = "unknown";
    let mut priority = sdjournal::JournalPriority::Critical;

    if !journal_entry.contains_key("MESSAGE") {
        // Not sure how this happens, but apparently it does!
        return;
    }

    let log_entry = journal_entry.get("MESSAGE").unwrap();
    let log_entry_str = log_entry.as_str();

    // Check to see if this entry is one we may have created
    if journal_entry.contains_key("MESSAGE_ID") {
        if journal_entry.get("MESSAGE_ID").unwrap() == MSG_STORAGE_ID {
            return;
        }
    }

    /*

    Sample journal message to UA
    sd 3:0:0:0: Warning! Received an indication that the LUN assignments on this target have changed. (...)

    Sample journal message for Media error
    blk_update_request: critical medium error, dev sdr, sector 4656
    Buffer I/O error on dev sdr, logical block 582, async page read

    */

    lazy_static! {
        static ref UA_MSG: Regex = Regex::new("^[a-z]+ ([0-9:]+): Warning! Received an indication that the (.+)").unwrap();
        static ref TARGET_ERRORS: Regex = Regex::new("^blk_update_request: ([a-z A-Z/]+) error, dev ([a-z]+), sector ([0-9]+)$").unwrap();
    }

    if TARGET_ERRORS.is_match(log_entry_str) {
        log = true;
        source_man = "see: block/blk-core.c";
        priority = sdjournal::JournalPriority::Critical;
        state = "failing";

        let m = TARGET_ERRORS.captures(log_entry_str).unwrap();
        device = String::from(&m[1]);
        let device_id_lookup = id_for_devnode(device.as_str());

        device_id = match device_id_lookup {
            None => String::from(""),
            Some(ret) => ret,
        };

        details = format!("Device block {} is in question!", &m[3]);
    } else if UA_MSG.is_match(log_entry_str) {
        log = true;
        source_man = "see: drivers/scsi/scsi_error.c";
        priority = sdjournal::JournalPriority::Info;
        state = "discovery";

        let m = UA_MSG.captures(log_entry_str).unwrap();
        device = String::from(&m[1]);

        let device_id_lookup = id_for_path_id(device.as_str());
        device_id = match device_id_lookup {
            None => String::from(""),
            Some(ret) => ret,
        };
    }

    // Log the additional information to the journal
    if log {
        let result = sdjournal::send_journal_basic(MSG_STORAGE_ID,
                                                   message, source, source_man, device.as_str(), device_id.as_str(),
                                                   state, priority, details);

        match result {
            Ok(_) => println!("DEBUG: Added an annotated journal entry!"),
            Err(result) => println!("Error adding journal entry: {}", result),
        }
    }

    println!("DEBUG: JOURNAL_ENTRY({})", log_entry);
}

fn log_disk_add_remove(device: &libudev::Device, msg: &'static str, durable_name: &str) {
    let result = sdjournal::send_journal_basic(MSG_STORAGE_ID,
                                               format!("Annotation: Storage device {}", msg).as_str(),
                                               "storage_event_monitor",
                                               "",
                                               &String::from(device.devnode().unwrap_or(Path::new("")).to_str().unwrap_or("Unknown")),
                                               durable_name,
                                               "discovery",
                                               sdjournal::JournalPriority::Info,
                                               String::from(""));

        match result {
            Ok(_) => (),
            Err(result) => println!("Error adding journal entry: {}", result),
        }
}

// We will focus on block devices that are added or removed, when this happens we will add a journal
// entry that adds more information about the device, mainly the durable name.
fn process_udev_entry(event: &libudev::Event) {
    match event.event_type() {
        libudev::EventType::Add => {
            let durable_name = fetch_durable_name(event.device());

            match durable_name {
                Some(name) => {
                    // Add annotated journal entry
                    log_disk_add_remove(event.device(), "added", name.as_str());
                }
                None => ()
            }
        }
        libudev::EventType::Remove => {
            let durable_name = fetch_durable_name(event.device()).unwrap_or(String::from("Unknown"));
            log_disk_add_remove(event.device(), "removed", durable_name.as_str());
        }
        _ => ()
    }
}

fn main() {
    // Setup the connection for journal entries
    let mut journal = sdjournal::Journal::new().expect("Failed to open systemd journal");
    // We never want to block, so set the timeout to 0
    journal.timeout_us = 0;
    // Jump to the end as we cannot annotate old journal entries.
    journal.seek_tail().expect("Unable to seek to end of journal!");


    // Setup a connection for udev events for block devices
    let context = libudev::Context::new().unwrap();
    let mut monitor = libudev::Monitor::new(&context).unwrap();
    monitor.match_subsystem_devtype("block", "disk").unwrap();
    let mut udev = monitor.listen().unwrap();

    let mut fds = vec!(pollfd { fd: udev.as_raw_fd(), events: POLLIN, revents: 0 },
                       pollfd { fd: journal.as_raw_fd(), events: journal.get_events_bit_mask(), revents: 0 });

    loop {
        let result = unsafe {
            ppoll((&mut fds[..]).as_mut_ptr(), fds.len() as nfds_t,
                  ptr::null_mut(), ptr::null())
        };

        if result < 0 {
            println!("ppoll: {:?}", Error::last_os_error());
            break;
        }

        if fds[0].revents != 0 {
            if fds[0].revents & (POLLERR | POLLHUP | POLLNVAL) != 0 {
                println!("Error in udev revents {}", fds[0].revents);
                break;
            }

            // Process udev
            loop {
                match udev.receive_event() {
                    Some(event) => {
                        process_udev_entry(&event);
                        println!("{}: {} {} (subsystem={}, sysname={}, devtype={})",
                                 event.sequence_number(),
                                 event.event_type(),
                                 event.syspath().to_str().unwrap_or("---"),
                                 event.subsystem().to_str().unwrap_or("unknown subsystem"),
                                 event.sysname().to_str().unwrap_or(""),
                                 event.devtype().map_or("", |s| { s.to_str().unwrap_or("") }));
                    }
                    None => {
                        break;
                    }
                };
            }
        }

        if fds[1].revents != 0 {
            if fds[1].revents & (POLLERR | POLLHUP | POLLNVAL) != 0 {
                println!("Error in journal revents {}", fds[1].revents);
                break;
            }

            // Process journal entries, need to figure out why we cannot use the iterator as we
            // are getting an error from the borrow checker about journal getting moved!
            loop {
                let entry = journal.get_next();
                match entry {
                    Some(entry) => {
                        match entry {
                            Ok(entry) => process_journal_entry(&entry),
                            Err(e) => println!("Error retrieving the journal entry: {}", e),
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
}
