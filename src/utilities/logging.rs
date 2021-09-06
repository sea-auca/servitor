
use std::{fs, fmt, io::Write, io::Read};
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
    path: String,
    file: Option<fs::File>  
}

impl Logger {
    pub fn new() -> Logger {
        Logger {path: String::new(), file: None}
    }
    pub fn configure_logger(&mut self, path: &str) {
        self.path = String::from(path);
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
    pub fn extract_entries(&mut self, amount: usize) -> String {
        if let None = self.file {
            return String::from("No log file!");
        }
        let mut contents = String::new();  
        self.file = Some(fs::OpenOptions::new().read(true).write(true).truncate(false).create(true).open(self.path.as_str()).expect("Error creating or opening log file"));      
        self.file.as_ref().unwrap().read_to_string(&mut contents).unwrap();
        let mut split = contents.split("\n");
        let mut result = String::new();
        for _i in 0..amount {
            let line = split.next();
            if let None = line {
                break;
            }
            let line_string = line.unwrap();
            result = format!("{}\n{}", result, line_string)
        }
        result
    }
}