# Seeed Studio XIAO ESP32S3 - DERSim Setup

## ⚡ Quick Start (5 minutes)

### Prerequisites
- Seeed Studio XIAO ESP32S3 board
- USB-C cable connected to macOS
- WiFi: RaptorNetOG (password: C@rtoonNetwork)

### Step 1: Install Toolchain (one-time, ~3 min)

```bash
cd /Users/vishy/Downloads/sma_dersim_client
./setup_esp32.sh
```

### Step 2: Put Board in Download Mode

1. **Connect** USB-C cable to XIAO ESP32S3
2. **Hold down** the BOOT button
3. **Press** the RST button once while holding BOOT
4. **Release** BOOT button
5. Board should now show as a USB device

Verify:
```bash
ls -la /dev/cu.usbmodem*
# Should see something like: /dev/cu.usbmodem14101
```

### Step 3: Flash & Run

```bash
cargo +esp run --release --target xtensa-esp32-espidf --monitor
```

### Step 4: Watch Serial Output

```
🚀 ESP32 SMA DERSim Modbus Client (Seeed XIAO)
📡 Connecting to WiFi: RaptorNetOG
✅ WiFi connected!
IP Address: 192.168.1.XXX

📋 Reading SunSpec Device Information:
  Specification ID: SunS166
  Device ID: 21325

📊 DERSim Live Measurements:
  AC Output (registers 40070):
    W=701, VA=153, VAR=2

✓ ESP32 WiFi:        Connected to RaptorNetOG
✓ Modbus TCP:        Connected to 192.168.1.252:8503
✓ Device Type:       SunSpec-compliant (Model 1 detected)

📝 Continuous monitoring mode (reading every 5 seconds)

[00005] ⚡ W= 701 | VA=  153 | VAR=    2
[00010] ⚡ W= 701 | VA=  153 | VAR=    2
[00015] ⚡ W= 700 | VA=  154 | VAR=    3
```

✅ **Done!** Device is now monitoring DERSim

---

## 🔧 Detailed Instructions

### Seeed XIAO ESP32S3 Board Info

- **Microcontroller**: ESP32-S3 (2 cores, 240 MHz)
- **RAM**: 8 MB PSRAM + 512 KB SRAM
- **Flash**: 8 MB
- **WiFi**: Built-in 802.11 a/b/g/n
- **USB**: USB-C (native)
- **Buttons**: BOOT (GPIO9), RST
- **Antenna**: Built-in PCB antenna
- **No GPIO pins soldered** (custom pinout via JST connectors)

### Board Pinout

```
           ┌─────────────┐
           │  XIAO ESP32 │
    ───────┤   S3        ├───────
    GND    │ USB-C    5V │
    D10    │             │ B+
    D9     │             │ B-
    D8     │   [RST]     │ G
    D7     │  [BOOT]     │ ✓
    D6     │             │ SDA
    D5     │             │ SCL
    D4     │             │
    D3     │             │
    D2     │             │
    D1     │             │
    D0     │             │
    TX     │             │ RX
           └─────────────┘
```

### Installing ESP32 Toolchain

The `setup_esp32.sh` script will:
1. Install `espup` (Espressif toolchain manager)
2. Install `espflash` (flashing tool)
3. Add Rust target `xtensa-esp32s3-espidf`

If you need to install manually:

```bash
# Install espup
cargo install espup

# Run espup to install tools
espup install

# Add Rust target
rustup target add xtensa-esp32s3-espidf
```

### Flashing the Code

#### Option A: Automatic (recommended)

```bash
cd /Users/vishy/Downloads/sma_dersim_client
cargo +esp run --release --target xtensa-esp32s3-espidf --monitor
```

This will:
1. Compile the code
2. Detect the board
3. Flash automatically
4. Open serial monitor

#### Option B: Manual Flash

```bash
# Build
cargo +esp build --release --target xtensa-esp32s3-espidf

# Flash with espflash
espflash flash target/xtensa-esp32s3-espidf/release/sma_test --monitor

# Or with esptool
PORT=$(ls /dev/cu.usbmodem* | head -1)
esptool.py --chip esp32s3 --port $PORT write_flash \
  0x0 target/xtensa-esp32s3-espidf/release/sma_test.bin
```

### Monitoring Output

Once flashed, the board runs automatically. To see output:

```bash
# Using espflash
espflash monitor --baud 115200

# Or using cargo
cargo +esp run --release --target xtensa-esp32s3-espidf --monitor

# Or manual serial (requires picocom)
picocom /dev/cu.usbmodem14101 -b 115200
```

Press `Ctrl+C` in the terminal to stop (board keeps running).

---

## ❌ Troubleshooting

### "No device found" during flash

1. **Check USB connection**:
   ```bash
   ls -la /dev/cu.usb*
   ```
   Should show `/dev/cu.usbmodem*`

2. **Put board in download mode**:
   - Hold BOOT button
   - Press RST once
   - Release BOOT
   - Try again

3. **Try different USB port** on Mac

4. **Check if another app is using port**:
   ```bash
   lsof | grep cu.usbmodem
   # Kill any processes using the port
   ```

### Build fails: "xtensa-esp32s3-espidf not found"

```bash
rustup target list | grep esp32s3
# If not found, install:
rustup target add xtensa-esp32s3-espidf
```

### WiFi connection times out

1. Check SSID is correct (case-sensitive): `RaptorNetOG`
2. Check password: `C@rtoonNetwork`
3. Verify WiFi is not hidden
4. Check ESP32 antenna is making contact
5. Move closer to WiFi router
6. Try resetting board (press RST)

### Can't connect to DERSim (192.168.1.252)

1. Verify DERSim is running:
   ```bash
   docker ps | grep dersim
   ```

2. Test from macOS:
   ```bash
   nc -zv 192.168.1.252 8503
   ping 192.168.1.252
   ```

3. Check ESP32 got correct IP address:
   - Should show `IP Address: 192.168.1.XXX` in serial output
   - If shows `0.0.0.0`, WiFi isn't fully connected

4. Check firewall:
   ```bash
   sudo ufw allow 8503
   ```

### Serial output shows garbage

- **Wrong baud rate**: Should auto-detect at 115200
- **Try manual**: `picocom /dev/cu.usbmodem14101 -b 115200`

### Board won't reset or flash

1. Unplug USB
2. Wait 5 seconds
3. Plug back in
4. Try again

If still stuck:
```bash
# Force reset via esptool
esptool.py --chip esp32s3 --port /dev/cu.usbmodem14101 reset
```

---

## 📊 Performance Stats

```
Build time (first):        ~3-4 minutes
Build time (subsequent):   ~20-30 seconds
Binary size:               ~350 KB
Flash usage:               ~5% of 8MB
RAM at startup:            ~50 KB
WiFi connection time:      ~2-5 seconds
DERSim connection time:    <1 second
Measurement read interval: 5 seconds
```

---

## 🔌 Adding External LED (Optional)

If you want LED status feedback later, you can add an external LED via jumper wires:

```
XIAO Pin D10 ----[220Ω resistor]----[LED anode]
                                      LED cathode ---- GND
```

Then uncomment the LED code in `src/main.rs` and set GPIO10.

---

## 📝 Customization

### Change WiFi Network

Edit `src/main.rs`:
```rust
const SSID: &str = "YourNetwork";
const PASSWORD: &str = "YourPassword";
```

Then rebuild:
```bash
cargo +esp run --release --target xtensa-esp32s3-espidf --monitor
```

### Change Measurement Interval

Edit `src/main.rs` around line 180:
```rust
thread::sleep(Duration::from_secs(5));  // Change 5 to your desired seconds
```

### Change Modbus Registers

Edit lines 90-95 for different measurements.

---

## ✅ Success Checklist

- [ ] `./setup_esp32.sh` completed without errors
- [ ] Board detected as `/dev/cu.usbmodem*`
- [ ] Code compiled successfully
- [ ] Code flashed to board
- [ ] Serial output shows "WiFi connected"
- [ ] Serial output shows "DERSim Connected"
- [ ] Measurements updating every 5 seconds
- [ ] Device is monitoring live power data

## 🎉 You're All Set!

Your ESP32 XIAO is now monitoring the DERSim simulator in real-time!

---

## Useful Commands

```bash
# Full rebuild + flash + monitor
cargo +esp run --release --target xtensa-esp32s3-espidf --monitor

# Just monitor (no rebuild)
espflash monitor --baud 115200

# Check board is detected
ls /dev/cu.usbmodem*

# Reset board
esptool.py --chip esp32s3 --port /dev/cu.usbmodem14101 reset

# Erase and reflash
esptool.py --chip esp32s3 --port /dev/cu.usbmodem14101 erase_flash
cargo +esp run --release --target xtensa-esp32s3-espidf --monitor

# Check binary size
ls -lh target/xtensa-esp32s3-espidf/release/sma_test
```

---

## Need Help?

Check:
1. `ESP32_SETUP.md` - Detailed troubleshooting
2. `DEPLOYMENT.md` - General info
3. [Seeed Wiki](https://wiki.seeedstudio.com/xiao_esp32s3_getting_started/) - Board docs
4. [ESP-IDF Docs](https://docs.espressif.com/projects/esp-idf/en/latest/esp32s3/)
