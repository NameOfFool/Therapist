mod menu_plugin;
mod port;
mod tray;

use crate::port::send_to_port;
use serde::Serialize;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Emitter, RunEvent, Runtime};
use tokio::time::sleep;

#[cfg(all(desktop, not(test)))]
pub struct PopupMenu<R: Runtime>(tauri::menu::Menu<R>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let handle = app.handle();
            tray::create_tray(handle)?;
            handle.plugin(menu_plugin::init())?;
            get_status(handle.clone(), String::from("COM3"));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_status])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |_app_handle, _event| {
            #[cfg(all(desktop, not(test)))]
            match &_event {
                RunEvent::ExitRequested { api, code, .. } => {
                    if code.is_none() {
                        api.prevent_exit();
                    }
                }
                _ => (),
            }
        });
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SystemStatus {
    cpu_usage: f32,
    ram_usage: u64,
    ram_total: u64,
}
#[tauri::command(rename_all = "snake_case")]
fn get_available_ports() {
    let ports = port::get_ports().unwrap();
}
#[tauri::command(rename_all = "snake_case")]
fn get_status(app: AppHandle, port_name: String) {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    tauri::async_runtime::spawn(async move {
        let mut port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(10))
            .open();
        loop {
            sys.refresh_all();
            let cpu_usage = sys.global_cpu_usage();
            let ram_usage = sys.used_memory();
            let ram_total = sys.total_memory();
            let result = SystemStatus {
                cpu_usage,
                ram_usage,
                ram_total,
            };
            if let Ok(ref mut p) = port {
                send_to_port(cpu_usage, ram_usage, ram_total, p).await;
            }
            app.emit("status", result).ok();
            sleep(Duration::from_millis(1000)).await;
        }
    });
}
