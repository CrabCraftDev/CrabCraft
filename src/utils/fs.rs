use std::fs::read_dir;
use std::io;
use std::path::Path;

pub fn count_files_with_prefix(dir: &Path, prefix: &str) -> io::Result<usize> {
    let mut count = 0;
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.file_name().unwrap().to_str().unwrap().starts_with(prefix) {
                count += 1;
            }
        }
    }
    Ok(count)
}