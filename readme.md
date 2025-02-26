
# TM1637
This is driver for TM1637 chip written in Rust programming language. This driver was developed using ESP32C3 micro controller. This chip is used to control four digit seven segment display. This is supposed to work on any microcontroller but this has not been tested yet.

### Datasheet
Link to the datasheet:
> https://www.makerguides.com/wp-content/uploads/2019/08/TM1637-Datasheet.pdf


## Build instructions

### ESP32-based boards
- cargo build --release --features esp32
- cargo build --release --features esp32s2
- cargo build --release --features esp32s3
- cargo build --release --features esp32c3

### Raspberry Pi
- cargo build --release --features rp-pico  
- cargo build --release --features rpi 

## Supported platforms

| Platform           | Status      |  
|--------------------|------------|  
| ESP32C3           | ✅ Tested  |  
| ESP32 / ESP32S2 / ESP32S3 | ⚠ Not tested |  
| Raspberry Pi Pico | ⚠ Not tested |  
| Raspberry Pi 4/5  | ⚠ Not tested |