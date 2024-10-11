use crate::utils::errors::Error;
use chrono::{DateTime, Local};
use crab_protocol::types::namespaced_key::NamespacedKey;
use lazy_static::lazy_static;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::{env, io};

lazy_static! {
    pub static ref FILES: ServerFiles = ServerFiles::new();
}

pub struct ServerFiles {
    pub server_root: PathBuf,
    pub logs: PathBuf,
    pub worlds: PathBuf,
}

impl ServerFiles {
    pub fn new() -> Self {
        let server_root = if let Ok(manifest_str) = env::var("CARGO_MANIFEST_DIR") {
            Path::new(&manifest_str).join("server").to_path_buf()
        } else {
            let mut exe_path = env::current_exe()
                .expect("Couldn't get server directory path! (Maybe permissions issue?)")
                .to_path_buf();
            exe_path.pop();
            exe_path
        };

        ServerFiles {
            logs: server_root.join("logs"),
            worlds: server_root.join("worlds"),
            server_root,
        }
    }

    pub fn create_directories(&self) -> Result<(), Error> {
        std::fs::create_dir_all(&self.logs)?;
        std::fs::create_dir_all(&self.worlds)?;
        Ok(())
    }

    pub fn rotate_log_files(&self) -> Result<(), Error> {
        let last_log_path = self.latest_log();

        if !last_log_path.try_exists()? {
            return Ok(());
        }

        let time = fs::metadata(&last_log_path)?.modified()?;
        let modification_datetime: DateTime<Local> = time.into();
        let formatted_time = modification_datetime.format("%Y-%m-%d").to_string();

        let file_count = count_files_with_prefix(self.logs.as_path(), &formatted_time)?;

        let log_path_str = format!("{formatted_time}-{}.log", file_count + 1);
        let new_log_path = self.logs.join(log_path_str);
        fs::rename(last_log_path, new_log_path)?;

        Ok(())
    }

    pub fn latest_log(&self) -> PathBuf {
        self.logs.join("latest.log")
    }

    pub fn world_region_path(&self, namespaced_key: &NamespacedKey) -> Result<PathBuf, Error> {
        let path = self
            .server_root
            .join("worlds")
            .join(&namespaced_key.namespace)
            .join(&namespaced_key.value)
            .join("region");
        std::fs::create_dir_all(&path)?;
        Ok(path)
    }
}

pub fn count_files_with_prefix(dir: &Path, prefix: &str) -> io::Result<usize> {
    let mut count = 0;
    if !dir.is_dir() {
        return Ok(count);
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        count += (path.is_file() && path.file_name().unwrap().to_str().unwrap().starts_with(prefix)) as usize;
    }
    Ok(count)
}
