#!/bin/bash
# Quick ESP32 Setup for macOS

set -e

# Load Rust environment
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

echo "🚀 ESP32 Modbus Client Setup"
echo "════════════════════════════════════════════════════════════════"
echo ""

# Check if we have Rust
if ! command -v rustup &> /dev/null; then
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust found"

# Install ESP32 tools
echo ""
echo "📦 Installing ESP32 toolchain..."

if ! cargo install --list | grep -q "espup"; then
    echo "  Installing espup..."
    cargo install espup
fi

if ! cargo install --list | grep -q "espflash"; then
    echo "  Installing espflash..."
    cargo install espflash
fi

# Install target
echo ""
echo "🎯 Installing xtensa-esp32-espidf target..."
rustup target add xtensa-esp32-espidf 2>/dev/null || true

# Check for USB device
echo ""
echo "🔍 Looking for ESP32 device..."
PORT=$(ls /dev/cu.usb* 2>/dev/null | head -1 || ls /dev/cu.SLAB* 2>/dev/null | head -1 || echo "")

if [ -z "$PORT" ]; then
    echo "⚠️  No USB device found."
    echo "   Please connect your ESP32 and try again."
    exit 1
fi

echo "✓ Found device at: $PORT"

# Build
echo ""
echo "🔨 Building for ESP32 (this takes 2-5 minutes)..."
cd "$(dirname "$0")"
cargo +esp build --release --target xtensa-esp32-espidf

echo ""
echo "✅ Build complete!"
echo ""
echo "════════════════════════════════════════════════════════════════"
echo "📝 Next steps:"
echo ""
echo "  1. Flash to device:"
echo "     cargo +esp run --release --target xtensa-esp32-espidf --monitor"
echo ""
echo "  2. Or use espflash directly:"
echo "     espflash flash target/xtensa-esp32-espidf/release/sma_test --monitor"
echo ""
echo "  3. Watch for output (should connect to WiFi then DERSim)"
echo ""
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "💡 For full troubleshooting, see ESP32_SETUP.md"
echo ""
