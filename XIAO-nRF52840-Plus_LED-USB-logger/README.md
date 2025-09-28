# USB logger on XIAO nRF53840 Sense Plus with Embassy

Project based on [XIAO-nRF52840-Plus_LED-blink](XIAO-nRF52840-Plus_LED-blink) to log messages over USB serial connection. See [project folder](XIAO-nRF52840-Plus_LED-blink) for basic setup details.

The temperature from the chip sensor is logged and send via USB serial connections every 10 seconds.

You can see the logs with `screen` or using `cat` in tmux.

```sh
screen /dev/cu.usbmodem1101
```

or

```sh
# in tmux
cat /dev/cu.usbmodem1101
```

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

The blue LED should blink every 10 seconds, and the die temperature of the chip is sent to the usb logger.

4. Connect to the USB serial port

```sh
screen /dev/cu.usbmodem1101
```

or

```sh
# in tmux
cat /dev/cu.usbmodem1101
```

Replace the port with the one where the board is plugged on your computer.
