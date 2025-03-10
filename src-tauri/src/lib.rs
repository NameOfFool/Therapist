use std::thread::sleep;
use std::time::Duration;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    let mut port = serialport::new("COM3", 9600).timeout(Duration::from_millis(1)).open().expect("Failed to open port");
    loop {
        let s = &port.write("100,10.4,16.2;".as_bytes()).expect("Write failed");
        sleep(Duration::from_millis(1000));
        println!("{s}");
    }
}