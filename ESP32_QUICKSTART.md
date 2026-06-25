# ESP32 Modbus Client - Quick Start

## 🎯 30-Second Setup

### Prerequisites
- ESP32 device (any variant)
- USB cable
- Connected to same network as DERSim (RaptorNetOG with C@rtoonNetwork)

### Commands

```bash
# 1. Run setup script (installs toolchain)
./setup_esp32.sh

# 2. Connect ESP32 via USB

# 3. Flash to device
cargo +esp run --release --target xtensa-esp32-espidf --monitor

# 4. Watch serial output - should show:
# ✅ WiFi: Connected to RaptorNetOG
# ✅ DERSim: Connected at 192.168.1.252:8503
# 📊 Measurements streaming every 5 seconds
```

That's it! 🎉

---

## Detailed Walkthrough

### Step 1: Prepare Device

```bash
# Connect ESP32 to USB on Mac
# LED should light up (or blink if bootloader mode)
```

### Step 2: Install Toolchain (one-time)

```bash
cd /Users/vishy/Downloads/sma_dersim_client
./setup_esp32.sh

# This installs:
# - espup (Espressif toolchain manager)
# - espflash (flashing tool)
# - xtensa-esp32-espidf target for Rust
#
# Takes ~2-3 minutes the first time
```

### Step 3: Build & Flash

```bash
# This builds the project and flashes to device automatically
cargo +esp run --release --target xtensa-esp32-espidf --monitor

# Expected output:
# [00:00] Compiling...
# [00:30] Flashing...
# [00:45] Booting ESP32...
# [01:00] 🚀 ESP32 SMA DERSim Modbus Client
#        📡 Connecting to WiFi: RaptorNetOG
#        ✓ WiFi connected!
#        IP Address: 192.168.1.XXX
#        🔌 Connecting to DERSim...
#        ✅ Connected!
#        📊 Measurements:
#        [00001s] W= 701 VA=  153 VAR=    2
#        [00006s] W= 701 VA=  153 VAR=    2
```

Press `Ctrl+C` to stop monitoring (device keeps running).

---

## What's Happening

1. **WiFi Connection**: Connects to "RaptorNetOG" network
2. **DERSim Connection**: Connects to 192.168.1.252:8503 (Modbus TCP)
3. **Device Discovery**: Reads SunSpec header
4. **Continuous Monitoring**: Reads power measurements every 5 seconds

---

## Troubleshooting

### ❌ "command not found: cargo +esp"

```bash
# The setup script didn't complete. Try manually:
cargo install espup cargo-espflash
rustup toolchain install esp
```

### ❌ "No USB device found"

```bash
# Check what ports are available:
ls -la /dev/cu.*

# If nothing shows up:
# 1. Try different USB cable (some are charge-only)
# 2. Check Device Manager (Arduino IDE → Tools → Port)
# 3. Try different USB port on Mac
```

### ❌ WiFi "Connection timeout"

The ESP32 will timeout after 30 seconds. Try:
1. Check WiFi is broadcasting (not hidden)
2. Check password in `src/main.rs` line 13-14
3. Verify ESP32 antenna (S3 has antenna on board)
4. Check WiFi signal strength near ESP32

### ❌ "Connection failed to 192.168.1.252:8503"

1. Verify DERSim is running:
   ```bash
   docker ps | grep dersim
   ```

2. Test from Mac:
   ```bash
   nc -zv 192.168.1.252 8503
   ```

3. Check ESP32 has correct IP (should print during boot)

4. Check firewall isn't blocking:
   ```bash
   sudo ufw allow 8503
   ```

### ⚠️ Build takes forever

First build takes 3-5 minutes (compiling tokio + dependencies). Subsequent builds are ~30 seconds.

---

## Customization

### Change WiFi Network

Edit `src/main.rs`:
```rust
const SSID: &str = "YourNetworkName";       // Line 13
const PASSWORD: &str = "YourPassword";      // Line 14
const DERSIM_ADDR: &str = "192.168.1.252:8503";  // Line 15
```

Then rebuild:
```bash
cargo +esp run --release --target xtensa-esp32-espidf --monitor
```

### Change Measurement Interval

Edit `src/main.rs` around line 140:
```rust
thread::sleep(Duration::from_secs(5));  // Change 5 to your desired seconds
```

### Change Modbus Registers

Edit `src/main.rs` around lines 88-95:
```rust
if let Ok(regs) = read_holding_registers(&mut socket, 40070, 12) {
    // ↑ 40070 is starting register
    // ↑ 12 is how many registers to read
}
```

---

## Success Indicators

✅ **You know it's working when you see:**

1. **WiFi connected message** - ESP32 joined the network
2. **DERSim IP printed** - Device found correct IP
3. **SunSpec ID (SunS166)** - Device type recognized
4. **Measurements streaming** - Live power values updating

```
[00001s] W= 701 VA=  153 VAR=    2  ← New measurement every 5 sec
[00006s] W= 701 VA=  153 VAR=    2
[00011s] W= 700 VA=  154 VAR=    3
```

---

## Next Steps

Once working:

### Option A: Long-term Monitoring
- Leave plugged in via USB for continuous operation
- Could add SD card for data logging
- Could add cloud upload (MQTT, HTTP)

### Option B: Battery Operation
- Use USB power bank
- Modify sleep intervals to reduce power usage
- Expected battery life: 8-24 hours depending on power bank

### Option C: Integration
- Connect to larger monitoring system
- Add temperature sensors
- Add data logging

---

## Key Files

| File | Purpose |
|------|---------|
| `src/main.rs` | ESP32 implementation (WiFi + Modbus) |
| `src/main_linux.rs` | Original Linux version (for reference) |
| `.cargo/config.toml` | Build configuration |
| `Cargo.toml` | Dependencies |
| `setup_esp32.sh` | Automated toolchain setup |
| `ESP32_SETUP.md` | Full troubleshooting guide |

---

## Support

### Documentation
- See `ESP32_SETUP.md` for detailed troubleshooting
- See `DEPLOYMENT.md` for cross-platform notes

### Hardware Docs
- [ESP32 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf)
- [ESP-IDF Programming Guide](https://docs.espressif.com/projects/esp-idf/en/latest/esp32/)

### Community
- `#rust-esp32` on IRCC
- GitHub: `esp-rs/esp-idf-svc`

---

**You're all set!** 🚀 Enjoy monitoring your DERSim on ESP32!
