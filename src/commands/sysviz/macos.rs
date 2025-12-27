use crate::utils;
use crate::constants;
use crate::commands::sysviz::formatter;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn run(csv: bool) {
    utils::ensure_root();
    
    println!("{}", constants::MSG_START);
    
    let formatter = formatter::get_formatter(csv);

    formatter.header();
    
    let mut child = Command::new("fs_usage")
        .args(["-w", "-f", "network"])
        .stdout(Stdio::piped())
        .spawn()
        .expect(constants::ERROR_FS_USAGE);
    
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        
        for line in reader.lines().flatten() {
            if line.starts_with("TIMESTAMP") || line.trim().is_empty() {
                continue; 
            }
            
            if let Some(event) = parse_fs_usage_line(&line) {
                formatter.format(&event);
            }
        }
    }
    
    let _ = child.wait();
}

fn parse_fs_usage_line(line: &str) -> Option<formatter::FsUsageEvent> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 { return None; }

    let timestamp = parts[0].to_string();
    let operation = parts[1].to_string();
    let process_info = parts.last()?;
    let (process_name, pid_str) = process_info.rsplit_once('.')?;
    let pid = pid_str.parse::<u32>().unwrap_or(0);
    let path = parts[3..parts.len() - 1].join(" ");
    
    Some(formatter::FsUsageEvent {
        timestamp,
        process_name: process_name.to_string(),
        pid,
        operation,
        path,
        result: "OK".to_string(),
    })
}