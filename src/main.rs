#![no_std]
#![no_main]

use cortex_m_rt::entry;
#[allow(unused)]
use panic_halt;
// use nrf52832_hal::prelude;

#[entry]
fn main() -> ! {
    cortex_m::asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {}
}
