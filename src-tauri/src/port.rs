use math::round;
use serde_json::json;
use serialport::{Error as SerialPortError, SerialPort};
use std::io::Write;
use tauri::App;
use tauri_plugin_store::{StoreExt, Error as StoreError};

pub async fn send_to_port(
    cpu_usage: f32,
    ram_usage: u64,
    ram_total: u64,
    port: &mut Box<dyn SerialPort>,
) {
    let cpu_usage = cpu_usage.round();
    let ram_usage = round::ceil(bytes_to_gigabytes(ram_usage), 1);
    let ram_total = round::ceil(bytes_to_gigabytes(ram_total), 1);
    let _ = &port
        .write(format!("{cpu_usage},{ram_usage},{ram_total};").as_bytes())
        .expect("Write failed");
}
pub fn get_ports() -> Result<Vec<String>, SerialPortError> {
    let ports = serialport::available_ports()?
        .iter()
        .map(|x| x.port_name.clone())
        .collect();
    Ok(ports)
}
pub fn save_port(app: &mut App, port: &str) -> Result<(), StoreError> {
    let store = app.store("store.json")?;
        store.set("port".to_string(), json!(port));
        Ok(())
}
pub fn load_port(app: &mut App) -> Result<String, StoreError> {
    let store = app.store("store.json")?;
    Ok(store.get("port").unwrap().to_string())
}

fn bytes_to_gigabytes(bytes: u64) -> f64 {
    bytes as f64 / 1024f32.powi(3) as f64
}
