// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate sdjournal;

fn main() {
    let mut journal = sdjournal::Journal::new().expect("Failed to open systemd journal");

    // Default is for iterator to wait forever, you can control it with the timeout_us value
    journal.timeout_us = 5000000;

    // Iterate through all the systemd journal entries and new ones as they show up
    for i in journal {
        match i {
            // TODO: Take the log entry and make something useful happen
            Ok(msg) => println!("{}", msg),
            Err(e) => {
                println!("Error= {}", e);
                break;
            }
        }
    }
}
