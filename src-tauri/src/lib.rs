use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use serde::Serialize;
use tauri::{Manager};
use machine_info::GraphicsUsage;

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
    ram_usage: u64,
    ram_total: u64,
}
#[tauri::command(rename_all = "snake_case")]
fn get_status() -> SystemStatus {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()).with_memory(MemoryRefreshKind::everything())
    );
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();

    let cpu_usage = sys.global_cpu_usage();
    let ram_usage = sys.used_memory();
    let ram_total = sys.total_memory();
    let gpu_usage = GraphicsUsage;
    println!("GPU usage: {}", gpu_usage);

    println!("{}", serialport::available_ports().unwrap().len());
    //send_to_port(cpu_usage, ram_usage, ram_total);
    let result = SystemStatus{cpu_usage, ram_usage, ram_total};
    result.into()
}
//TODO Add System module


pub fn send_to_port(cpu_usage: f32,ram_usage:u64, ram_total: u64) {
    let mut port = serialport::new("COM3", 9600).open().expect("failed to open COM3");
    let cpu_usage = cpu_usage.round();
    let ram_usage = bytes_to_gigabytes(ram_usage).round();
    let ram_total = bytes_to_gigabytes(ram_total).round();
    &port.write(format!("{cpu_usage},{ram_usage},{ram_total};").as_bytes());
       // let s = &port.write("100,10.4,16.2;".as_bytes()).expect("Write failed");
}
fn bytes_to_gigabytes(bytes: u64) -> f32 {
    bytes as f32/ 1024f32.powi(3)
}
