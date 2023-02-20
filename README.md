# Basic Template for ESP8266 projects in rust
This package is based on the current versions of the included dependencies (as of 2023-02-20).

## Dependency versions
| Dependency    | Version   | Notes                                                                                           |
| ------------- | --------- | ----------------------------------------------------------------------------------------------- |
| esp8266-hal   | `v0.5.1`  |                                                                                                 |
| esp32-hal     | `v0.8.0`  |                                                                                                 |
| esp-backtrace | `v0.5.0`  | only used for ESP32, no ESP8266 support                                                         |
| esp-println   | `v0.3.1`  |                                                                                                 |
| xtensa-lx-rt  | `v0.15.0` |                                                                                                 |
| panic-halt    | `v0.2.0`  | currently necessary as esp-backtrace doesn't support ESP8266, the latter is preferred for ESP32 |

## Toolchain Setup
The toolchain files and setup instructions from Espressif can be found [here](https://docs.espressif.com/projects/esp8266-rtos-sdk/en/latest/get-started/linux-setup.html).

## Create your Project
You can create a project from this template with `cargo-generate`:
```
# if not yet installed
cargo install cargo-generate

# create from template
cargo generate --git https://github.com/thecodechemist99/esp8266-rust
```

To test your setup and later compile and flash your code run:
```
cargo espflash --release --monitor /dev/<USB device>
```
You need to have `cargo-espflash` installed for this to work.

## TODO
- ESP32 config not checked on physical device yet (works in simulator)

## Sources
This project is based upon:
- https://github.com/esp-rs/esp8266-hal
- https://github.com/esp-rs/esp-hal
- https://github.com/MabezDev/xtensa-rust-quickstart
- https://github.com/esp-rs/esp-template
- https://github.com/esp-rs/xtensa-lx-rt

For additional information, have a look into [The Rust on ESP Book](https://esp-rs.github.io/book/).
