#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    let digital_out = pac::Peripherals::take().unwrap();

    // Configure the clock.
    let mut clock = digital_out.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = digital_out.GPIOA.split(&mut clock);

    // Configure PA1 as output.
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        for _ in 0..1_000 {
            led.set_high().unwrap();
        }
        for _ in 0..1_000 {
            led.set_low().unwrap();
        }
    }
}