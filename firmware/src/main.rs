#![no_std]
#![no_main]

mod hardware;
mod panic;

use crate::hardware::Hardware;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

#[cortex_m_rt::entry]
fn main() -> ! {
    // Grab our hardware objects
    // Note: The LED pin is P0.06 which is the green LED for the NRF58240 dongle – depending on your hardware of choice,
    // you may need to initialize a different pin in `Hardware::init()`
    let Hardware { mut led, mut delay } = Hardware::init().expect("failed to initialize hardware");

    // Blink the LED at 1 Hz
    loop {
        led.set_high().expect("failed to set LED pin state");
        delay.delay_ms(500);
        led.set_low().expect("failed to set LED pin state");
        delay.delay_ms(500);
    }
}
