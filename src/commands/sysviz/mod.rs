pub mod formatter;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod win;

use crate::constants;
use crate::utils;

pub fn run(csv: bool) {
    println!("{}", constants::BANNER_SYSVIZ);

    utils::info();

    #[cfg(target_os = "linux")]
    linux::run();
    
    #[cfg(target_os = "macos")]
    macos::run(csv);
    
    #[cfg(target_os = "windows")]
    win::run();
}