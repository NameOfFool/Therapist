use std::{thread, time};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemStatus {
    cpu_usage: f32,
    ram_usage: f32,
    ram_total: f32,
}
#[tauri::command(rename_all = "snake_case")]
fn get_status() -> SystemStatus {
    let cpu_usage = 52.2;
    let ram_usage = 4.2;
    let ram_total = 8.0;
    println!("status event called");
    let result = SystemStatus{cpu_usage, ram_usage, ram_total};
    result.into()
}
/*
pub fn get_port(){
    let mut port = serialport::new("COM3", 9600).timeout(Duration::from_millis(1)).open().expect("Failed to open port");
    loop {
        let s = &port.write("100,10.4,16.2;".as_bytes()).expect("Write failed");
        sleep(Duration::from_millis(1000));
        println!("{s}");
    }
}
*/
