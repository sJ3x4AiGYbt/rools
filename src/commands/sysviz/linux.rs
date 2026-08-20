use crate::commands::sysviz::common::alert;
use crate::commands::sysviz::common::config::{self as cfg, SystemMonitor};
use crate::commands::sysviz::common::constants;
use crate::commands::sysviz::common::formatter;
use crate::commands::sysviz::common::stats;
use crate::global::export;
use crate::global::permissions;
use crate::global::shutdown as shutdown_mod;
use chrono::Local;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::sync_channel;

pub struct LinuxMonitor;

impl SystemMonitor for LinuxMonitor {
  fn run(&mut self, config: &cfg::SysVizConfig) {
    let shutdown = shutdown_mod::Shutdown::new(config.timeout);

    permissions::ensure_root();
    println!("{}", self.start_message());

    let fmt = config.get_formatter();
    fmt.header();

    let mut child = self.spawn_strace(config);

    let mut stats_collector = stats::Stats::new();
    let alert_manager = alert::Alert::new(config.alert.clone());

    if let Some(stderr) = child.stderr.take() {
      self.process_events(
        stderr,
        &shutdown,
        config,
        &fmt,
        &mut stats_collector,
        &alert_manager,
      );

      fmt.footer();

      if config.stats {
        fmt.stats(stats_collector.get_counts(), config.top);
      }

      if let Err(e) = export::Export::finalize(&config.export_path, config.zip) {
        eprintln!("{} {}", constants::ERROR_EXPORT_FAILED, e);
      }
    }
  }

  fn default_syscalls(&self) -> Vec<&'static str> {
    constants::platform::DEFAULT_SYSCALLS.to_vec()
  }

  fn start_message(&self) -> &'static str {
    constants::platform::MSG_START_LINUX
  }
}

impl LinuxMonitor {
  pub fn new() -> Self {
    Self
  }

  fn spawn_strace(&self, config: &cfg::SysVizConfig) -> std::process::Child {
    let syscalls = match &config.name_filter {
      Some(name) => name.clone(),
      None => self.default_syscalls().join(","),
    };

    let mut cmd = Command::new("strace");
    cmd
      .arg("-f")
      .arg("-tt")
      .arg("-e")
      .arg(format!("trace={}", syscalls))
      .stderr(Stdio::piped());

    match config.pid_filter {
      Some(pid) => {
        cmd.arg("-p").arg(pid.to_string());
      }
      None => {
        let pids = self.collect_all_pids();
        if pids.is_empty() {
          eprintln!("{}", constants::platform::ERROR_LINUX_NO_PIDS);
          std::process::exit(1);
        }
        for pid in &pids {
          cmd.arg("-p").arg(pid.to_string());
        }
      }
    }

    cmd.spawn().expect(constants::platform::ERROR_LINUX_CMD)
  }

  fn collect_all_pids(&self) -> Vec<u32> {
    std::fs::read_dir("/proc")
      .map(|entries| {
        entries
          .filter_map(|e| e.ok())
          .filter_map(|e| e.file_name().to_str()?.parse::<u32>().ok())
          .collect()
      })
      .unwrap_or_default()
  }

  fn process_events(
    &self,
    stderr: std::process::ChildStderr,
    shutdown: &shutdown_mod::Shutdown,
    config: &cfg::SysVizConfig,
    fmt: &Box<dyn formatter::Formatter>,
    stats_collector: &mut stats::Stats,
    alert_manager: &alert::Alert,
  ) {
    let (tx, rx) = sync_channel(1000);

    std::thread::spawn(move || {
      let reader = BufReader::new(stderr);
      for line in reader.lines() {
        match line {
          Ok(content) => {
            if tx.send(content).is_err() {
              break;
            }
          }
          Err(e) => {
            eprintln!("{}: {}", constants::platform::ERROR_LINUX_READ_STREAM, e);
            break;
          }
        }
      }
    });

    let mut line_count = 0;
    let rotation_limit = 10000;

    while let Ok(content) = rx.recv() {
      if !shutdown.is_running() {
        break;
      }

      if let Some(event) = self.parse_strace_line(&content, config.pid_filter) {
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
    }
  }

  fn parse_strace_line(&self, line: &str, pid_filter: Option<u32>) -> Option<formatter::Event> {
    let line = line.trim();

    if line.contains("<...")
      || line.contains("--- SIG")
      || line.contains("+++ exited")
      || line.contains("+++ killed")
    {
      return None;
    }

    let (pid, rest) = if let Some(after) = line.strip_prefix("[pid") {
      match after.split_once(']') {
        Some((pid_str, rest)) => (pid_str.trim().parse::<u32>().ok(), rest.trim()),
        None => (None, line),
      }
    } else {
      match line.find(' ') {
        Some(pos) => (line[..pos].parse::<u32>().ok(), line[pos + 1..].trim()),
        None => return None,
      }
    };

    let pid = match pid.or(pid_filter) {
      Some(p) => p,
      None => return None,
    };

    if let Some(target) = pid_filter {
      if pid != target {
        return None;
      }
    }

    let (timestamp_str, syscall_part) = match rest.find(' ') {
      Some(pos) => (&rest[..pos], rest[pos + 1..].trim()),
      None => return None,
    };

    let syscall_name = syscall_part.split('(').next()?.trim().to_string();
    if syscall_name.is_empty() || syscall_name.contains(' ') {
      return None;
    }

    let timestamp = format!("{} {}", Local::now().format("%Y-%m-%d"), timestamp_str);

    Some(formatter::Event {
      timestamp,
      source: syscall_name,
      pid,
      responsible_pid: pid,
      user: self.get_process_user(pid),
      process_name: self.get_process_name(pid),
    })
  }

  fn get_process_name(&self, pid: u32) -> String {
    std::fs::read_to_string(format!("/proc/{}/comm", pid))
      .map(|s| s.trim().to_string())
      .unwrap_or_else(|_| "unknown".to_string())
  }

  fn get_process_user(&self, pid: u32) -> String {
    let status = std::fs::read_to_string(format!("/proc/{}/status", pid)).unwrap_or_default();

    let uid = status
      .lines()
      .find(|l| l.starts_with("Uid:"))
      .and_then(|l| l.split_whitespace().nth(1))
      .and_then(|u| u.parse::<u32>().ok())
      .unwrap_or(0);

    std::fs::read_to_string("/etc/passwd")
      .unwrap_or_default()
      .lines()
      .find_map(|line| {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() >= 3 && fields[2].parse::<u32>().ok() == Some(uid) {
          Some(fields[0].to_string())
        } else {
          None
        }
      })
      .unwrap_or_else(|| uid.to_string())
  }
}
