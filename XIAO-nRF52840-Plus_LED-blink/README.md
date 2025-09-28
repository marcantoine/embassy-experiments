# Green LED blink on XIAO nRF52840 Sense Plus with Embassy

Minimal project to blink the green LED on Seed Studio XIAO nRF52840 Sense Plus board using [Embassy](https://embassy.dev/).

The goal of this project is to have a minimal example using a custom runner script with `adafruit-nrfutil` to flash the board.

## Installation

1. Create a Python virtual environment and install `adafruit-nrfutil`

```sh
python -m venv .venv
source .venv/bin/activate
pip install adafruit-nrfutil
```

2. Modify the `flash-runner.sh` script to set the correct port where your board is connected:

```sh
COM_PORT="${NRF_PORT:-/dev/cu.usbmodem1101}"
```

You can find the port on mac OS with `ls /dev/cu.usb*` when the board is connected. The XIAO nRF52840 Sense Plus is only detected when you double click the reset button on the board.

3. Build and flash the project

Double-click on the reset button to make the board detectable, then run:

```sh
cargo run
```

The green LED should blink every second.

4. Test to reflash the project with a different LED.

Blue User LED pin: P0_06
Red User LED pin: P0_26
Green User LED pin: P0_30

## How was the project setup

1. Init embassy project

```sh
cargo embassy init {project_name} --chip nRF52840
```

2. Edit memory.x
   Based on [Seeed-nRF52840-Sense-projects](https://github.com/Wumpf/Seeed-nRF52840-Sense-projects/blob/main/memory.x)

The nRF52840 chip needs to reserve some space for the SoftDevice (Nordic proprietary Bluetooth stack).

```
MEMORY
{
  /* Need to leave space for the SoftDevice
    These values are confirmed working for S140 7.3.0
    They were extracted from the Arduino IDE plugin linked indirectly at https://wiki.seeedstudio.com/XIAO_BLE/
  */
  FLASH (rx)     : ORIGIN = 0x27000, LENGTH = 0xED000 - 0x27000

  /* SRAM required by Softdevice depend on
   * - Attribute Table Size (Number of Services and Characteristics)
   * - Vendor UUID count
   * - Max ATT MTU
   * - Concurrent connection peripheral + central + secure links
   * - Event Len, HVN queue, Write CMD queue
   */
  RAM (rwx) :  ORIGIN = 0x20006000, LENGTH = 0x20040000 - 0x20006000
}
```

3. Create `flash-runner.sh` file

XIAO nRF52840 Sense Plus is not detected as a probe by probe-rs.

We use a custom script `flash-runner.sh` which use `adafruit-nrfutil` to flash the binary to the board.

Configure the port where your board is connected:

```sh
COM_PORT="${NRF_PORT:-/dev/cu.usbmodem1101}"
```

4. Edit `.cargo/config.toml`

Replace the original probe-rs runner line with the script:

```toml
runner = "./flash-runner.sh"
```

5. Create a Python virtual environment and install `adafruit-nrfutil`

```sh
python -m venv .venv
source .venv/bin/activate
pip install adafruit-nrfutil
```

6. Build and flash the project

```sh
cargo run
```
