use machine_info;
use math::round;
use serde::Serialize;
use serialport::SerialPort;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::sleep;

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
fn get_status(app: AppHandle) {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    tauri::async_runtime::spawn(async move {
        let mut port:Box<dyn SerialPort> = serialport::new("COM3", 9600)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");
        loop{
            sys.refresh_all();
            let cpu_usage = sys.global_cpu_usage();
            let ram_usage = sys.used_memory();
            let ram_total = sys.total_memory();
            let result = SystemStatus {
                cpu_usage,
                ram_usage,
                ram_total,
            };
            send_to_port(cpu_usage, ram_usage,ram_total, &mut port).await;
            app.emit("status", result).ok();
            sleep(Duration::from_millis(1000)).await;
        }
    });
}
//TODO Add System module

pub async fn send_to_port(cpu_usage: f32, ram_usage: u64, ram_total: u64, port:&mut Box<dyn SerialPort>) {
    let cpu_usage = cpu_usage.round();
    let ram_usage = round::ceil(bytes_to_gigabytes(ram_usage), 1);
    let ram_total = round::ceil(bytes_to_gigabytes(ram_total), 1);
    let s = &port.write(format!("{cpu_usage},{ram_usage},{ram_total};").as_bytes()).expect("Write failed");
}
fn bytes_to_gigabytes(bytes: u64) -> f64 {
    bytes as f64 / 1024f32.powi(3) as f64
}
