#![no_std]
#![no_main]
#[allow(unused)]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// use panic_halt;
use nrf52832_pac;
use nrf52832_hal::prelude;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {
        continue;
    }
}