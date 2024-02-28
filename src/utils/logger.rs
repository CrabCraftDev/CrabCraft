use std::fs::{create_dir_all, metadata, rename, OpenOptions};
use std::io::{Result, Write};

use chrono::{DateTime, Local};
use log::{Level, Log, Metadata, Record};

use crate::utils::fs::{count_files_with_prefix, get_server_path};

pub struct Logger;

impl Logger {
    pub fn rename_log_file() -> Result<()> {
        let logs_path = get_server_path().join("logs");
        let log_path = logs_path.join("latest.log");

        if !log_path.exists() {
            return Ok(());
        }

        let time = metadata(log_path)?.modified()?;
        let modification_datetime: DateTime<Local> = time.into();
        let formatted_time = modification_datetime.format("%Y-%m-%d").to_string();

        let file_count = count_files_with_prefix(logs_path.as_path(), &formatted_time)?;

        let log_path_str = format!("{}-{}.log", formatted_time, file_count + 1);
        let new_log_path = logs_path.join(log_path_str);
        rename(logs_path.join("latest.log"), new_log_path)?;

        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%H:%M:%S");
            let message = format!("[{}] [{}] {}\n", timestamp, record.level(), record.args());

            print!("{}", message);

            create_dir_all(get_server_path().join("logs")).expect("Failed to create directories");

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(get_server_path().join("logs/latest.log"));

            file.unwrap()
                .write_all(message.as_bytes())
                .expect("Couldn't write to file!");
        }
    }

    fn flush(&self) {}
}
