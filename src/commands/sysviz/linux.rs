pub fn run() {
    println!("Hello Linux! 🐧");
    
    // Informations spécifiques à Linux
    println!("\n--- Informations spécifiques Linux ---");
    if let Ok(uptime) = std::fs::read_to_string("/proc/uptime") {
        if let Some(uptime_sec) = uptime.split_whitespace().next() {
            if let Ok(secs) = uptime_sec.parse::<f64>() {
                let hours = (secs / 3600.0) as u64;
                let mins = ((secs % 3600.0) / 60.0) as u64;
                println!("Uptime : {}h {}m", hours, mins);
            }
        }
    }
    
    // Distribution Linux
    if let Ok(os_release) = std::fs::read_to_string("/etc/os-release") {
        for line in os_release.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let distro = line.strip_prefix("PRETTY_NAME=")
                    .unwrap_or("")
                    .trim_matches('"');
                println!("Distribution : {}", distro);
                break;
            }
        }
    }
}