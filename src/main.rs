#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f0x1::{rcc, gpioc};

pub fn delay_ms(x: u32) {
    for _ in 0..x {
        continue
    }
}

#[entry]
fn main() -> ! {
    unsafe { (*rcc()).ahbenr.write((*rcc()).ahbenr.read() | (1 << 19)) }                    // IOPCEN - enable clock on GPIOC
    unsafe { (*gpioc()).moder.write((*gpioc()).moder.read() | (1 << 16) | (1 << 18)) }      // MODER8_0 | MODER9_0 = output mode

    let mut on = true;

    loop {
        delay_ms(1000);
        if on {
            unsafe { (*gpioc()).bsrr.write(0x0300) }
        } else {
            unsafe { (*gpioc()).brr.write(0x0300) }
        }
        on = !on;
    }
}

