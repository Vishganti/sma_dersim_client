#!/bin/bash
set -e

echo "🚀 ESP32-S3 DERSim Modbus Client - Build & Flash"
echo "════════════════════════════════════════════════════════════════"
echo ""

# Step 1: Setup Rust environment
echo "Step 1: Setting up Rust environment..."
source "$HOME/.cargo/env"
echo "✓ Rust environment loaded"
echo ""

# Step 2: Ensure espup is installed
echo "Step 2: Ensuring espup is installed..."
if ! command -v espup &> /dev/null; then
    echo "Installing espup..."
    cargo install espup
fi
echo "✓ espup ready"
echo ""

# Step 3: Setup ESP32-S3 toolchain
echo "Step 3: Setting up ESP32-S3 toolchain..."
if [ ! -d "$HOME/.espressif" ]; then
    echo "Running espup install (first time, takes a few minutes)..."
    # Use heredoc to avoid interactive prompts
    espup install --skip-confirmation --targets esp32s3 || \
    espup install --targets esp32s3 2>&1 | head -20
fi

# Add toolchain to PATH (find the actual installation)
TOOLCHAIN_PATH=$(find "$HOME/.espressif/tools/xtensa-esp32s3-elf" -name "xtensa-esp32s3-elf-gcc" -type f 2>/dev/null | head -1 | xargs dirname)
if [ -n "$TOOLCHAIN_PATH" ]; then
    export PATH="$TOOLCHAIN_PATH:$PATH"
else
    export PATH="$HOME/.espressif/tools/xtensa-esp32s3-elf/esp-13.2.0_20230426/xtensa-esp32s3-elf/bin:$PATH"
fi

# Verify linker
if ! command -v xtensa-esp32s3-elf-gcc &> /dev/null; then
    echo "⚠️  Linker not in PATH. Checking espressif installation..."
    find "$HOME/.espressif" -name "xtensa-esp32s3-elf-gcc" 2>/dev/null | head -1 || echo "Linker not found"
fi
echo "✓ Toolchain ready"
echo ""

# Step 4: Build
echo "Step 4: Building binary (this takes 5-10 minutes)..."
cd /Users/vishy/Downloads/sma_dersim_client
cargo clean
cargo +esp build --release --target xtensa-esp32s3-espidf -Z build-std=core,alloc,std,panic_abort

BINARY="target/xtensa-esp32s3-espidf/release/sma_test"
if [ ! -f "$BINARY" ]; then
    echo "❌ Build failed - binary not found"
    exit 1
fi

SIZE=$(ls -lh "$BINARY" | awk '{print $5}')
echo "✓ Binary built: $SIZE"
echo ""

# Step 5: Verify board is connected
echo "Step 5: Verifying ESP32-S3 board connection..."
if [ ! -e "/dev/cu.usbmodem2101" ]; then
    echo "⚠️  Board not found at /dev/cu.usbmodem2101"
    echo "Available ports:"
    ls /dev/cu.usbmodem* 2>/dev/null || echo "(none found)"
    echo ""
    read -p "Enter port (e.g. /dev/cu.usbmodem2101): " PORT
else
    PORT="/dev/cu.usbmodem2101"
fi
echo "✓ Using port: $PORT"
echo ""

# Step 6: Flash
echo "Step 6: Flashing to board..."
espflash flash "$BINARY" --port "$PORT"
echo "✓ Flash complete!"
echo ""

# Step 7: Monitor
echo "Step 7: Monitoring serial output (Ctrl+C to exit)..."
echo "════════════════════════════════════════════════════════════════"
espflash monitor --port "$PORT" --baud 115200

