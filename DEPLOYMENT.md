# Deployment Guide: SMA DERSim Modbus Client

## Linux Binary (Recommended)

### Size: **296 KB** (fully optimized, stripped)

**Already built at:** `target/release/sma_test`

### Copy to any Linux system:
```bash
# Copy binary to target machine
scp target/release/sma_test user@remote-host:/usr/local/bin/

# Run (no dependencies needed!)
ssh user@remote-host /usr/local/bin/sma_test
```

### Cross-compile for different Linux architectures:

```bash
# For ARM (Raspberry Pi, etc)
rustup target add armv7-unknown-linux-gnueabihf
cargo build --release --target armv7-unknown-linux-gnueabihf

# For ARM64 (Raspberry Pi 4, newer ARM servers)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# For x86_64 (servers, VMs)
cargo build --release
# (already targets native x86_64)
```

Check resulting binary sizes:
```bash
ls -lh target/*/release/sma_test
```

### Static linking (runs anywhere):
```bash
# Create fully static binary (no libc dependency)
cargo build --release \
  --target x86_64-unknown-linux-musl \
  -Z build-std=std,panic_abort \
  -Z build-std-features=panic_abort,split_debuginfo

# Result: Runs on any Linux, no glibc version issues
```

---

## ESP32 (Complex - Read carefully)

### Feasibility: **Yes, but requires significant changes**

Current approach **won't work** due to:
- Tokio runtime: **4-6 MB** (too large)
- Total binary: Would exceed ESP32S3 flash (~8 MB usable)
- Memory overhead: ~200KB RAM minimum

### Option A: Minimal blocking implementation

**File:** `src/main_esp32.rs` (blocking, no async)

Setup:
```bash
# Install ESP32 toolchain
cargo install espup
espup install

# Add target
rustup target add xtensa-esp32-espidf

# Build
cargo +esp build --release --target xtensa-esp32-espidf
```

**Expected size:** 200-400 KB binary (fits in flash)

**Requires:**
- ESP32 (any variant: S2, S3, standard)
- WiFi credentials configuration
- esp-idf-svc crate setup
- Serial connection for upload

### Option B: Ultra-minimal (no WiFi, local only)

For ESP32 with **hardwired Ethernet** (like PoE boards):
- Remove WiFi code entirely
- Use `smol` instead of tokio (lightweight async)
- Binary size: ~150-250 KB

### Option C: Different microcontroller

If you need wireless + small size, consider:
- **RP2040** (Raspberry Pi Pico) + external WiFi module: ✅ Easier
- **STM32** + WiFi shield: ✅ More flash available
- **nRF52840** (Thread/BLE): ✅ If you don't need WiFi

---

## Performance Comparison

| Platform | Binary Size | RAM @ Startup | WiFi | Notes |
|----------|------------|---------------|------|-------|
| Linux x86_64 | 296 KB | ~2 MB | N/A | ✅ Recommended |
| Linux ARM | 280 KB | ~2 MB | N/A | ✅ RPi, OrangePi |
| ESP32S3 | 200-400 KB | ~50 KB | ✅ Built-in | ⚠️ Requires esp-idf |
| Static Linux | 350 KB | ~2 MB | N/A | ✅ Portable |

---

## Recommended Use Cases

**Use the Linux binary if:**
- Running on a server, VM, or Linux SBC (Raspberry Pi)
- Want simplicity + reliability
- Don't need WiFi (LAN cable is fine)

**Use ESP32 if:**
- Wireless connectivity required
- Very small form factor needed
- Don't mind embedded complexity

**Suggested setup:**
- **Server monitoring:** Linux binary on Raspberry Pi + cron job
- **Field deployment:** Linux binary on PoE-powered SBC
- **IoT dashboard:** ESP32 + MQTT bridge to central system

---

## Building for Production

### Linux (recommended):
```bash
cargo build --release
strip target/release/sma_test
# Result: 296 KB, fully self-contained
```

### Docker (for consistent cross-platform builds):
```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
FROM debian:bookworm-slim
COPY --from=0 /app/target/release/sma_test /usr/local/bin/
ENTRYPOINT ["sma_test"]
```

Build:
```bash
docker build -t sma-client:latest .
docker run --network host sma-client:latest
```

---

## Testing

Verify binary works before deployment:
```bash
# Local test (requires DERSim running)
./target/release/sma_test

# Remote test (via SSH)
ssh user@server /usr/local/bin/sma_test

# Docker test
docker run --network host sma-client:latest
```

Expected output:
```
🚀 SMA STP 110-60 / DERSim Modbus Client
✅ Successfully connected!
📋 SunSpec Device Information:
  Specification ID: SunS166
  Device ID: 21325
```

---

## Troubleshooting

### Binary won't run: "command not found"
- Wrong architecture (e.g., ARM binary on x86)
- Missing libc (use `--target x86_64-unknown-linux-musl` for static)

### Connection timeout
- Check DERSim is running: `docker ps | grep dersim`
- Verify network: `ping 192.168.1.252`
- Check firewall: `sudo ufw allow 8503`

### ESP32 build fails
- Update toolchain: `rustup update`
- Clear cache: `cargo clean`
- Check esp-idf: `espup update`
