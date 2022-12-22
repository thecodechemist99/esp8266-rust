#![no_std]
#![no_main]

{% if mcu == "esp32" -%}
use esp32_hal::{clock::ClockControl, pac::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc, IO};
use esp_backtrace as _;
{% else -%}
use esp8266_hal::{ prelude::*, target::Peripherals };
use panic_halt as _;
{% endif %}

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    {% if mcu == "esp32" -%}
    let system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let pins = IO::new(peripherals.GPIO, peripherals.IO_MUX).pins;
    let mut delay = Delay::new(&clocks);

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    {% else -%}
    let pins = peripherals.GPIO.split();
    {% endif %}

    let mut led = pins.gpio2.into_push_pull_output();
    led.set_high().unwrap();

    {% if mcu == "esp8266" -%}
    let (mut timer1, _) = peripherals.TIMER.timers();
    {% endif %}

    loop {
        led.toggle().unwrap();
        {% if mcu == "esp32" -%}
        delay.delay_ms(500u32);
        {% else -%}
        timer1.delay_ms(500);
        {% endif %}
    }
}
