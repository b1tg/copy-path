use std::env;
mod utils;
use utils::*;
fn main() {
    let cur_file = env::args().nth(1).unwrap();
    let cur_path_str = expand_path(&cur_file);
    let cur_path_str_escape = cur_path_str.replace("\\", "\\\\");
    fill_clipboard(cur_path_str_escape);
}
