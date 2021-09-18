use std::process::Command;
fn main() {
    Command::new("cmd.exe").spawn().unwrap();
}
