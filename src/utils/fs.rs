use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::{env, io};

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

pub fn get_server_path() -> PathBuf {
    if let Ok(manifest_str) = env::var("CARGO_MANIFEST_DIR") {
        Path::new(&manifest_str).join("server").to_path_buf()
    } else {
        let mut exe_path = env::current_exe()
            .expect("Couldn't get server directory path! (Maybe permissions issue?)")
            .to_path_buf();
        exe_path.pop();
        exe_path
    }
}
