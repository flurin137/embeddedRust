#![no_main]
#![no_std]

extern crate panic_halt;

use core::cell::Cell;
use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::asm;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32l0xx_hal::{
    gpio::*,
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    pac::{self, interrupt, Interrupt},
    prelude::*,
    rcc::Config,
    syscfg::SYSCFG,
    timer::Timer,
    pwm,
};

static TIMER: Mutex<RefCell<Option<Timer<pac::TIM3>>>> = Mutex::new(RefCell::new(None));
static PWM: Mutex<RefCell<Option<pwm::Pwm<pac::TIM2, pwm::C2, pwm::Assigned<gpiob::PB3<Analog>>>>>> = Mutex::new(RefCell::new(None));

static SPEED_COUNTER: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
static TIME_COUNTER: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));
static MAX: Mutex<Cell<u16>> = Mutex::new(Cell::new(0));

#[entry]
fn main() -> ! {
    let board_peripherals = pac::Peripherals::take().unwrap();

    let mut clock = board_peripherals.RCC.freeze(Config::hsi16());

    let gpiob = board_peripherals.GPIOB.split(&mut clock);

    let button = gpiob.pb2.into_pull_up_input();

    let mut syscfg = SYSCFG::new(board_peripherals.SYSCFG, &mut clock);
    let mut exti = Exti::new(board_peripherals.EXTI);

    let line = GpioLine::from_raw_line(button.pin_number()).unwrap();
    exti.listen_gpio(&mut syscfg, button.port(), line, TriggerEdge::Falling);
    
    let mut timer = board_peripherals.TIM3.timer(1.hz(), &mut clock);
    
    let pwm_timer = pwm::Timer::new(board_peripherals.TIM2, 10.khz(), &mut clock);
    let mut pwm = pwm_timer.channel2.assign(gpiob.pb3);
    
    timer.listen();
    pwm.enable();
    
    let max = pwm.get_max_duty();

    cortex_m::interrupt::free(|cs| {
        *PWM.borrow(cs).borrow_mut() = Some(pwm);
        *TIMER.borrow(cs).borrow_mut() = Some(timer);

        MAX.borrow(cs).set(max);
    });

    unsafe {
        NVIC::unmask(Interrupt::EXTI2_3);
        NVIC::unmask(Interrupt::TIM3);
    }

    loop {
        asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    cortex_m::interrupt::free(|cs| {
        Exti::unpend(GpioLine::from_raw_line(2).unwrap());
        SPEED_COUNTER.borrow(cs).set(SPEED_COUNTER.borrow(cs).get() + 1);
    });
}

#[interrupt]
fn TIM3() {
    cortex_m::interrupt::free(|cs| {
        let time = TIME_COUNTER.borrow(cs).get();
        let value = SPEED_COUNTER.borrow(cs).get() + 1;
        let max = MAX.borrow(cs).get();

        TIME_COUNTER.borrow(cs).set(time + 1);

        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            timer.clear_irq();
        }

        if let Some(ref mut pwm) = PWM.borrow(cs).borrow_mut().deref_mut()
        {
            pwm.set_duty(max / 12 * value);

            if time % 5 == 0
            {
                SPEED_COUNTER.borrow(cs).set(0);
            }   
        }
    });
}