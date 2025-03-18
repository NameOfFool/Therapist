use std::io::Write;
use math::round;
use serialport::{Error, SerialPort};

pub async fn send_to_port(cpu_usage: f32, ram_usage: u64, ram_total: u64, port:&mut Box<dyn SerialPort>) {
    let cpu_usage = cpu_usage.round();
    let ram_usage = round::ceil(bytes_to_gigabytes(ram_usage), 1);
    let ram_total = round::ceil(bytes_to_gigabytes(ram_total), 1);
    let _ = &port.write(format!("{cpu_usage},{ram_usage},{ram_total};").as_bytes()).expect("Write failed");
}
pub fn get_ports() -> Result<Vec<String>, Error> {
    let ports = serialport::available_ports()?.iter().map(|x|{
        x.port_name.clone()
    }).collect();
    Ok(ports)
}

fn bytes_to_gigabytes(bytes: u64) -> f64 {
    bytes as f64 / 1024f32.powi(3) as f64
}