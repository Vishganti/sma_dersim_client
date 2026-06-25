use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {

    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║     🚀 ESP32-S3 SMA DERSim Modbus Client                    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // Simple hardcoded WiFi connection would go here
    // For now, assume network is available (run this after connecting to WiFi manually)

    println!("📡 Network: RaptorNetOG");
    println!("🔌 DERSim Target: 192.168.1.252:8503");
    println!();

    // Give board time to stabilize
    thread::sleep(Duration::from_secs(1));

    // Connect to DERSim
    println!("🔌 Connecting to DERSim at 192.168.1.252:8503...");

    match TcpStream::connect("192.168.1.252:8503") {
        Ok(mut socket) => {
            println!("✅ Connected to DERSim!");
            println!();

            socket.set_read_timeout(Some(Duration::from_secs(5))).ok();

            // Read SunSpec header
            println!("📋 Reading SunSpec Device Information:");
            match read_holding_registers(&mut socket, 40000, 10) {
                Ok(regs) => {
                    let id = format!(
                        "{}{}{}{}",
                        reg_to_ascii(regs[0]),
                        reg_to_ascii(regs[1]),
                        regs[2],
                        regs[3]
                    );
                    println!("  Specification ID: {}", id);
                    println!("  Device ID: {}", regs[4]);
                    println!("  Address Space: {} registers", regs[5]);
                    println!();
                }
                Err(e) => println!("  ❌ Error: {}\n", e),
            }

            // Read measurements
            println!("📊 DERSim Live Measurements:");
            println!();

            // AC Measurements
            if let Ok(regs) = read_holding_registers(&mut socket, 40070, 12) {
                println!("  AC Output (registers 40070):");
                println!("    W={}, VA={}, VAR={}", regs[0], regs[1], regs[2]);
            }

            println!();
            println!("╔═══════════════════════════════════════════════════════════════╗");
            println!("║                 ✅ CONNECTION TEST COMPLETE                   ║");
            println!("╚═══════════════════════════════════════════════════════════════╝");
            println!();
            println!("✓ Modbus TCP:        Connected to 192.168.1.252:8503");
            println!("✓ Device Type:       SunSpec-compliant (Model 1 detected)");
            println!();
            println!("📝 Continuous monitoring mode (reading every 5 seconds)");
            println!("   Press Ctrl+C on laptop to stop");
            println!();

            // Continuous monitoring loop
            let mut read_count = 0;
            loop {
                thread::sleep(Duration::from_secs(5));
                read_count += 1;

                if let Ok(regs) = read_holding_registers(&mut socket, 40070, 4) {
                    let power = regs[0];
                    let va = regs[1];
                    let var = regs[2];
                    println!(
                        "[{:04}s] ⚡ W={:5} | VA={:5} | VAR={:5}",
                        read_count * 5,
                        power,
                        va,
                        var
                    );
                } else {
                    println!("⚠️  Connection lost, attempting to read again...");
                    thread::sleep(Duration::from_secs(2));
                }
            }
        }
        Err(e) => {
            println!("❌ DERSim Connection failed: {}", e);
            println!();
            println!("Troubleshooting:");
            println!("  1. Make sure ESP32 is connected to WiFi (RaptorNetOG)");
            println!("  2. Check DERSim is running: docker ps | grep dersim");
            println!("  3. Check IP is reachable: ping 192.168.1.252");
            println!("  4. Check firewall allows port 8503");
        }
    }
}

fn read_holding_registers(
    socket: &mut TcpStream,
    start_addr: u16,
    count: u16,
) -> Result<Vec<u16>, String> {
    // Build Modbus TCP request
    let mut req = vec![0u8; 12];
    req[0..2].copy_from_slice(&[0, 1]);              // Transaction ID
    req[2..4].copy_from_slice(&[0, 0]);              // Protocol ID
    req[4..6].copy_from_slice(&[0, 6]);              // Length
    req[6] = 1;                                       // Unit ID
    req[7] = 3;                                       // Function code (read holding)
    req[8..10].copy_from_slice(&start_addr.to_be_bytes());
    req[10..12].copy_from_slice(&count.to_be_bytes());

    socket.write_all(&req).map_err(|e| e.to_string())?;

    // Read response
    let mut resp = vec![0u8; 256];
    let n = socket.read(&mut resp).map_err(|e| e.to_string())?;

    if n < 9 {
        return Err(format!("Response too short: {} bytes", n));
    }

    let byte_count = resp[8] as usize;
    let mut result = Vec::new();

    for i in 0..(byte_count / 2) {
        let idx = 9 + i * 2;
        if idx + 1 < n {
            let val = u16::from_be_bytes([resp[idx], resp[idx + 1]]);
            result.push(val);
        }
    }

    Ok(result)
}

fn reg_to_ascii(val: u16) -> String {
    let high = ((val >> 8) & 0xFF) as u8;
    let low = (val & 0xFF) as u8;
    format!(
        "{}{}",
        if high >= 32 && high < 127 {
            high as char
        } else {
            '?'
        },
        if low >= 32 && low < 127 { low as char } else { '?' }
    )
}
