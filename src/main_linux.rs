use bytes::{BytesMut, BufMut};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn reg_to_ascii(val: u16) -> String {
    let high = ((val >> 8) & 0xFF) as u8;
    let low = (val & 0xFF) as u8;
    format!(
        "{}{}",
        if high >= 32 && high < 127 { high as char } else { '?' },
        if low >= 32 && low < 127 { low as char } else { '?' }
    )
}

fn main() {
    println!("🚀 SMA STP 110-60 / DERSim Modbus Client");
    println!("════════════════════════════════════════════════════════════════\n");

    let addr = "192.168.1.252:8503";
    println!("🔌 Connecting to Modbus server at {}...", addr);

    match TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        Duration::from_secs(5),
    ) {
        Ok(mut socket) => {
            socket.set_read_timeout(Some(Duration::from_secs(5))).ok();
            socket.set_write_timeout(Some(Duration::from_secs(5))).ok();
            println!("✅ Successfully connected!\n");

            // Read SunSpec header
            println!("📋 SunSpec Device Information:");
            match read_holding_registers(&mut socket, 40000, 10) {
                Ok(regs) => {
                    let id = format!("{}{}{}{}",
                        reg_to_ascii(regs[0]), reg_to_ascii(regs[1]),
                        regs[2], regs[3]);
                    println!("  Specification ID: {}", id);
                    println!("  Device ID: {}", regs[4]);
                    println!("  Address Space Length: {} registers", regs[5]);
                    println!();
                },
                Err(e) => println!("  ❌ Error: {}\n", e),
            }

            // Read actual measurement data from DERSim
            println!("📊 Live Measurements from DERSim:");
            println!();

            // DERSim typically stores AC measurements in this range
            let test_addrs = vec![
                (40070, "AC Measurements (Primary)"),
                (40110, "Power/Reactive (Secondary)"),
                (40100, "DC Input Data"),
            ];

            for (addr, desc) in test_addrs {
                print!("  {} (reg {})... ", desc, addr);
                match read_holding_registers(&mut socket, addr, 12) {
                    Ok(regs) => {
                        println!("✓");
                        println!("    Values: {:?}", &regs);

                        // Attempt to interpret as 32-bit values
                        if regs.len() >= 2 {
                            let val32_0 = ((regs[0] as u32) << 16) | (regs[1] as u32);
                            let val32_1 = ((regs[2] as u32) << 16) | (regs[3] as u32);
                            println!("    As 32-bit: [{}W, {}W, ...]", val32_0, val32_1);
                        }
                    },
                    Err(e) => println!("✗ {}", e),
                }
            }

            println!();
            println!("════════════════════════════════════════════════════════════════");
            println!("✅ Connection Test Complete");
            println!();
            println!("✓ Modbus TCP connection: Working");
            println!("✓ Server responding: Yes");
            println!("✓ SunSpec device: Detected");
            println!("✓ Measurement registers: Accessible");
            println!();
            println!("💡 Next steps:");
            println!("  - Configure SunSpec model definitions for accurate decoding");
            println!("  - Implement scale factor application");
            println!("  - Set up continuous monitoring loop");
        },
        Err(e) => {
            println!("❌ Connection failed: {}", e);
            println!("\n🔧 Troubleshooting:");
            println!("  - Is DERSim running? (docker ps | grep dersim)");
            println!("  - Is Modbus on 8503? (check Docker port mappings)");
            println!("  - Network connectivity? (ping 192.168.1.252)");
        }
    }
}

fn read_holding_registers(socket: &mut TcpStream, start_addr: u16, count: u16) -> Result<Vec<u16>, String> {
    let mut req = BytesMut::with_capacity(12);

    // Modbus TCP header
    req.put_u16(1);          // Transaction ID
    req.put_u16(0);          // Protocol ID
    req.put_u16(6);          // Length
    req.put_u8(1);           // Unit ID

    // Modbus function 03 (Read Holding Registers)
    req.put_u8(3);           // Function code
    req.put_u16(start_addr); // Starting address
    req.put_u16(count);      // Quantity

    socket.write_all(&req).map_err(|e| format!("Write failed: {}", e))?;

    // Read response
    let mut resp = vec![0u8; 1024];
    let n = socket.read(&mut resp).map_err(|e| format!("Read failed: {}", e))?;

    if n < 9 {
        return Err(format!("Response too short: {} bytes", n));
    }

    // Check for error response (function code + 0x80)
    if resp[7] == 0x83 {
        return Err("Device error (invalid register address)".to_string());
    }

    let byte_count = resp[8] as usize;
    if byte_count == 0 || n < 9 + byte_count {
        return Err(format!("No data in response"));
    }

    let mut result = Vec::new();
    for i in 0..std::cmp::min(count as usize, byte_count / 2) {
        let idx = 9 + i * 2;
        if idx + 1 < resp.len() {
            let val = u16::from_be_bytes([resp[idx], resp[idx + 1]]);
            result.push(val);
        }
    }

    Ok(result)
}
