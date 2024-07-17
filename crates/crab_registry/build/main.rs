use std::path::Path;
use std::process::Command;
use std::{env, fs};

mod blocks;

const BLOCKS_OUTPUT_FILE: &str = "blocks.rs";

fn main() {
    let output_dir = env::var_os("OUT_DIR").unwrap();
    let blocks_file = Path::new(&output_dir).join(BLOCKS_OUTPUT_FILE);
    fs::write(blocks_file.clone(), blocks::generate().to_string()).unwrap();

    Command::new("rustfmt")
        .arg(blocks_file)
        .output()
        .expect("Failed to run rustfmt!");
}
