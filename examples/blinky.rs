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

use bluepill::pin::{halPin, Pin};
use bluepill::frequency;
use bluepill::stm32f103xx::interrupt::Tim3;
use bluepill::stm32f103xx;
use bluepill::timer::{halTimer, Timer};
use rtfm::{Local, P0, P1, T0, T1, TMax};

// CONFIGURATION
const FREQUENCY: u32 = 1; // Hz

// RESOURCES
peripherals!(stm32f103xx, {
    GPIOC: Peripheral {
        register_block: Gpioc,
        ceiling: C1,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
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
    let gpioc = GPIOC.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let timer = Timer{timer: &**tim3};
    let led = Pin{pin: 13, port: &**gpioc};

    // set clock to 72Mhz
    frequency::init(&rcc, &flash, frequency::Speed::S72Mhz);

    // Configure the PEx pins as output pins
    led.init(&rcc);

    // Configure TIM2 for periodic update events
    timer.init(&rcc, FREQUENCY);

    // Start the timer
    timer.resume();
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
        interrupt: Tim3,
        priority: P1,
        enabled: true,
    },
});

fn periodic(mut task: Tim3, ref priority: P1, ref threshold: T1) {
    // Task local data
    static STATE: Local<bool, Tim3> = Local::new(false);


    let tim3 = TIM3.access(priority, threshold);
    let timer = Timer{timer: &**tim3};
    let gpioc = GPIOC.access(priority, threshold);
    let led = Pin{pin: 13, port: &**gpioc};

    if timer.clear_update_flag().is_ok() {
        let state = STATE.borrow_mut(&mut task);

        *state = !*state;

        if *state {
            led.on();
        } else {
            led.off();
        }
    } else {
        // Only reachable through `rtfm::request(periodic)`
        //#[cfg(debug_assertion)]
        //unreachable!()
    }

}
