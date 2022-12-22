# Basic Template for ESP8266 projects in rust
This package is based on the current versions of the included dependencies (as of 2022-12-22).

## Dependency versions
| Dependency    | Version   | Notes                                                                                           |
| ------------- | --------- | ----------------------------------------------------------------------------------------------- |
| esp8266-hal   | `v0.5.1`  |                                                                                                 |
| esp32-hal     | `v0.7.0`  |                                                                                                 |
| esp-backtrace | `v0.4.0`  | not currently used, but included for future ESP32 support                                       |
| esp-println   | `v0.3.1`  |                                                                                                 |
| xtensa-lx-rt  | `v0.14.0` |                                                                                                 |
| panic-halt    | `v0.2.0`  | currently necessary as esp-backtrace doesn't support ESP8266, the latter is preferred for ESP32 |

## Toolchain Setup
The toolchain files and setup instructions from Espressif can be found [here](https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/linux-setup.html).

## Running
To test your setup and later compile and flash your code, run `cargo espflash --release --monitor`.

## TODO
- ESP32 config included from sources but not checked yet.

## Sources
This project is based upon:
- https://github.com/esp-rs/esp8266-hal
- https://github.com/esp-rs/esp-hal
- https://github.com/MabezDev/xtensa-rust-quickstart
- https://github.com/esp-rs/esp-template
- https://github.com/esp-rs/xtensa-lx-rt

For additional information, have a look into [The Rust on ESP Book](https://esp-rs.github.io/book/).
