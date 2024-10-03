use std::fs::{metadata, rename, OpenOptions};
use std::io::Write;

use crate::utils::errors::Error;
use chrono::{DateTime, Local};
use colored::Colorize;
use log::{Level, Log, Metadata, Record};

use crate::utils::paths::{count_files_with_prefix, PATHS};

pub struct Logger;

impl Logger {
    pub fn rename_log_file() -> Result<(), Error> {
        let logs_dir = &PATHS.logs;
        let last_log_path = PATHS.latest_log();

        if !last_log_path.exists() {
            return Ok(());
        }

        let time = metadata(&last_log_path)?.modified()?;
        let modification_datetime: DateTime<Local> = time.into();
        let formatted_time = modification_datetime.format("%Y-%m-%d").to_string();

        let file_count = count_files_with_prefix(logs_dir.as_path(), &formatted_time)?;

        let log_path_str = format!("{formatted_time}-{}.log", file_count + 1);
        let new_log_path = logs_dir.join(log_path_str);
        rename(last_log_path, new_log_path)?;

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
            let uncolored_message = format!("[{timestamp}] [{}] {}", record.level(), record.args());

            let message = match record.level() {
                Level::Debug => uncolored_message.blue(),
                Level::Warn => uncolored_message.yellow(),
                Level::Error => uncolored_message.red(),
                _ => uncolored_message.normal(),
            };

            println!("{message}");

            if log_to_file(uncolored_message).is_err() {
                println!("{}", "Failed to save log to file!".red())
            }
        }
    }

    fn flush(&self) {}
}

fn log_to_file(message: String) -> Result<(), Error> {
    let file = OpenOptions::new().create(true).append(true).open(PATHS.latest_log());

    file?.write_all(format!("{message}\n").as_bytes())?;
    Ok(())
}
