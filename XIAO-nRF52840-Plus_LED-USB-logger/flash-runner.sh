#!/bin/bash
set -e

# This script is used as a cargo runner to automatically flash the nRF52840
# Usage: This gets called automatically by cargo when you run `cargo run` (configured in .cargo/config.toml)

BINARY_PATH="$1"
COM_PORT="${NRF_PORT:-/dev/cu.usbmodem1101}"

# Extract the binary name for naming the hex and zip files
BINARY_NAME=$(basename "$BINARY_PATH")
TARGET_DIR=$(dirname "$BINARY_PATH")

echo "🔨 Converting $BINARY_NAME to Intel HEX format..."
arm-none-eabi-objcopy -O ihex "$BINARY_PATH" "$TARGET_DIR/$BINARY_NAME.hex"

echo "📦 Creating DFU package..."
adafruit-nrfutil dfu genpkg --dev-type 0x0052 --sd-req 0x0123 --application "$TARGET_DIR/$BINARY_NAME.hex" "$TARGET_DIR/$BINARY_NAME.zip"

echo "⚡ Flashing to device on $COM_PORT..."

# Capture both stdout and stderr
FLASH_OUTPUT=$(adafruit-nrfutil --verbose dfu serial -pkg "$TARGET_DIR/$BINARY_NAME.zip" -p "$COM_PORT" -b 115200 --singlebank 2>&1)
echo "$FLASH_OUTPUT"

# Check if the output contains failure indicators
if echo "$FLASH_OUTPUT" | grep -q "Failed to upgrade target\|could not open port\|No such file or directory\|SerialException"; then
    echo ""
    echo "❌ Failed to flash $BINARY_NAME!"
    echo ""
    echo "Troubleshooting tips:"
    echo "1. Check if device is connected: ls /dev/cu.usbmodem*"
    echo "2. Put device in DFU mode: Hold RESET + DFU buttons, release RESET first, then DFU"
    echo "3. Set correct port: NRF_PORT=/dev/cu.usbmodemXXXX cargo run --bin hello_world"
    exit 1
else
    echo "✅ Successfully flashed $BINARY_NAME!"
fi 