use crate::utils::errors::Error;
use lazy_static::lazy_static;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::{env, io};

lazy_static! {
    pub static ref PATHS: ServerPaths = ServerPaths::new();
}

pub struct ServerPaths {
    pub server_root: PathBuf,
    pub logs: PathBuf,
}

impl ServerPaths {
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

        ServerPaths {
            logs: server_root.join("logs"),
            server_root,
        }
    }

    pub fn create_directories(&self) -> Result<(), Error> {
        std::fs::create_dir_all(&self.logs)?;
        Ok(())
    }

    pub fn latest_log(&self) -> PathBuf {
        self.logs.join("latest.log")
    }
}

pub fn count_files_with_prefix(dir: &Path, prefix: &str) -> io::Result<usize> {
    let mut count = 0;
    if !dir.is_dir() {
        return Ok(count);
    }

    for entry in read_dir(dir)? {
        let path = entry?.path();
        count += (path.is_file() && path.file_name().unwrap().to_str().unwrap().starts_with(prefix)) as usize;
    }
    Ok(count)
}
