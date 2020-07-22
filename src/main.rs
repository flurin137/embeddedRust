#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    let digital_out = pac::Peripherals::take().unwrap();

    let mut clock = digital_out.RCC.freeze(Config::hsi16());

    let gpioa = digital_out.GPIOA.split(&mut clock);
    let gpioc = digital_out.GPIOC.split(&mut clock);

    let mut led = gpioa.pa5.into_push_pull_output();
    let button = gpioc.pc13.into_pull_up_input();

    loop {
        let limit = match button.is_high() {
            Ok(true) => 1_000_000,
            Ok(false) => 300_000,
            _ => unreachable!(),
        };

        for _ in 0..limit {
            led.set_high().unwrap();
        }
        for _ in 0..limit {
            led.set_low().unwrap();
        }
    }
}