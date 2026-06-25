# LED Status Indicator Configuration

## Overview

The ESP32 Modbus client now has LED status feedback to show connection state:

| Status | LED Behavior | Meaning |
|--------|--------------|---------|
| 🔴 OFF | Solid off | Device not powered or initialization |
| 🟡 BLINKING | Rapid blink | Connecting to WiFi or DERSim |
| 🟢 ON | Solid on | Fully connected, monitoring active |
| 🔴 ERROR | Rapid blink pattern | WiFi/network error |

## Configuring the LED Pin

The LED GPIO pin is defined at the top of `src/main.rs`:

```rust
const LED_PIN: i32 = 2;  // ← Change this to your pin
```

### Default Pins by Board

| Board | LED Pin | Notes |
|-------|---------|-------|
| **ESP32** | 2 or 5 | Most common |
| **ESP32-S2** | 13 or 15 | Check your board |
| **ESP32-S3** | 48 (RGB) or 13 | RGB LED usually on 48/47/46 |
| **ESP32-C3** | 3 or 8 | Smaller board |
| **Generic Dev Board** | 2 (blue) or 5 (red) | Check silkscreen |

### Finding Your LED Pin

**Method 1: Check the silkscreen on your board**
- Look for "LED" label
- Trace the trace to find GPIO number

**Method 2: Check board documentation**
- Common patterns:
  - Blue LED: GPIO2, GPIO5
  - Red LED: GPIO5, GPIO12
  - RGB LED: GPIO47/48 (S3), GPIO8 (C3)

**Method 3: Try common pins**
```rust
// Test pins in order:
const LED_PIN: i32 = 2;   // Try this first
// If doesn't work, try 5, 13, 47, 48, etc.
```

## How to Configure

### Step 1: Identify your LED pin

Connect to a computer, check your board, or look up the datasheet.

### Step 2: Update the constant

Edit `src/main.rs` line 20:

```rust
// Before (default):
const LED_PIN: i32 = 2;

// After (example for ESP32-S3):
const LED_PIN: i32 = 48;
```

### Step 3: Rebuild and flash

```bash
cargo +esp run --release --target xtensa-esp32-espidf --monitor
```

### Step 4: Verify

You should see LED behavior:

```
📡 Connecting to WiFi: RaptorNetOG (LED blinking...)
✅ WiFi connected!

💡 LED is now solid (fully connected)

🔌 Connecting to DERSim at 192.168.1.252:8503...
✅ Connected!

💡 LED is now solid (fully connected)
```

## LED Behavior Details

### Boot to WiFi Connection
```
[Boot] LED: OFF
  ↓
[Initializing WiFi] LED: BLINKING (slow)
  ↓
[WiFi Connected] LED: Blink x3 (confirmation)
  ↓
[Connecting to DERSim] LED: BLINKING
  ↓
[DERSim Connected] LED: SOLID ON ✓
```

### Continuous Operation
```
[Monitoring] LED: SOLID ON
  ↓
[Every 5 seconds] Read measurements, LED stays ON
  ↓
[Connection lost] LED: FLASH once, then BLINKING (retry)
  ↓
[Reconnected] LED: SOLID ON ✓
```

### Error States
```
[WiFi Failed] LED: BLINKING x5 (error pattern)
[DERSim Failed] LED: BLINKING x5 (error pattern)
[Timeout] LED: OFF
```

## Multiple LEDs (Advanced)

If your board has multiple LEDs (e.g., RGB):

```rust
// Add multiple LED pins
const LED_PIN_POWER: i32 = 2;    // Power status
const LED_PIN_WIFI: i32 = 5;     // WiFi status
const LED_PIN_DATA: i32 = 13;    // Data status
```

Then update the `LedController` to handle multiple pins:

```rust
struct LedController {
    pins: Vec<PinDriver<'static, AnyIOPin, Output>>,
    // ... rest of impl
}
```

## Power Consumption

LED power usage:
- **LED ON**: ~5-10mA (depends on brightness)
- **LED BLINKING**: ~2-5mA average
- **LED OFF**: <1mA

For battery-powered devices, you can modify the LED behavior or disable it:

```rust
// Disable LED in main():
// let led = LedController::new(None);  // No LED control

// Or use dimming (PWM):
// Instead of set_high/set_low, use PWM for dimmer control
```

## Troubleshooting

### LED doesn't light up
1. Check pin number matches your board
2. Check LED polarity (cathode/anode)
3. Check for solder bridges or bad connections
4. Try different pin number

### LED always on
1. Pin might be inverted (active low)
2. Edit code to swap `set_high()` ↔ `set_low()`

### LED won't blink
1. Check refresh rate (Duration in code)
2. Check if pin supports GPIO output
3. Try GPIO2 or GPIO5 (most compatible)

### Pin already in use
1. Another part of code might be using that GPIO
2. Check esp-idf initialization
3. Try a different pin

## Modify Blink Patterns

To change blink speed/pattern, edit the `blink()` calls in `src/main.rs`:

```rust
// Current: 3 blinks, 100ms each
led.blink(3, 100);

// Faster: 5 blinks, 50ms each
led.blink(5, 50);

// Slower: 2 blinks, 200ms each
led.blink(2, 200);
```

## External LED

If your board doesn't have a built-in LED, you can attach an external one:

```
ESP32 GPIO2 ----[220Ω resistor]----[LED anode]
                                     LED cathode ---- GND
```

Then use GPIO2 (or your chosen pin) as normal.

## RGB LED Support (Future)

For RGB LEDs on ESP32-S3:

```rust
const LED_PIN_R: i32 = 48;  // Red
const LED_PIN_G: i32 = 47;  // Green
const LED_PIN_B: i32 = 46;  // Blue

// Red: Error
// Yellow (R+G): Connecting
// Green: Connected
```

This would require updating the `LedController` to handle 3 pins.

---

## Quick Checklist

- [ ] Identified correct LED GPIO pin for your board
- [ ] Updated `const LED_PIN` in `src/main.rs`
- [ ] Rebuilt and flashed to device
- [ ] Verified LED lights up when connected
- [ ] Verified LED blinks during boot
- [ ] Device is monitoring DERSim (LED solid on)

✅ You're all set!
