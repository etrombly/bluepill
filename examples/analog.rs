// examples/blinky.rs
//! Blinks an LED

#![feature(const_fn)]
#![feature(used)]
#![no_std]

// version = "0.2.0"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate bluepill;

use bluepill::pin::{halPin, Pin, Mode};
use bluepill::frequency;
use bluepill::stm32f103xx::interrupt::Tim2;
use bluepill::stm32f103xx;
use bluepill::timer::{halTimer, Timer};
use rtfm::{Local, P0, P1, T0, T1, TMax};

// CONFIGURATION
const TICKS: u32 = 36_000_000; // should be one a second

// RESOURCES
peripherals!(stm32f103xx, {
    GPIOB: Peripheral {
        register_block: Gpiob,
        ceiling: C1,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    TIM2: Peripheral {
        register_block: Tim2,
        ceiling: C1,
    },
    TIM3: Peripheral {
        register_block: Tim3,
        ceiling: C1,
    },
    FLASH: Peripheral {
        register_block: Flash,
        ceiling: C0,
    },
});

// INITIALIZATION PHASE
fn init(ref priority: P0, threshold: &TMax) {
    let gpiob = GPIOB.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim2 = TIM2.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let timer2 = Timer::new(&**tim2);
    let led = Pin::new_analog_out(1, &**gpiob, &**tim3);

    // set clock to 72Mhz
    frequency::init(&rcc, &flash, frequency::Speed::S72Mhz);

    // Configure the PEx pins as output pins
    led.init(&rcc, Mode::ANALOG_OUTPUT);

    // Configure TIM2 for periodic update events
    timer2.init(&rcc, TICKS);

    // Start the timer
    timer2.resume();
}

// IDLE LOOP
fn idle(_priority: P0, _threshold: T0) -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f103xx, {
    periodic: Task {
        interrupt: Tim2,
        priority: P1,
        enabled: true,
    },
});

fn periodic(mut task: Tim2, ref priority: P1, ref threshold: T1) {
    // Task local data
    static STATE: Local<bool, Tim2> = Local::new(false);
    static DUTY: Local<u16, Tim2> = Local::new(1000);


    let tim2 = TIM2.access(priority, threshold);
    let timer2 = Timer{timer: &**tim2};
    let tim3 = TIM3.access(priority, threshold);
    let gpiob = GPIOB.access(priority, threshold);
    let led = Pin::new_analog_out(1, &**gpiob, &**tim3);

    if timer2.clear_update_flag().is_ok() {
        //let state = STATE.borrow_mut(&mut task);

        //*state = !*state;

        //if *state {
            // led is inverted, this actually turns the led off
        //    led.analog_write(0);
        //} else {
            // arr is 12_000 for testing, should be 50% duty cycle
            // will make this a percentage or something later
            let duty = DUTY.borrow_mut(&mut task);
            led.analog_write(*duty);
            *duty -= 10;
        //}
    } else {
        // Only reachable through `rtfm::request(periodic)`
        //#[cfg(debug_assertion)]
        //unreachable!()
    }
}
