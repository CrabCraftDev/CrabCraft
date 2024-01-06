use std::fs::OpenOptions;
use std::io::Write;

use chrono::Local;
use log::{Level, Metadata, Record};

use crate::utils::fs::create_subdirectories;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%H:%M:%S");
            let message = format!("[{}] [{}] {}\n", timestamp, record.level(), record.args());

            print!("{}", message);

            create_subdirectories("logs/latest.txt");
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("logs/latest.txt");

            file.unwrap()
                .write_all(message.as_bytes())
                .expect("Couldn't write to file!");
        }
    }

    fn flush(&self) {}
}
