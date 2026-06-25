// ESP32 Modbus Client (requires esp-idf-svc + WiFi setup)
// This is a proof-of-concept - requires esp-idf environment
//
// To use: Install esp-idf-sys, configure Wifi credentials, compile with:
// cargo +esp build --target xtensa-esp32-espidf --release
//
// This implementation is much smaller (~200-400KB) and uses:
// - No async runtime (blocking for simplicity)
// - esp-idf-svc for WiFi/TCP
// - Minimal dependencies

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() {
    println!("🚀 ESP32 Modbus Client");

    // WiFi setup would go here (requires esp-idf configuration)
    // let wifi = connect_to_wifi("SSID", "PASSWORD").unwrap();

    // For now, mock the connection
    match TcpStream::connect_timeout(
        &"192.168.1.252:8503".parse().unwrap(),
        Duration::from_secs(5),
    ) {
        Ok(mut socket) => {
            socket.set_read_timeout(Some(Duration::from_secs(5))).ok();

            println!("✅ Connected!");

            // Read SunSpec header (minimal code footprint)
            match read_regs(&mut socket, 40000, 10) {
                Ok(regs) => {
                    println!("Spec ID: {:?}", &regs[0..4]);
                    println!("Device ID: {}", regs[4]);
                    println!("Length: {} regs", regs[5]);
                },
                Err(e) => println!("Error: {}", e),
            }

            // Read measurements
            if let Ok(regs) = read_regs(&mut socket, 40070, 12) {
                println!("AC: W={}, VA={}, VAR={}", regs[0], regs[1], regs[2]);
            }
        }
        Err(e) => println!("Connection failed: {}", e),
    }
}

// Minimal Modbus TCP read (no external crates needed)
fn read_regs(socket: &mut TcpStream, addr: u16, count: u16) -> Result<Vec<u16>, String> {
    let mut req = vec![0u8; 12];

    // Modbus TCP request
    req[0..2].copy_from_slice(&[0, 1]);      // Transaction ID
    req[2..4].copy_from_slice(&[0, 0]);      // Protocol ID
    req[4..6].copy_from_slice(&[0, 6]);      // Length
    req[6] = 1;                               // Unit ID
    req[7] = 3;                               // Function code (read holding)
    req[8..10].copy_from_slice(&addr.to_be_bytes());
    req[10..12].copy_from_slice(&count.to_be_bytes());

    socket.write_all(&req).map_err(|e| e.to_string())?;

    // Read response
    let mut resp = vec![0u8; 256];
    let n = socket.read(&mut resp).map_err(|e| e.to_string())?;

    if n < 9 {
        return Err("Response too short".to_string());
    }

    let byte_count = resp[8] as usize;
    let mut result = Vec::new();

    for i in 0..(byte_count / 2) {
        let idx = 9 + i * 2;
        let val = u16::from_be_bytes([resp[idx], resp[idx + 1]]);
        result.push(val);
    }

    Ok(result)
}
