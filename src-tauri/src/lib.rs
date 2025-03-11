use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use serde::Serialize;
use tauri::{Manager};

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

    let result = SystemStatus{cpu_usage, ram_usage, ram_total};
    result.into()
}
//TODO Add System module

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
