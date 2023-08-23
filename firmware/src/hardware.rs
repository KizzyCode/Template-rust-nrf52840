//! Provides access handles for hardware/peripheral access

use nrf52840_hal::{
    gpio::{
        p0::{Parts, P0_06},
        Level, Output, PushPull,
    },
    pac::{CorePeripherals, Peripherals},
    Delay,
};

/// A hardware handle
pub struct Hardware {
    /// BSP replacement for the HAL [`Pins`](rp2040_hal::gpio::Pins) type
    pub led: P0_06<Output<PushPull>>,
    /// System timer (SysTick) as a delay provider
    pub delay: Delay,
}
impl Hardware {
    /// Initializes the required hardware
    pub fn init() -> Option<Self> {
        // Destructure peripherals
        let Peripherals { P0, .. } = Peripherals::take()?;
        let CorePeripherals { SYST, .. } = CorePeripherals::take()?;

        // Get GPIO
        let port0 = Parts::new(P0);
        let led = port0.p0_06.into_push_pull_output(Level::Low);

        // Setup delay
        let delay = Delay::new(SYST);
        Some(Self { led, delay })
    }
}
