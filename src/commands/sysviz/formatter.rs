#[derive(Debug)]
pub struct FsUsageEvent {
    pub timestamp: String,
    pub process_name: String,
    pub pid: u32,
    pub operation: String,
    pub path: String,
    pub result: String,
}

pub trait Formatter {
    fn header(&self);
    fn format(&self, event: &FsUsageEvent);
}

struct Txt;
struct Csv;

impl Formatter for Txt {
    fn header(&self) {
        println!("{:-<120}", "");
        println!(
            "{:<20} {:<10} {:<15} {:<25} {:<30} {}",
            "TIMESTAMP", "PID", "OP", "PROCESS", "PATH", "RESULT"
        );
        println!("{:-<120}\n", "");
    }

    fn format(&self, e: &FsUsageEvent) {
        println!(
            "{:<20} {:<10} {:<15} {:<25} {:<30} {}",
            e.timestamp,
            e.pid,
            e.operation,
            e.process_name,
            e.path,
            e.result
        );
    }
}

impl Formatter for Csv {
    fn header(&self) {
        println!("timestamp,pid,process,operation,path,result");
    }

    fn format(&self, e: &FsUsageEvent) {
        println!(
            "\"{}\",{},\"{}\",\"{}\",\"{}\",\"{}\"",
            e.timestamp,
            e.pid,
            e.process_name,
            e.operation,
            e.path.replace('"', "\"\""),
            e.result
        );
    }
}


pub fn get_formatter(is_csv: bool) -> Box<dyn Formatter> {
    if is_csv {
        Box::new(Csv)
    } else {
        Box::new(Txt)
    }
}