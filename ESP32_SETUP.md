# ESP32 Setup & Deployment Guide (macOS)

## Prerequisites

- ESP32 board (S2, S3, or standard)
- USB cable (USB-C or Micro-USB depending on board)
- macOS (Intel or Apple Silicon)
- Homebrew installed

## Step 1: Install ESP32 Toolchain

### Option A: Using Espressif's Official Tool (Recommended)

```bash
# Install espup
cargo install espup

# Install ESP32 toolchain (choose your board)
espup install

# Add to your shell (add to ~/.zshrc or ~/.bash_profile)
export PATH="$HOME/.espressif/tools/xtensa-esp32-elf/esp-13.2.0_20230426/xtensa-esp32-elf/bin:$PATH"
export PATH="$HOME/.espressif/tools/esp-riscv32-elf/esp-13.2.0_20230426/esp-riscv32-elf/bin:$PATH"
export PATH="$HOME/.espressif/tools/esp32ulp-elf/2.28.51-esp32-20220623/esp32ulp-elf/bin:$PATH"

# Reload shell
source ~/.zshrc
```

### Option B: Using Homebrew (Simpler)

```bash
brew install espressif/tap/idf-env
idf-env install
```

## Step 2: Add Rust Target

```bash
# Install Rust ESP32 target
rustup target add xtensa-esp32-espidf

# Or for other ESP32 variants:
rustup target add riscv32imc-esp-espidf    # ESP32-C3, ESP32-C2
rustup target add riscv32imac-esp-espidf   # ESP32-C6
```

## Step 3: Install esptool

```bash
pip3 install esptool
# or
cargo install espflash
```

## Step 4: Identify Your ESP32 Board

Connect your ESP32 via USB and check:

```bash
# List USB devices
ls -la /dev/tty.usb*
# or
ls -la /dev/cu.usbserial*
# or
ls -la /dev/cu.SLAB_USBtoUART*

# Typical output: /dev/cu.usbserial-1420 or /dev/cu.SLAB_USBtoUART
```

Note your port: `PORT=/dev/cu.usbserial-XXXX`

## Step 5: Configure Build Environment

Create `.cargo/config.toml` (already done):

```toml
[build]
target = "xtensa-esp32-espidf"

[target.xtensa-esp32-espidf]
runner = "espflash flash --monitor"
```

## Step 6: Build for ESP32

```bash
cd /Users/vishy/Downloads/sma_dersim_client

# Build the project
cargo +esp build --release --target xtensa-esp32-espidf

# Or with verbose output
cargo +esp build --release --target xtensa-esp32-espidf -v
```

**Expected build time:** 3-5 minutes (first time), 30 seconds (subsequent)

**Expected binary size:** 250-400 KB

## Step 7: Flash to ESP32

### Option A: Using cargo runner (automatic)

```bash
# This will build AND flash automatically
cargo +esp run --release --target xtensa-esp32-espidf --monitor

# The board will reset and start running
# You should see serial output in real-time
```

### Option B: Manual flash with espflash

```bash
# First, find your port
PORT=$(ls /dev/cu.usb* 2>/dev/null | head -1)
echo "Using port: $PORT"

# Flash the binary
espflash flash \
  target/xtensa-esp32-espidf/release/sma_test \
  --port $PORT

# Monitor output
espflash monitor --port $PORT
```

### Option C: Manual flash with esptool

```bash
PORT=/dev/cu.usbserial-1420  # Update with your port

# Flash
esptool.py --port $PORT --baud 460800 write_flash \
  --flash_mode dio \
  --flash_freq 80m \
  0x0 target/xtensa-esp32-espidf/release/sma_test.bin

# Monitor output
picocom $PORT -b 115200
# (quit with Ctrl+A then Q)
```

## Step 8: Verify Connection

Once flashed, you should see output:

```
🚀 ESP32 SMA DERSim Modbus Client
════════════════════════════════════════════════════════════════

📡 Connecting to WiFi: RaptorNetOG
  Waiting for IP address...
  ✓ WiFi connected!
  IP Address: 192.168.1.XXX

🔌 Connecting to DERSim at 192.168.1.252:8503...
✅ Connected!

📋 Reading SunSpec Device Information:
  Specification ID: SunS166
  Device ID: 21325
  Address Space: 16640 registers

📊 DERSim Measurements:

  AC Output (registers 40070):
    W=701, VA=153, VAR=2
    As 32-bit: 45940889W

  Power Readings (registers 40110):
    Value1=250, Value2=83

════════════════════════════════════════════════════════════════
✅ DERSim Connection Test Complete

✓ ESP32 WiFi: Connected to RaptorNetOG
✓ Modbus TCP: Connected to 192.168.1.252:8503
✓ Device Type: SunSpec-compliant

📝 Continuous monitoring mode
   Reading every 5 seconds

[00001s] W= 701 VA=  153 VAR=    2
[00006s] W= 701 VA=  153 VAR=    2
[00011s] W= 701 VA=  153 VAR=    2
```

## Troubleshooting

### "command not found: cargo +esp"

```bash
# Install esp tools
cargo install espup cargo-espflash
rustup toolchain install esp
```

### USB Port Not Found

```bash
# List all USB devices
system_profiler SPUSBDataType

# Or check /dev for USB devices
ls -la /dev/cu.*
```

### Permission Denied on /dev/tty.usb*

```bash
# Grant permission
sudo chmod 666 /dev/cu.usbserial-1420

# Or add yourself to dialout group (permanent)
sudo dserial -A $(whoami) dialout
```

### Build Fails: "xtensa-esp32-espidf not found"

```bash
# Verify installation
rustup target list | grep xtensa

# Reinstall
rustup target add xtensa-esp32-espidf
```

### WiFi Connection Fails

1. Check SSID/password in `src/main.rs`:
   ```rust
   const SSID: &str = "RaptorNetOG";
   const PASSWORD: &str = "C@rtoonNetwork";
   ```

2. Verify WiFi is running (no captive portal)

3. Check ESP32 is in range (WiFi signal strength)

4. Try resetting device: Press EN button 2x quickly

### Can't Connect to DERSim (192.168.1.252:8503)

1. Verify DERSim is running:
   ```bash
   docker ps | grep dersim
   ```

2. Check IP address - ESP32 should be on same subnet (192.168.1.x)

3. Test connectivity from macOS:
   ```bash
   nc -zv 192.168.1.252 8503
   # or
   ping 192.168.1.252
   ```

4. Check firewall isn't blocking port 8503

### Board Won't Start After Flash

1. Try pressing the EN (enable) button

2. Check USB power - some cables are data-only

3. Try different USB port on Mac

4. Flash the factory reset:
   ```bash
   esptool.py --port /dev/cu.XXXX erase_flash
   # Then flash again
   ```

## Next Steps

Once verified:

1. **Continuous Monitoring**: Device will monitor every 5s (modify interval in code)

2. **Power Down**: Unplug USB to save battery

3. **Remote Operation**: Keep ESP32 powered via USB/PoE for long-term monitoring

4. **Data Logging**: Could add SD card storage or send data to cloud

## Development Tips

### Modify WiFi/Settings

Edit `src/main.rs`:
```rust
const SSID: &str = "YourNetwork";
const PASSWORD: &str = "YourPassword";
const DERSIM_ADDR: &str = "192.168.1.252:8503";
```

Then rebuild:
```bash
cargo +esp run --release --target xtensa-esp32-espidf --monitor
```

### Monitor Only (no rebuild)

```bash
espflash monitor --port /dev/cu.usbserial-1420
```

### Full Clean Build

```bash
cargo clean
cargo +esp build --release --target xtensa-esp32-espidf
```

### Check Binary Size

```bash
ls -lh target/xtensa-esp32-espidf/release/sma_test
# Should be 250-400 KB
```

## Board-Specific Notes

| Board | USB Port | Target | Flash Size |
|-------|----------|--------|-----------|
| ESP32 | Micro-USB | xtensa-esp32-espidf | 4MB |
| ESP32-S2 | USB-C | xtensa-esp32s2-espidf | 4MB |
| ESP32-S3 | USB-C | xtensa-esp32s3-espidf | 8MB ⭐ |
| ESP32-C3 | USB-C | riscv32imc-esp-espidf | 4MB |
| ESP32-C6 | USB-C | riscv32imac-esp-espidf | 8MB |

If using S2/S3/C3/C6, adjust Cargo.toml target accordingly.

---

**You're ready to deploy!** 🚀
