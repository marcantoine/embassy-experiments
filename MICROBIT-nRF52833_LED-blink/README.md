# First LED blink on MICRBOBIT v2.2 nRF52833 with Embassy

Very basic implementation to blink the first LED of the micro:bit v2.2 board using [Embassy](https://embassy.dev/).

Everything is in the main task, not really async or using embassy capacity.

A lot of issue to make this project flash on the microbit board due to dependencies configuration (see [./.cargo/config.toml](./.cargo/config.toml)).

Use embassy-nrf to access the GPIOs.
Use embassy-time to manage the delays to blink the LED.
Use defmt for logging with time.
