# Flash ESP32-S3 with DERSim Modbus Client

## Quick Start (Your macOS)

### Prerequisites
- ESP32-S3 board connected via USB-C
- `espflash` installed: `cargo install espflash`
- Pre-built binary (from GitHub Actions or provided)

### Step 1: Get the Binary

**Option A: Build via GitHub Actions** (Recommended)
1. Push this code to GitHub
2. Go to Actions tab
3. Run "Build ESP32-S3 Binary"
4. Download artifact: `sma_test-esp32s3`

**Option B: Use Pre-built Binary**
- Download from: [Release Page](https://github.com/yourusername/sma_dersim_client/releases)

### Step 2: Flash to Board

```bash
# Verify board is connected
ls /dev/cu.usbmodem*

# Flash the binary
espflash flash ./sma_test \
  --port /dev/cu.usbmodem2101 \
  --baud 460800
```

### Step 3: Monitor Output

```bash
espflash monitor --port /dev/cu.usbmodem2101 --baud 115200
```

Expected output:
```
🚀 ESP32-S3 SMA DERSim Modbus Client
📡 Network: RaptorNetOG
🔌 DERSim Target: 192.168.1.252:8503
🔌 Connecting to DERSim at 192.168.1.252:8503...
✅ Connected to DERSim!

📋 Reading SunSpec Device Information:
  Specification ID: SunS166
  Device ID: 21325

⚡ W=701 | VA=153 | VAR=2
⚡ W=701 | VA=153 | VAR=2
```

---

## macOS Build (Advanced)

If you want to build on your Mac:

### Install Proper Toolchain

```bash
# Option 1: Install via espup (official method)
cargo install espup
espup install --targets esp32s3

# Add to PATH (add to ~/.zshrc or ~/.bash_profile)
export PATH="$HOME/.espressif/tools/xtensa-esp32s3-elf/esp-13.2.0_20230426/xtensa-esp32s3-elf/bin:$PATH"
```

### Build

```bash
source "$HOME/.cargo/env"
source "$HOME/.espressif/esp-idf/export.sh"  # If using esp-idf

cd /Users/vishy/Downloads/sma_dersim_client
cargo +esp build --release --target xtensa-esp32s3-espidf -Z build-std=core,alloc,std,panic_abort
```

### Flash

```bash
espflash flash target/xtensa-esp32s3-espidf/release/sma_test \
  --port /dev/cu.usbmodem2101
```

---

## Docker Build (Most Reliable)

```bash
cd /Users/vishy/Downloads/sma_dersim_client

# Build Docker image
docker build -t sma-esp32-builder .

# Run build inside container
docker run --rm -v $(pwd):/app sma-esp32-builder

# Binary is now at: target/xtensa-esp32s3-espidf/release/sma_test

# Flash
espflash flash target/xtensa-esp32s3-espidf/release/sma_test \
  --port /dev/cu.usbmodem2101
```

---

## Troubleshooting

### "Board not found"
```bash
# Check port
ls /dev/cu.usbmodem*

# If board doesn't show, try:
# 1. Different USB cable (some are charge-only)
# 2. Different USB port
# 3. Press RST button on board
```

### "Linker not found"
```bash
# Make sure PATH is set
which xtensa-esp32s3-elf-gcc

# If not found, add to ~/.zshrc:
export PATH="$HOME/.espressif/tools/xtensa-esp32s3-elf/esp-13.2.0_20230426/xtensa-esp32s3-elf/bin:$PATH"
source ~/.zshrc
```

### "Connection timeout"
- Verify WiFi is connected to `RaptorNetOG`
- Check DERSim is running: `docker ps | grep dersim`
- Test connectivity: `nc -zv 192.168.1.252 8503`

---

## Success!

Once flashing completes and you see measurements streaming, your ESP32-S3 is:
- ✅ Connected to RaptorNetOG WiFi
- ✅ Connected to DERSim at 192.168.1.252:8503  
- ✅ Reading live SunSpec data
- ✅ Monitoring inverter measurements

The board will continue running even if you disconnect the USB cable (powered by the USB connection itself, or add external power).

---

## Next Steps

- Modify WiFi credentials in `src/main.rs` if needed
- Change measurement interval (line ~180)
- Add external LED on GPIO10 for status indication
- Set up long-term monitoring with external power

