//! Module responsible for writing the log messages.
use chrono;
use std::{fmt, io::Read, io::Write};
use tokio::{fs, io::{AsyncWriteExt, AsyncReadExt}};

/// Logging level indicates the importance of log message. 
#[derive(Debug)]
pub enum Level {
    Debug,
    Warning,
    Error,
    Trace,
    Info,
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

/// The logger is struct providing an interface to write log messages. 
/// 
/// It is intended that logger is used as a singleton It is intended that struct is used as singleton in [`global::shared`](crate::global::shared)
/// It is possible to use two or more instances of logger provided that at least one is implemented as shared static in [`global::shared`](crate::global::shared)
/// For safety reasons multiple instance of logger should use different logfiles.
pub struct Logger {
    path: String,
    file: Option<fs::File>,
    bytes_written: usize,
    max_bytes: usize
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            path: String::new(),
            file: None,
            bytes_written: 0,
            max_bytes: 1e7 as usize + 1
        }
    }
    pub async fn configure_logger(&mut self, path: &str) {
        self.path = String::from(path);
        self.file = Some(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(true)
                .create(true)
                .open(path)
                .await
                .expect("Error creating or opening log file"),
        );
    }
    pub async fn write_log(&mut self, text: String, level: Level) {
        if let None = self.file {
            println!("No log file!");
            return;
        }
        let file = self.file.as_mut().unwrap();
        let datetime = chrono::Utc::now();
        let level_string = level.to_string().to_uppercase();
        let message = format!("{}\t[{}]\t{}\n", datetime, level_string, text);
        let bytes_len = message.len();
        file.write_all(message.as_bytes()).await
            .expect("Error writing log entry");
        file.sync_all().await.expect("Error syncronising logfile metadata"); 
        self.bytes_written += bytes_len;
        self.rotate_on_overflow().await;   
    }
    
    pub fn get_logfile_path(&mut self) -> Option<String>
    {
        match self.file {
            None => {
                None
            }    
            Some(_) => {
                Some(self.path.clone())
            }
        }
    }
    
    pub async fn extract_entries(&mut self, amount: usize) -> String {
        if let None = self.file {
            return String::from("No log file!");
        }
        let mut contents = String::new();
        self.file = Some(
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .truncate(false)
                .create(true)
                .open(self.path.as_str())
                .await
                .expect("Error creating or opening log file"),
        );
        self.file
            .as_mut()
            .unwrap()
            .read_to_string(&mut contents)
            .await
            .unwrap();
        
        self.file.as_mut().unwrap().sync_all().await.expect("Error syncronising logfile metadata");    
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
    
    /// Updates logfile if `self.bytes_written >= max_bytes`.
    /// Rotation is perfomed by renaming current logfile from
    /// `self.path` to `self.path.<UNIX_TIME>
    /// and creating new logfile instead of rotated one.
    /// As for current version no compression is performed for rotated files
    /// and should be handled separately. 
    async fn rotate_on_overflow(&mut self)
    {
        if self.bytes_written < self.max_bytes {
            return
        }
        let timestamp = chrono::Utc::now().timestamp();
        let new_path = format!("{}.{}", self.path.clone(), timestamp);
        let result = fs::rename(self.path.clone(), new_path).await;
        match result {
            Ok(()) => {
                self.file = Some(
                    fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .truncate(true)
                        .create(true)
                        .open(self.path.clone())
                        .await
                        .expect("Error creating or opening log file"),
                );
            }  
            Err(err) => {
                println!("{}", err);
                return
            }
        };
    }
}
