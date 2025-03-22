mod menu_plugin;
mod port;
mod tray;

use std::sync::Arc;
use crate::port::send_to_port;
use serde::Serialize;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Emitter, RunEvent, Runtime, State};
use tokio::time::sleep;

use tauri::{Builder, Manager};
use tokio::sync::Mutex;

#[cfg(all(desktop, not(test)))]
pub struct PopupMenu<R: Runtime>(tauri::menu::Menu<R>);

struct AppState{
    system_status: SystemStatus
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {

            app.manage(Arc::new(Mutex::new(AppState{system_status: SystemStatus{
                cpu_usage: 0.0,
                ram_usage: 0,
                ram_total: 0,
            }
            })));

            let handle = app.handle();
            tray::create_tray(handle)?;
            handle.plugin(menu_plugin::init())?;
            get_status(handle.clone());

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_status, get_available_ports, open_port])
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
fn get_available_ports() ->Vec<String> {
    port::get_ports().unwrap()
}
#[tauri::command(rename_all = "snake_case")]
fn open_port<'a>(state: State<'_,Arc<Mutex<AppState>>>,port_name: String) -> Result<String, String> {
    let arc_state= Arc::clone(&state);
    tauri::async_runtime::spawn(async move{
        let mut port = serialport::new(port_name, 9600)
            .timeout(Duration::from_millis(10))
            .open();
        loop {

            let mut arc_state = arc_state.lock().await;
            if let Ok(ref mut p) = port {
                send_to_port(
                    arc_state.system_status.cpu_usage,
                    arc_state.system_status.ram_usage,
                    arc_state.system_status.ram_total,
                    p).await;
            }
        }
    });
    Ok("Succeeded".to_string())
}
#[tauri::command(rename_all = "snake_case")]
fn get_status(app: AppHandle) {
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    let arc_state= app.state::<Arc<Mutex<AppState>>>();
    let arc_state = Arc::clone(&arc_state);
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    tauri::async_runtime::spawn(async move {
        loop {
            let mut arc_state = arc_state.lock().await;
            sys.refresh_all();
            arc_state.system_status.cpu_usage = sys.global_cpu_usage();
            arc_state.system_status.ram_usage = sys.used_memory();
            arc_state.system_status.ram_total = sys.total_memory();
            let result = arc_state.system_status.clone();

            app.emit("status", result).ok();
            sleep(Duration::from_millis(1000)).await;
        }
    });
}
