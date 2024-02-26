use std::fs::read_dir;
use std::io;
use std::path::Path;

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
