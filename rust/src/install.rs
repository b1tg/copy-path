#![feature(windows_process_extensions_raw_arg)]
use std::os::windows::process::CommandExt;
use std::process;
fn main() {
    let cur_dir = std::env::current_dir().unwrap();
    // let release_dir = cur_dir.join("target\\release");
    let release_dir = cur_dir;
    let copy_path_raw = release_dir.join("copy_path_raw.exe");
    let copy_path_escape = release_dir.join("copy_path_escape.exe");
    let copy_md5 = release_dir.join("copy_md5.exe");
    install("copy_path_raw", &copy_path_raw.to_string_lossy());
    install("copy_path_escape", &copy_path_escape.to_string_lossy());
    install("copy_md5", &copy_md5.to_string_lossy());
}

fn install(menu: &str, exe: &str) {
    let status = process::Command::new("cmd.exe")
        .raw_arg("/c")
        .raw_arg(format!(
            r#"reg add HKEY_CLASSES_ROOT\*\shell\{}\command /ve /t REG_SZ /d "{} %1" /f"#,
            menu, exe
        ))
        .status()
        .unwrap();
    println!("status: {:?}", status);
}
