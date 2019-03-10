#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f30x_hal;

use cortex_m_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    // initialization

    loop {
        // application logic
    }
}