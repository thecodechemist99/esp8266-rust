#![no_std]
#![no_main]

use panic_halt as _;
use esp8266_hal::{ prelude::*, target::Peripherals };

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _) = peripherals.TIMER.timers();

    led.set_high().unwrap();

    loop {
        timer1.delay_ms(500);
        led.toggle().unwrap();
    }
}
