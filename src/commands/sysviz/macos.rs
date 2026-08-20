use crate::commands::sysviz::common::alert;
use crate::commands::sysviz::common::config::{self as cfg, SystemMonitor};
use crate::commands::sysviz::common::constants;
use crate::commands::sysviz::common::formatter;
use crate::commands::sysviz::common::stats;
use crate::global::export;
use crate::global::permissions;
use crate::global::shutdown as shutdown_mod;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::sync_channel;

pub struct MacOsMonitor;

impl SystemMonitor for MacOsMonitor {
  fn run(&mut self, config: &cfg::SysVizConfig) {
    let shutdown = shutdown_mod::Shutdown::new(config.timeout);

    permissions::ensure_root();
    println!("{}", self.start_message());

    let fmt = config.get_formatter();
    fmt.header();

    let args = self.build_args(config);
    let mut child = self.spawn_eslogger(&args);

    let mut stats_collector = stats::Stats::new();
    let alert_manager = alert::Alert::new(config.alert.clone());

    if let Some(stdout) = child.stdout.take() {
      self.process_events(
        stdout,
        &shutdown,
        config,
        &*fmt,
        &mut stats_collector,
        &alert_manager,
      );

      let _ = child.kill();
      let _ = child.wait();

      fmt.footer();

      if config.stats {
        fmt.stats(stats_collector.get_counts(), config.top);
      }

      if let Err(e) = export::Export::finalize(&config.export_path, config.zip) {
        eprintln!("{} {}", constants::ERROR_EXPORT_FAILED, e);
      }
    }
    let _ = child.kill();
    let _ = child.wait();
  }

  fn default_syscalls(&self) -> Vec<&'static str> {
    constants::platform::DEFAULT_SYSCALLS.to_vec()
  }

  fn start_message(&self) -> &'static str {
    constants::platform::MSG_START_MACOS
  }
}

impl MacOsMonitor {
  pub fn new() -> Self {
    Self
  }

  fn build_args(&self, config: &cfg::SysVizConfig) -> Vec<String> {
    if let Some(ref syscall) = config.name_filter {
      vec![syscall.clone()]
    } else {
      self
        .default_syscalls()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }
  }

  fn spawn_eslogger(&self, args: &[String]) -> std::process::Child {
    Command::new("eslogger")
      .args(args)
      .stdout(Stdio::piped())
      .spawn()
      .expect(constants::platform::ERROR_MACOS_CMD)
  }

  fn process_events(
    &self,
    stdout: std::process::ChildStdout,
    shutdown: &shutdown_mod::Shutdown,
    config: &cfg::SysVizConfig,
    fmt: &dyn formatter::Formatter,
    stats_collector: &mut stats::Stats,
    alert_manager: &alert::Alert,
  ) {
    let (tx, rx) = sync_channel(1000);

    std::thread::spawn(move || {
      let stdout_reader = BufReader::new(stdout);
      for line in stdout_reader.lines() {
        match line {
          Ok(content) => {
            if tx.send(content).is_err() {
              break;
            }
          }
          Err(e) => {
            eprintln!("{}: {}", constants::platform::ERROR_MACOS_READ_STREAM, e);
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

      if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content)
        && let Some(event) = self.parse_es_event(json, config.pid_filter)
      {
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

  fn parse_es_event(
    &self,
    json: serde_json::Value,
    pid_filter: Option<u32>,
  ) -> Option<formatter::Event> {
    let timestamp = json["time"].as_str()?.to_string();
    let event = json["event"].as_object()?;
    let pid = json["process"]["audit_token"]["pid"].as_u64().unwrap_or(0) as u32;

    if let Some(target_pid) = pid_filter
      && pid != target_pid
    {
      return None;
    }

    let source = event.keys().next()?.to_string();
    let responsible_pid = json["process"]["responsible_audit_token"]["pid"]
      .as_u64()
      .unwrap_or(0) as u32;
    let user = json["process"]["signing_id"]
      .as_str()
      .unwrap_or("unsigned")
      .to_string();
    let process_name = json["process"]["executable"]["path"]
      .as_str()
      .unwrap_or("unknown")
      .to_string();

    Some(formatter::Event {
      timestamp,
      source,
      pid,
      responsible_pid,
      user,
      process_name,
    })
  }
}
