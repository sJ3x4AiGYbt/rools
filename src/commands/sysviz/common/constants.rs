// #![allow(warnings)]
pub const BANNER_SYSVIZ: &str = r#"

.d88b.             Yb    dP w      
YPwww. Yb  dP d88b  Yb  dP  w 888P 
    d8  YbdP  `Yb.   YbdP   8  dP  
`Y88P'   dP   Y88P    YP    8 d888  
        dP                         
"#;

#[cfg(target_os = "linux")]
pub mod platform {
  pub const MSG_START_LINUX: &str = "[start]    monitoring with strace... (Press Ctrl+C to stop)";
  pub const ERROR_LINUX_CMD: &str =
    "[error]   failed to spawn strace. Is it installed? (apt install strace)";
  pub const ERROR_LINUX_READ_STREAM: &str = "[error]   reading strace stream";
  pub const ERROR_LINUX_NO_PIDS: &str = "[error]   could not enumerate PIDs from /proc";
  pub const DEFAULT_SYSCALLS: &[&str] = &[
    "execve",
    "execveat",
    "exit_group",
    "openat",
    "unlinkat",
    "renameat2",
  ];
}

#[cfg(target_os = "macos")]
pub mod platform {
  pub const MSG_START_MACOS: &str = "[start]    monitoring with eslogger... (Press Ctrl+C to stop)";
  pub const ERROR_MACOS_CMD: &str = "[error]   failed to start eslogger. Is it installed?";
  pub const ERROR_MACOS_READ_STREAM: &str = "[error]   reading event stream";
  pub const DEFAULT_SYSCALLS: &[&str] = &[
    "exec",
    "exit",
    "uipc_connect",
    "uipc_bind",
    "mmap",
    "xpc_connect",
    "open",
    "create",
    "rename",
    "unlink",
  ];
}

#[cfg(target_os = "windows")]
pub mod platform {
  pub const MSG_START_WINDOWS: &str = "[start]    monitoring with ETW... (Press Ctrl+C to stop)";
  pub const ERROR_WINDOWS_ETW_START: &str = "[error]   failed to start ETW session";
  pub const ERROR_WINDOWS_ETW_OPEN: &str = "[error]   failed to open ETW trace";
  pub const ETW_SESSION_NAME: &str = "NT Kernel Logger";
}

pub const ERROR_EXPORT_FAILED: &str = "[error]   export finalization failed:";
pub const ERROR_ROTATION_FAILED: &str = "[error]   log rotation failed:";

/* formatter.rs */
pub const TXT_HEADER_TIMESTAMP: &str = "TIMESTAMP";
pub const TXT_HEADER_PID: &str = "PID";
pub const TXT_HEADER_RESPONSIBLE_PID: &str = "R-PID";
pub const TXT_HEADER_SIGNING_ID: &str = "SIGNING ID (USER)";
pub const TXT_HEADER_OPERATION: &str = "OPERATION";
pub const TXT_HEADER_PROCESS_PATH: &str = "PROCESS PATH";
pub const TXT_HEADER_COUNT: &str = "COUNT";
pub const CSV_STATS_HEADER: &str = "operation,count";
pub const FIELD_TIMESTAMP: &str = "timestamp";
pub const FIELD_PID: &str = "pid";
pub const FIELD_RESPONSIBLE_PID: &str = "responsible_pid";
pub const FIELD_USER: &str = "user";
pub const FIELD_OPERATION: &str = "operation";
pub const FIELD_PROCESS_PATH: &str = "process_path";
pub const WARN_JSON: &str = "[warning] failed to serialize event to JSON:";

/* alert.rs */
pub const MSG_ALERT: &str = "[ALERT] DETECTED:";
