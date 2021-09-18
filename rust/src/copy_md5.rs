#![windows_subsystem = "windows"]
use failure::Error;
use std::env;
use std::fs;
use std::io::Read;
mod utils;
use utils::*;
fn main() {
    let cur_file = env::args().nth(1).unwrap();
    let cur_path_str = expand_path(&cur_file);
    let hash = md5(&cur_path_str).unwrap();
    fill_clipboard(hash);
}

fn md5(target: &str) -> Result<String, Error> {
    let mut file = fs::File::open(target)?;
    let mut buf = vec![];
    let _ = file.read_to_end(&mut buf).unwrap();
    let digest = format!("{:x}", md5::compute(&buf));
    Ok(digest)
}
