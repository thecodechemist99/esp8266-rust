#![no_std]
#![no_main]

use panic_halt as _;
{% if mcu == "esp32" -%}
use esp32_hal::target;
use hal::prelude::*;
use xtensa_lx::timer::delay;
use esp32_hal as hal;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
const CORE_HZ: u32 = 40_000_000;

const WDT_WKEY_VALUE: u32 = 0x50D83AA1;
{% else -%}
use esp8266_hal::{ prelude::*, target::Peripherals };
{% endif %}

#[xtensa_lx_rt::entry]
fn main() -> ! {
    {% if mcu == "esp32" -%}
    let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    let mut rtccntl = dp.RTCCNTL;
    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    // (https://github.com/espressif/openocd-esp32/blob/97ba3a6bb9eaa898d91df923bbedddfeaaaf28c9/src/target/esp32.c#L431)
    // openocd disables the wdt's on halt
    // we will do it manually on startup
    disable_timg_wdts(&mut timg0, &mut timg1);
    disable_rtc_wdt(&mut rtccntl);    
    {% else -%}
    let dp = Peripherals::take().unwrap();
    {% endif %}

    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_push_pull_output();

    {% if mcu == "esp8266" -%}
    let (mut timer1, _) = dp.TIMER.timers();
    {% endif %}

    loop {
        led.set_high().unwrap();
        {% if mcu == "esp32" -%}
        delay(CORE_HZ);
        {% else -%}
        timer1.delay_ms(500);
        {% endif %}
        led.set_low().unwrap();
        {% if mcu == "esp32" -%}
        delay(CORE_HZ);
        {% else -%}
        timer1.delay_ms(500);
        {% endif %}
    }
}

{% if mcu == "esp32" -%}
fn disable_rtc_wdt(rtccntl: &mut target::RTCCNTL) {
    /* Disables the RTCWDT */
    rtccntl
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w.wdt_stg0()
            .bits(0x0)
            .wdt_stg1()
            .bits(0x0)
            .wdt_stg2()
            .bits(0x0)
            .wdt_stg3()
            .bits(0x0)
            .wdt_flashboot_mod_en()
            .clear_bit()
            .wdt_en()
            .clear_bit()
    });
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(0x0) });
}

fn disable_timg_wdts(timg0: &mut target::TIMG0, timg1: &mut target::TIMG1) {
    timg0
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    timg1
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });

    timg0.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
    timg1.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
}
{% endif %}
