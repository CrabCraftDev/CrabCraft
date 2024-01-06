use std::fs::create_dir_all;

pub fn create_subdirectories(path: &str) {
    if let Some(parent) = std::path::Path::new(&path).parent() {
        create_dir_all(parent).expect("Failed to create directories");
    }
}