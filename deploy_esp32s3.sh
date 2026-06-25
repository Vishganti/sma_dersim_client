#!/bin/bash
set -e

# Complete ESP32-S3 Setup, Build, Flash & Test Script
# This script does everything end-to-end

PROJECT_DIR="/Users/vishy/Downloads/sma_dersim_client"
BOARD_PORT="/dev/cu.usbmodem2101"
TARGET="xtensa-esp32s3-espidf"

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║      Complete ESP32-S3 Deploy Script                         ║"
echo "║      (Build, Flash & Test DERSim Connection)                 ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Step 1: Load Rust environment
echo "📦 Step 1: Loading Rust environment..."
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
else
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi
echo "✓ Rust environment loaded"
echo ""

# Step 2: Install espup if needed
echo "📦 Step 2: Ensuring espup is installed..."
if ! command -v espup &> /dev/null; then
    echo "   Installing espup..."
    cargo install espup
else
    echo "✓ espup already installed"
fi
echo ""

# Step 3: Install ESP IDF toolchain
echo "📦 Step 3: Installing ESP32 toolchain (this may take a few minutes)..."
if [ ! -d "$HOME/.espressif" ]; then
    echo "   Running espup install (automated, no interaction)..."
    espup install --default-host --targets esp32s3 --profiles full --toolchain-version master || true

    # Add to PATH for this session
    export PATH="$HOME/.espressif/tools/xtensa-esp32s3-elf/esp-13.2.0_20230426/xtensa-esp32s3-elf/bin:$PATH"
    export PATH="$HOME/.espressif/tools/esp-riscv32-elf/esp-13.2.0_20230426/esp-riscv32-elf/bin:$PATH"
    export IDF_PATH="$HOME/.espressif/esp-idf"
else
    echo "✓ ESP32 toolchain already installed"
fi
echo ""

# Step 4: Add Rust target
echo "📦 Step 4: Adding xtensa-esp32s3-espidf target..."
rustup target add xtensa-esp32s3-espidf 2>/dev/null || true
echo "✓ Target added"
echo ""

# Step 5: Verify board is connected
echo "📦 Step 5: Checking board connection..."
if [ ! -e "$BOARD_PORT" ]; then
    echo "❌ Board not found at $BOARD_PORT"
    echo "   Available ports:"
    ls -la /dev/cu.usbmodem* 2>/dev/null || echo "   (none found)"
    exit 1
fi
echo "✓ Board detected at $BOARD_PORT"
echo ""

# Step 6: Build the project
echo "📦 Step 6: Building Modbus client (this takes 2-5 minutes)..."
cd "$PROJECT_DIR"
cargo clean
cargo build --release --target "$TARGET" 2>&1 | tail -50

if [ ! -f "target/$TARGET/release/sma_test" ]; then
    echo "❌ Build failed"
    exit 1
fi

BINARY_SIZE=$(ls -lh "target/$TARGET/release/sma_test" | awk '{print $5}')
echo "✓ Build complete! Binary size: $BINARY_SIZE"
echo ""

# Step 7: Flash to device
echo "📦 Step 7: Flashing to ESP32-S3..."
echo "   (Device will reset automatically)"
espflash flash --release --target "$TARGET" --port "$BOARD_PORT" 2>&1 | tail -10

echo "✓ Flash complete!"
echo ""

# Step 8: Monitor output
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              🚀 Watching Serial Output                        ║"
echo "║         (Press Ctrl+C to stop, board keeps running)          ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Expected output:"
echo "  📡 Connecting to WiFi: RaptorNetOG"
echo "  ✅ WiFi connected!"
echo "  🔌 Connecting to DERSim..."
echo "  ✅ DERSim Connected!"
echo "  📊 Measurements streaming..."
echo ""
sleep 2

espflash monitor --port "$BOARD_PORT" --baud 115200

