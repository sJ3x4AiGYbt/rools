use crate::commands::sysviz::common::alert;
use crate::commands::sysviz::common::config::{self as cfg, SystemMonitor};
use crate::commands::sysviz::common::constants;
use crate::commands::sysviz::common::formatter;
use crate::commands::sysviz::common::stats;
use crate::global::export;
use crate::global::permissions;
use crate::global::shutdown as shutdown_mod;
use chrono::DateTime;
use chrono::Local;
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;
use std::sync::mpsc::{SyncSender, sync_channel};
use windows::Win32::Foundation::*;
use windows::Win32::Security::*;
use windows::Win32::System::Diagnostics::Etw::*;
use windows::Win32::System::Threading::*;
use windows::core::{GUID, PCWSTR, PWSTR};

static ETW_SENDER: Mutex<Option<SyncSender<RawEtwEvent>>> = Mutex::new(None);

#[derive(Debug, Clone)]
struct RawEtwEvent {
  timestamp: i64,
  pid: u32,
  syscall_id: u32,
}

pub struct WindowsMonitor;

impl SystemMonitor for WindowsMonitor {
  fn run(&mut self, config: &cfg::SysVizConfig) {
    let shutdown = shutdown_mod::Shutdown::new(config.timeout);

    permissions::ensure_root();
    println!("{}", self.start_message());

    let fmt = config.get_formatter();
    fmt.header();

    let mut stats_collector = stats::Stats::new();
    let alert_manager = alert::Alert::new(config.alert.clone());
    let syscall_table = build_syscall_table();

    let mut process_cache: HashMap<u32, (String, String)> = HashMap::new();

    let (tx, rx) = sync_channel::<RawEtwEvent>(2000);

    {
      *ETW_SENDER.lock().unwrap() = Some(tx);
    }

    let (session_handle, trace_handle) = match unsafe { self.start_etw(config) } {
      Ok(handles) => handles,
      Err(e) => {
        eprintln!("{}: {}", constants::platform::ERROR_WINDOWS_ETW_START, e);
        return;
      }
    };

    std::thread::spawn(move || unsafe {
      let _ = ProcessTrace(&[trace_handle], None, None);
    });

    let mut line_count = 0;
    let rotation_limit = 10000;

    while let Ok(raw) = rx.recv() {
      if !shutdown.is_running() {
        break;
      }

      if let Some(target_pid) = config.pid_filter {
        if raw.pid != target_pid {
          continue;
        }
      }

      let syscall_name = syscall_table
        .get(&raw.syscall_id)
        .cloned()
        .unwrap_or_else(|| format!("syscall_{:#06x}", raw.syscall_id));

      if let Some(ref filter) = config.name_filter {
        if !syscall_name.eq_ignore_ascii_case(filter) {
          continue;
        }
      }

      let (process_name, user) = process_cache
        .entry(raw.pid)
        .or_insert_with(|| (get_process_name(raw.pid), get_process_user(raw.pid)))
        .clone();

      let event = formatter::Event {
        timestamp: filetime_to_string(raw.timestamp),
        source: syscall_name,
        pid: raw.pid,
        responsible_pid: raw.pid,
        user,
        process_name,
      };

      alert_manager.check_and_trigger(&event.source);

      if config.stats {
        stats_collector.increment(event.source.clone());
      }

      fmt.format(&event);
      line_count += 1;

      if line_count >= rotation_limit {
        fmt.footer();
        if let Err(e) = export::Export::finalize(&config.export_path, config.zip) {
          eprintln!("{} {}", constants::ERROR_ROTATION_FAILED, e);
        }
        fmt.header();
        line_count = 0;
      }
    }

    unsafe {
      let _ = CloseTrace(trace_handle);
      let session_name: Vec<u16> = constants::platform::ETW_SESSION_NAME
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
      let _ = StopTraceW(
        session_handle,
        PCWSTR(session_name.as_ptr()),
        std::ptr::null_mut(),
      );
    }

    *ETW_SENDER.lock().unwrap() = None;

    fmt.footer();

    if config.stats {
      fmt.stats(stats_collector.get_counts(), config.top);
    }

    if let Err(e) = export::Export::finalize(&config.export_path, config.zip) {
      eprintln!("{} {}", constants::ERROR_EXPORT_FAILED, e);
    }
  }

  fn default_syscalls(&self) -> Vec<&'static str> {
    Vec::new()
  }

  fn start_message(&self) -> &'static str {
    constants::platform::MSG_START_WINDOWS
  }
}

impl WindowsMonitor {
  pub fn new() -> Self {
    Self
  }

  unsafe fn start_etw(
    &self,
    _config: &cfg::SysVizConfig,
  ) -> Result<(CONTROLTRACE_HANDLE, PROCESSTRACE_HANDLE), String> {
    let session_name: Vec<u16> = constants::platform::ETW_SESSION_NAME
      .encode_utf16()
      .chain(std::iter::once(0))
      .collect();

    let name_bytes = session_name.len() * 2;
    let buf_size = mem::size_of::<EVENT_TRACE_PROPERTIES>() + name_bytes + 512;
    let mut buf = vec![0u8; buf_size];
    let props = buf.as_mut_ptr() as *mut EVENT_TRACE_PROPERTIES;

    (*props).Wnode.BufferSize = buf_size as u32;
    (*props).Wnode.Flags = WNODE_FLAG_TRACED_GUID;
    (*props).Wnode.ClientContext = 1;

    (*props).Wnode.Guid = GUID {
      data1: 0x9E814AAD,
      data2: 0x3204,
      data3: 0x11D2,
      data4: [0x9A, 0x82, 0x00, 0x60, 0x08, 0xA8, 0x69, 0x39],
    };

    (*props).LogFileMode = EVENT_TRACE_REAL_TIME_MODE;
    (*props).EnableFlags = EVENT_TRACE_FLAG_SYSTEMCALL;
    (*props).LoggerNameOffset = mem::size_of::<EVENT_TRACE_PROPERTIES>() as u32;

    let name_dst = (props as *mut u8).add((*props).LoggerNameOffset as usize) as *mut u16;
    std::ptr::copy_nonoverlapping(session_name.as_ptr(), name_dst, session_name.len());

    let session_name_pcwstr = PCWSTR(name_dst);

    let _ = StopTraceW(CONTROLTRACE_HANDLE { Value: 0 }, session_name_pcwstr, props);

    let mut session_handle = CONTROLTRACE_HANDLE { Value: 0 };
    let start_result = StartTraceW(&mut session_handle, session_name_pcwstr, props);
    if start_result != WIN32_ERROR(0) {
      return Err(format!("StartTraceW failed: {:?}", start_result));
    }

    let mut log_file: EVENT_TRACE_LOGFILEW = mem::zeroed();
    log_file.LoggerName = PWSTR(name_dst);
    log_file.Anonymous1.ProcessTraceMode =
      PROCESS_TRACE_MODE_REAL_TIME | PROCESS_TRACE_MODE_EVENT_RECORD;
    log_file.Anonymous2.EventRecordCallback = Some(etw_event_callback);

    let trace_handle = OpenTraceW(&mut log_file);
    if trace_handle.Value == u64::MAX {
      let _ = StopTraceW(session_handle, session_name_pcwstr, std::ptr::null_mut());
      return Err("OpenTraceW failed".to_string());
    }

    Ok((session_handle, trace_handle))
  }
}

unsafe extern "system" fn etw_event_callback(event: *mut EVENT_RECORD) {
  let event = unsafe { &*event };

  if event.EventHeader.EventDescriptor.Opcode != 51 {
    return;
  }

  if event.UserDataLength < 4 {
    return;
  }
  let syscall_id = unsafe { *(event.UserData as *const u32) };

  let raw = RawEtwEvent {
    timestamp: event.EventHeader.TimeStamp,
    pid: event.EventHeader.ProcessId,
    syscall_id,
  };

  if let Ok(guard) = ETW_SENDER.lock() {
    if let Some(tx) = guard.as_ref() {
      let _ = tx.try_send(raw);
    }
  }
}

fn filetime_to_string(filetime: i64) -> String {
  const EPOCH_DIFF: i64 = 11_644_473_600;
  let secs = (filetime / 10_000_000) - EPOCH_DIFF;
  let nanos = ((filetime % 10_000_000) * 100) as u32;

  DateTime::from_timestamp(secs, nanos)
    .map(|dt| {
      dt.with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S%.6f")
        .to_string()
    })
    .unwrap_or_else(|| "unknown".to_string())
}

fn get_process_name(pid: u32) -> String {
  unsafe {
    let handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid) {
      Ok(h) => h,
      Err(_) => return "unknown".to_string(),
    };

    let mut buf = vec![0u16; 512];
    let mut size = buf.len() as u32;

    if QueryFullProcessImageNameW(
      handle,
      PROCESS_NAME_WIN32,
      PWSTR(buf.as_mut_ptr()),
      &mut size,
    )
    .is_ok()
    {
      let path = String::from_utf16_lossy(&buf[..size as usize]);
      path.split('\\').last().unwrap_or("unknown").to_string()
    } else {
      "unknown".to_string()
    }
  }
}

fn get_process_user(pid: u32) -> String {
  unsafe {
    let proc_handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid) {
      Ok(h) => h,
      Err(_) => return "unknown".to_string(),
    };

    let mut token = HANDLE::default();
    if OpenProcessToken(proc_handle, TOKEN_QUERY, &mut token).is_err() {
      return "unknown".to_string();
    }

    let mut needed = 0u32;
    let _ = GetTokenInformation(token, TokenUser, None, 0, &mut needed);

    let mut buf = vec![0u8; needed as usize];
    if GetTokenInformation(
      token,
      TokenUser,
      Some(buf.as_mut_ptr() as *mut _),
      needed,
      &mut needed,
    )
    .is_err()
    {
      return "unknown".to_string();
    }

    let token_user = &*(buf.as_ptr() as *const TOKEN_USER);
    sid_to_username(token_user.User.Sid)
  }
}

unsafe fn sid_to_username(sid: PSID) -> String {
  let mut name = vec![0u16; 256];
  let mut domain = vec![0u16; 256];
  let mut name_len = name.len() as u32;
  let mut domain_len = domain.len() as u32;
  let mut sid_type = SID_NAME_USE::default();

  if LookupAccountSidW(
    PCWSTR::null(),
    sid,
    PWSTR(name.as_mut_ptr()),
    &mut name_len,
    PWSTR(domain.as_mut_ptr()),
    &mut domain_len,
    &mut sid_type,
  )
  .is_ok()
  {
    String::from_utf16_lossy(&name[..name_len as usize])
  } else {
    "unknown".to_string()
  }
}

fn build_syscall_table() -> HashMap<u32, String> {
  [
    (0x0000, "NtReadFile"),
    (0x0001, "NtWriteFile"),
    (0x000C, "NtOpenFile"),
    (0x0012, "NtCreateFile"),
    (0x0013, "NtDeleteFile"),
    (0x000A, "NtQueryInformationFile"),
    (0x0030, "NtSetInformationFile"),
    (0x0026, "NtFlushBuffersFile"),
    (0x0041, "NtQueryDirectoryFile"),
    (0x0002, "NtDeviceIoControlFile"),
    (0x0018, "NtAllocateVirtualMemory"),
    (0x001C, "NtFreeVirtualMemory"),
    (0x001D, "NtQueryVirtualMemory"),
    (0x0019, "NtProtectVirtualMemory"),
    (0x002C, "NtReadVirtualMemory"),
    (0x003A, "NtWriteVirtualMemory"),
    (0x0023, "NtCreateProcess"),
    (0x000D, "NtCreateUserProcess"),
    (0x004B, "NtOpenProcess"),
    (0x0050, "NtTerminateProcess"),
    (0x0004, "NtQueryInformationProcess"),
    (0x0055, "NtSetInformationProcess"),
    (0x0029, "NtCreateThread"),
    (0x010B, "NtCreateThreadEx"),
    (0x0048, "NtOpenThread"),
    (0x0053, "NtTerminateThread"),
    (0x003B, "NtQueryInformationThread"),
    (0x005B, "NtSetInformationThread"),
    (0x0057, "NtSuspendThread"),
    (0x0058, "NtResumeThread"),
    (0x0059, "NtGetContextThread"),
    (0x005A, "NtSetContextThread"),
    (0x0003, "NtWaitForSingleObject"),
    (0x0007, "NtWaitForMultipleObjects"),
    (0x000F, "NtCreateMutant"),
    (0x0040, "NtReleaseMutant"),
    (0x0015, "NtCreateEvent"),
    (0x000E, "NtSetEvent"),
    (0x0034, "NtResetEvent"),
    (0x005D, "NtCreateSemaphore"),
    (0x00B0, "NtReleaseSemaphore"),
    (0x005E, "NtCreateTimer"),
    (0x00C7, "NtSetTimer"),
    (0x0063, "NtQueryTimer"),
    (0x0028, "NtCreateKey"),
    (0x0077, "NtOpenKey"),
    (0x0114, "NtDeleteKey"),
    (0x003F, "NtQueryValueKey"),
    (0x0025, "NtSetValueKey"),
    (0x003D, "NtDeleteValueKey"),
    (0x0006, "NtClose"),
    (0x003C, "NtDuplicateObject"),
    (0x0035, "NtQueryObject"),
    (0x000B, "NtCreateSection"),
    (0x0028, "NtMapViewOfSection"),
    (0x002A, "NtUnmapViewOfSection"),
    (0x0060, "NtConnectPort"),
    (0x001A, "NtCreatePipe"),
    (0x0074, "NtCreateNamedPipeFile"),
    (0x001E, "NtCreateIoCompletion"),
    (0x0009, "NtSetIoCompletion"),
    (0x0008, "NtRemoveIoCompletion"),
    (0x0005, "NtQuerySystemInformation"),
    (0x0014, "NtCreateSymbolicLinkObject"),
    (0x0049, "NtOpenSymbolicLinkObject"),
    (0x0039, "NtQuerySymbolicLinkObject"),
  ]
  .into_iter()
  .map(|(id, name)| (id, name.to_string()))
  .collect()
}
