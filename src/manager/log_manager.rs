use crate::utility;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;

const FILENAME: &'static str = "output.txt";

pub struct LogManager {
    file: File,
}

impl LogManager {
    pub fn write_log(&self, kind: LogKind, message: String) {
        write!(
            &self.file,
            "{0} - {1} - {2}",
            utility::current_time_string(),
            kind,
            message,
        )
        .expect("Error handling the log file");

        writeln!(&self.file).expect("Error handling the log file");
    }
}

impl LogManager {
    pub fn new() -> Self {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(FILENAME)
            .expect("Unable to open or create file");

        LogManager { file }
    }
}

pub enum LogKind {
    INFO,
    WARNING,
    ERROR,
}

impl Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind_str = match self {
            LogKind::INFO => "INFO",
            LogKind::WARNING => "WARNING",
            LogKind::ERROR => "ERROR",
        };
        write!(f, "{}", kind_str)
    }
}
