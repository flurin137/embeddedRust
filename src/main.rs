#![no_main]
#![no_std]

extern crate panic_halt;

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    gpio::*,
    pac::{self, interrupt, Interrupt},
    prelude::*,
    rcc::Config,
    syscfg::SYSCFG,
};

static LED: Mutex<RefCell<Option<gpiob::PB6<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());

    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure PB6 as output.
    let led = gpiob.pb6.into_push_pull_output();

    // Configure PB2 as input.
    let button = gpiob.pb2.into_pull_up_input();

    let mut syscfg = SYSCFG::new(dp.SYSCFG, &mut rcc);
    let mut exti = Exti::new(dp.EXTI);

    let line = GpioLine::from_raw_line(button.pin_number()).unwrap();
    exti.listen_gpio(&mut syscfg, button.port(), line, TriggerEdge::Falling);

    cortex_m::interrupt::free(|cs| {
        *LED.borrow(cs).borrow_mut() = Some(led);
    });

    unsafe {
        NVIC::unmask(Interrupt::EXTI2_3);
    }

    loop {
        asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    static mut STATE: bool = false;

    cortex_m::interrupt::free(|cs| {
        Exti::unpend(GpioLine::from_raw_line(2).unwrap());

        if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
            if *STATE {
                led.set_low().unwrap();
                *STATE = false;
            } else {
                led.set_high().unwrap();
                *STATE = true;
            }
        }
    });
}