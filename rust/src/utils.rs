use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub fn fill_clipboard(content: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content).unwrap();
}

pub fn expand_path(relative_path: &str) -> String {
    let cur_dir = std::env::current_dir().unwrap();
    let cur_path = cur_dir.join(&relative_path);
    let cur_path_str = cur_path.to_string_lossy().to_string();
    cur_path_str
}
