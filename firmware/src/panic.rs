//! Implements the panic handler

use core::hint;
use core::panic::PanicInfo;
use cortex_m_rt::ExceptionFrame;

/// The panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Disable all interrupts
    cortex_m::interrupt::disable();
    if cfg!(debug_assertions) {
        // Print via semihosting
        // Note: This will crash if no debugger is attached
        cortex_m_semihosting::hprintln!("{}", info);
    }

    // Crash and wait forever or until a watchdog kills us
    cortex_m::asm::bkpt();
    cortex_m::asm::udf();
}

#[cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn DefaultHandler(irqn: i16) {
    loop {
        cortex_m::asm::bkpt();
        hint::black_box(irqn);
    }
}

#[cortex_m_rt::exception]
#[allow(non_snake_case)]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    loop {
        cortex_m::asm::bkpt();
        hint::black_box(ef);
    }
}
