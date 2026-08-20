use crate::global::constants::{APP_NAME, APP_VERSION};
use sysinfo::System;

pub fn info() {
  let os = System::name().unwrap_or_else(|| "Unknown".to_string());
  let version = System::os_version().unwrap_or_else(|| "?".to_string());
  let hostname = System::host_name().unwrap_or_else(|| "?".to_string());
  let arch = match std::env::consts::ARCH {
    "aarch64" => "ARM64",
    "x86_64" => "x64",
    _ => std::env::consts::ARCH,
  };

  println!("{APP_NAME} v{APP_VERSION} 📍 {os} {version} ({arch}) @ {hostname}\n");
}
