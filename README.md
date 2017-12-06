# storage_event_monitor
Proof of concept for utilizing the journal to generate meaningful alerts to end users


### Example of adding a journal entry for a usb drive being removed

```json
{
        "__CURSOR" : "s=6fc04e4e95fc4e8bb4ad23bcd46a6772;i=9a362;b=45906ac56b6d4d849d11eb9824efcf44;m=a6d6ee2e;t=55a913dc91892;x=a41f19fcedf414f7",
        "__REALTIME_TIMESTAMP" : "1506954736900242",
        "__MONOTONIC_TIMESTAMP" : "2799103534",
        "_BOOT_ID" : "45906ac56b6d4d849d11eb9824efcf44",
        "_UID" : "17209",
        "_GID" : "17209",
        "_CAP_EFFECTIVE" : "0",
        "_AUDIT_LOGINUID" : "17209",
        "_SYSTEMD_OWNER_UID" : "17209",
        "_SYSTEMD_SLICE" : "user-17209.slice",
        "_SYSTEMD_USER_SLICE" : "-.slice",
        "_MACHINE_ID" : "2b0ea69bcbf84a00aa6d71cb61d84f43",
        "_HOSTNAME" : "localhost.localdomain",
        "PRIORITY" : "6",
        "_TRANSPORT" : "journal",
        "_SELINUX_CONTEXT" : "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023",
        "_AUDIT_SESSION" : "2",
        "_SYSTEMD_CGROUP" : "/user.slice/user-17209.slice/session-2.scope",
        "_SYSTEMD_SESSION" : "2",
        "_SYSTEMD_UNIT" : "session-2.scope",
        "_SYSTEMD_INVOCATION_ID" : "d5daa32884fc4130a08a3a996b7232bc",
        "MESSAGE_ID" : "3183267b90074a4595e91daef0e01462",
        "DETAILS" : "",
        "SYSLOG_IDENTIFIER" : "storage_event_monitor",
        "_PID" : "30094",
        "_COMM" : "storage_event_m",
        "_EXE" : "/home/tasleson/projects/ffi/storage_event_monitor/target/debug/storage_event_monitor",
        "_CMDLINE" : "target/debug/storage_event_monitor",
        "DEVICE" : "/dev/sdg",
        "DEVICE_ID" : "SN: 415DEF01223547070904",
        "STATE" : "discovery",
        "SOURCE" : "storage_event_monitor",
        "SOURCE_MAN" : "",
        "PRIORITY_DESC" : "info",
        "MESSAGE" : "Annotation: Storage device removed",
        "_SOURCE_REALTIME_TIMESTAMP" : "1506954736900007"
}
```

### Example mdraid

```json
{
        "__CURSOR" : "s=6fc04e4e95fc4e8bb4ad23bcd46a6772;i=9a4be;b=45906ac56b6d4d849d11eb9824efcf44;m=165a9e339;t=55a91fc9c0d9d;x=9c24fefc27bb41ab",
        "__REALTIME_TIMESTAMP" : "1506957938396573",
        "__MONOTONIC_TIMESTAMP" : "6000599865",
        "_BOOT_ID" : "45906ac56b6d4d849d11eb9824efcf44",
        "PRIORITY" : "5",
        "_UID" : "17209",
        "_GID" : "17209",
        "_CAP_EFFECTIVE" : "0",
        "_AUDIT_LOGINUID" : "17209",
        "_SYSTEMD_OWNER_UID" : "17209",
        "_SYSTEMD_SLICE" : "user-17209.slice",
        "_SYSTEMD_USER_SLICE" : "-.slice",
        "_MACHINE_ID" : "2b0ea69bcbf84a00aa6d71cb61d84f43",
        "_HOSTNAME" : "localhost.localdomain",
        "_TRANSPORT" : "journal",
        "_SELINUX_CONTEXT" : "unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023",
        "_AUDIT_SESSION" : "2",
        "_SYSTEMD_CGROUP" : "/user.slice/user-17209.slice/session-2.scope",
        "_SYSTEMD_SESSION" : "2",
        "_SYSTEMD_UNIT" : "session-2.scope",
        "_SYSTEMD_INVOCATION_ID" : "d5daa32884fc4130a08a3a996b7232bc",
        "MESSAGE_ID" : "3183267b90074a4595e91daef0e01462",
        "DEVICE" : "",
        "DEVICE_ID" : "",
        "STATE" : "rebuilt",
        "SOURCE" : "kernel",
        "SOURCE_MAN" : "man 8 mdadm",
        "DETAILS" : "",
        "PRIORITY_DESC" : "notice",
        "MESSAGE" : "Storage error addendum for (md: recovery of RAID array md0)",
        "SYSLOG_IDENTIFIER" : "storage_event_monitor",
        "_COMM" : "storage_event_m",
        "_EXE" : "/home/tasleson/projects/ffi/storage_event_monitor/target/debug/storage_event_monitor",
        "_CMDLINE" : "target/debug/storage_event_monitor",
        "_PID" : "30948",
        "_SOURCE_REALTIME_TIMESTAMP" : "1506957938396503"
}

```
