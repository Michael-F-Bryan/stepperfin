#![no_std]
#![no_main]

#[cfg(not(test))]
extern crate panic_halt;

#[cfg(test)]
#[macro_use]
extern crate std;

use cortex_m_rt::entry;
use f3::led::Leds;
use stm32f30x_hal::prelude::*;
use stm32f30x_hal::stm32f30x;
use stm32f30x_hal::delay::Delay;

#[entry]
fn main() -> ! {
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let device_peripherals = stm32f30x::Peripherals::take().unwrap();

    let mut flash = device_peripherals.FLASH.constrain();
    let mut rcc = device_peripherals.RCC.constrain();
    let gpioe = device_peripherals.GPIOE.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    loop { 
        for i in 0..leds.len() {
            leds[i].off();
            let next = (i+1) % leds.len();
            leds[next].on();
            delay.delay_ms(100_u8);
        }
    }
}