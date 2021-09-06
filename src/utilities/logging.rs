
use std::{fs, fmt, io::Write};
use chrono;

#[derive(Debug)]
pub enum Level {
    Debug,
    Warning, 
    Error,
    Trace,
    Info
}

impl fmt::Display for Level {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}


pub struct Logger {
    file: Option<fs::File>  
}

impl Logger {
    pub fn new() -> Logger {
        Logger {file: None}
    }
    pub fn configure_logger(&mut self, path: &str) {
        self.file = Some(fs::OpenOptions::new().read(true).write(true).truncate(true).create(true).open(path).expect("Error creating or opening log file"));
    }
    pub fn write_log(&mut self, text: String, level: Level) {
        if let None = self.file {
            println!("No log file!");
            return;
        }
        let mut file = self.file.as_ref().unwrap();
        let datetime = chrono::Utc::now();
        let level_string = level.to_string().to_uppercase();
        let message = format!("{}\t[{}]\t{}\n", datetime, level_string, text);
        file.write_all(message.as_bytes()).expect("Error writing log entry");
    }
}