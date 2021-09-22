#![feature(windows_process_extensions_raw_arg)]
use std::io;
use std::io::prelude::*;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

const APP_MAP: &[(&str, &str)] = &[
    // Installer
    ("processhacker-2.39-setup.exe", "/VERYSILENT"),
    ("7z1900-x64.msi", "/q"),
    ("Everything-1.4.1.1005.x64-Setup.exe", "/S"),
    ("Git-2.31.1-64-bit.exe", "/VERYSILENT"), // /VERYSILENT or /SILENT
    ("npp.7.9.5.Installer.exe", "/S"),
    // Portable App
    ("PE-bear_0.5.4_qt4_x86_win_vs10.zip", "PE-bear"),
    ("ripgrep-13.0.0-x86_64-pc-windows-msvc.zip", "ripgrep"),
];
fn main() {
    let cur_dir = std::env::current_dir().unwrap();
    let total = APP_MAP.len();
    for (i, (name, arg)) in APP_MAP.iter().enumerate() {
        let app = Path::new(&cur_dir).join(&name);
        println!("installing {}", &name);
        if name.ends_with(".zip") {
            let status = Command::new("cmd.exe")
                .raw_arg("/c")
                .raw_arg(format!(
                    r#""C:\Program Files\7-Zip\7z.exe" x -oC:\Users\admin\Tools\{} {} -y -spe"#,
                    &name[..name.len() - 4],
                    name
                ))
                .status()
                .unwrap();
            println!("[{}/{}] status: {:?}", i + 1, total, status);
        } else {
            let status = Command::new("cmd.exe")
                .raw_arg("/c")
                .raw_arg(app)
                .raw_arg(arg)
                .status()
                .unwrap();
            println!("[{}/{}] status: {:?}", i + 1, total, status);
        }
    }
    pause();
}
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
