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
extern crate haldriver;

use bluepill::pin::{Pin, Mode};
use bluepill::frequency;
use bluepill::stm32f103xx::interrupt::{Tim3, Tim2};
use bluepill::stm32f103xx;
use bluepill::timer::{halTimer, Timer};
use rtfm::{Local, Resource, C2, P0, P1, P2, T0, T1, T2, TMax};
use haldriver::stepper::ulnXXXX::{Stepper, halStepper, Direction};
use core::cell::Cell;

// CONFIGURATION
const TICKS: u32 = 64_000;
const TICKS2: u32 = 16_000_000;

struct stepCount {
    steps: Cell<u32>,
}

impl stepCount {
    const fn new() -> Self {
        stepCount {steps: Cell::new(0)}
    }
}

// RESOURCES
peripherals!(stm32f103xx, {
    GPIOA: Peripheral {
        register_block: Gpioa,
        ceiling: C2,
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
        ceiling: C2,
    },
    FLASH: Peripheral {
        register_block: Flash,
        ceiling: C0,
    },
});

static XSTEPS: Resource<stepCount, C2> = Resource::new(stepCount::new());

// INITIALIZATION PHASE
fn init(ref priority: P0, threshold: &TMax) {
    let gpioa = GPIOA.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let tim2 = TIM2.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let timer = Timer::new(&**tim3);
    let timer2 = Timer::new(&**tim2);
    let in1 = Pin{pin: 1, port: &**gpioa};
    let in2 = Pin{pin: 2, port: &**gpioa};
    let in3 = Pin{pin: 3, port: &**gpioa};
    let in4 = Pin{pin: 4, port: &**gpioa};

    // set clock to 72Mhz
    frequency::init(&rcc, &flash, frequency::Speed::S32Mhz);

    // configure pins for output
    in1.init(&rcc, Mode::OUTPUT);
    in2.init(&rcc, Mode::OUTPUT);
    in3.init(&rcc, Mode::OUTPUT);
    in4.init(&rcc, Mode::OUTPUT);

    // Configure TIM3 for periodic update events
    timer.init(&rcc, TICKS);

    // Configure TIM5 for periodic update events
    timer2.init(&rcc, TICKS2);

    // Start the timer
    timer.resume();
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
    stepper: Task {
        interrupt: Tim3,
        priority: P2,
        enabled: true,
    },
    controller: Task {
        interrupt: Tim2,
        priority: P1,
        enabled: true,
    },
});

fn stepper(mut task: Tim3, ref priority: P2, ref threshold: T2) {
    // Task local data
    // have to track step manually since you can't persist
    // a stepper between calls to periodic 
    static STEP: Local<u16, Tim3> = Local::new(0);

    let tim3 = TIM3.access(priority, threshold);
    let timer = Timer{timer: &**tim3};
    let gpioa = GPIOA.access(priority, threshold);
    let in1 = Pin{pin: 1, port: &**gpioa};
    let in2 = Pin{pin: 2, port: &**gpioa};
    let in3 = Pin{pin: 3, port: &**gpioa};
    let in4 = Pin{pin: 4, port: &**gpioa};

    if timer.clear_update_flag().is_ok() {
        let xsteps = XSTEPS.access(priority, threshold);
        if xsteps.steps.get() > 0 {
            let step = STEP.borrow_mut(&mut task);

            let mut stepper = Stepper{direction: Direction::RIGHT,
                index: *step,
                pin1: &in1,
                pin2: &in2,
                pin3: &in3,
                pin4: &in4,};

            stepper.step();

            if *step < 8 {
                *step += 1;
            } else {
                *step = 0;
            }
            xsteps.steps.set(xsteps.steps.get() - 1);
        }
    } else {
        // Only reachable through `rtfm::request(periodic)`
        #[cfg(debug_assertion)]
        unreachable!()
    }
}

fn controller(mut task: Tim2, ref priority: P1, ref threshold: T1) {
    let tim2 = TIM2.access(priority, threshold);
    let timer = Timer{timer: &**tim2};


    if timer.clear_update_flag().is_ok() {
        // make sure current move is completed before sending next move
        while threshold.raise(
                &XSTEPS, |threshold| {
                    let xsteps = XSTEPS.access(priority, threshold);
                    xsteps.steps.get()
                }
        ) > 0 {}
        threshold.raise(
                &XSTEPS, |threshold| {
                    let xsteps = XSTEPS.access(priority, threshold);
                    xsteps.steps.set(100);
                }
        );
    } else {
        // Only reachable through `rtfm::request(periodic)`
        #[cfg(debug_assertion)]
        unreachable!()
    }
}