pub mod common;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use crate::args;
use crate::commands::sysviz::common::config::{self as cfg, SystemMonitor};
use crate::commands::sysviz::common::constants::BANNER_SYSVIZ;
use crate::global::system;

pub fn run(args: args::SysVizArgs) {
  println!("{BANNER_SYSVIZ}");
  system::info();

  let config = cfg::SysVizConfig::new(args);

  #[cfg(target_os = "linux")]
  {
    let mut monitor = linux::LinuxMonitor::new();
    monitor.run(&config);
  }

  #[cfg(target_os = "macos")]
  {
    let mut monitor = macos::MacOsMonitor::new();
    monitor.run(&config);
  }

  #[cfg(target_os = "windows")]
  {
    let mut monitor = windows::WindowsMonitor::new();
    monitor.run(&config);
  }

  #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
  compile_error!("[error]   platform not supported");
}
