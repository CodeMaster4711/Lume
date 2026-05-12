use std::process::Command;
use std::fs;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn take_screenshot(app: tauri::AppHandle) -> Result<String, String> {
    let window = app.get_webview_window("main").ok_or("no main window")?;

    println!("[screenshot] hiding window");
    if let Err(e) = window.hide() {
        println!("[screenshot] hide failed: {e}");
        return Err(format!("hide window failed: {e}"));
    }
    std::thread::sleep(std::time::Duration::from_millis(300));

    let tmp_path = std::env::temp_dir().join("lume_screenshot.png");
    let tmp_str = tmp_path.to_str().ok_or("invalid temp path")?;
    println!("[screenshot] running screencapture to {tmp_str}");

    let result = Command::new("screencapture")
        .args(["-i", "-s", tmp_str])
        .status();

    println!("[screenshot] showing window");
    let _ = window.show();
    let _ = window.set_focus();

    let status = result.map_err(|e| format!("screencapture launch failed: {e}"))?;
    println!("[screenshot] exit code: {:?}", status.code());

    if !tmp_path.exists() {
        return Err("cancelled".into());
    }

    if !status.success() {
        return Err(format!("screencapture failed, exit: {:?}", status.code()));
    }

    let bytes = fs::read(&tmp_path).map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&tmp_path);
    println!("[screenshot] done, {} bytes", bytes.len());
    Ok(base64_encode(&bytes))
}

fn base64_encode(data: &[u8]) -> String {
    use std::fmt::Write;
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i + 2 < data.len() {
        let b0 = data[i] as usize;
        let b1 = data[i + 1] as usize;
        let b2 = data[i + 2] as usize;
        write!(out, "{}{}{}{}", CHARS[b0 >> 2] as char, CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char, CHARS[((b1 & 0xf) << 2) | (b2 >> 6)] as char, CHARS[b2 & 0x3f] as char).unwrap();
        i += 3;
    }
    let rem = data.len() - i;
    if rem == 1 {
        let b0 = data[i] as usize;
        write!(out, "{}{}==", CHARS[b0 >> 2] as char, CHARS[(b0 & 3) << 4] as char).unwrap();
    } else if rem == 2 {
        let b0 = data[i] as usize;
        let b1 = data[i + 1] as usize;
        write!(out, "{}{}{}=", CHARS[b0 >> 2] as char, CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char, CHARS[(b1 & 0xf) << 2] as char).unwrap();
    }
    out
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, take_screenshot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
