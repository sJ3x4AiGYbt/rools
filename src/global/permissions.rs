#[cfg(windows)]
use crate::global::constants::ERROR_NEED_ADMIN_WINDOWS;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::global::constants::ERROR_NEED_ROOT;
use std::env;
use std::process;

pub fn check_sudo() -> bool {
  #[cfg(unix)]
  {
    unsafe { libc::geteuid() == 0 }
  }

  #[cfg(windows)]
  {
    is_elevated_windows()
  }
}

#[cfg(windows)]
fn is_elevated_windows() -> bool {
  use std::process::Command;

  Command::new("net")
    .args(["session"])
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
}

pub fn ensure_root() {
  if !check_sudo() {
    let exe_name = env::current_exe()
      .ok()
      .and_then(|p| p.file_name()?.to_str()?.to_string().into())
      .unwrap_or_else(|| "sysviz".to_string());

    let sub_command = env::args().nth(1).unwrap_or_default();

    #[cfg(windows)]
    eprintln!(
      "{} {} {}\n",
      ERROR_NEED_ADMIN_WINDOWS, exe_name, sub_command
    );
    #[cfg(not(windows))]
    eprintln!("{ERROR_NEED_ROOT} {exe_name} {sub_command}\n");

    process::exit(1);
  }
}
