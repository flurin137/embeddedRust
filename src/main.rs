#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32l0xx_hal::{pac, prelude::*, pwm, rcc::Config};

#[entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

    let mut clock = board_peripherals.RCC.freeze(Config::hsi16());

    let mut delay = cortex_peripherals.SYST.delay(clock.clocks);

    let gpioa = board_peripherals.GPIOA.split(&mut clock);
    let gpiob = board_peripherals.GPIOB.split(&mut clock);
    let gpioc = board_peripherals.GPIOC.split(&mut clock);

    let button = gpioc.pc13.into_pull_up_input();

    // Initialize TIM2 for PWM
    let timer2 = pwm::Timer::new(board_peripherals.TIM2, 10.khz(), &mut clock);

    let mut pwm1 = timer2.channel2.assign(gpiob.pb3);
    let mut pwm2 = timer2.channel3.assign(gpiob.pb10);

    loop {
        let limit : u16 = match button.is_high() {
            Ok(true) => 4,
            Ok(false) => 1,
            _ => 1,
        };
        
        let max1 = pwm1.get_max_duty() / limit;

        pwm1.enable();
        pwm2.enable();
        
        loop{
            for i in 0..max1 {
                pwm1.set_duty(i);
                pwm2.set_duty(max1-i);
                delay.delay_ms(5_u16);
            }
            for i in 0..max1 {
                pwm2.set_duty(i);
                pwm1.set_duty(max1-i);
                delay.delay_ms(5_u16);
            }
        }
    }
}