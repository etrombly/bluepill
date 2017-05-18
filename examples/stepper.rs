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
use bluepill::stm32f103xx::interrupt::Tim3;
use bluepill::stm32f103xx;
use bluepill::timer::{halTimer, Timer};
use rtfm::{Local, P0, P1, T0, T1, TMax};

// CONFIGURATION
const TICKS: u32 = 400_000; 
const ORDER:[[bool; 4]; 9] = [[false,false,false,true],
                [false,false,true,true],
                [false,false,true,false],
                [false,true,true,false],
                [false,true,false,false],
                [true,true,false,false],
                [true,false,false,false],
                [true,false,false,true],
                [false,false,false,false]];

// RESOURCES
peripherals!(stm32f103xx, {
    GPIOA: Peripheral {
        register_block: Gpioa,
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
    let gpioa = GPIOA.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let timer = Timer::new(&**tim3);
    let in1 = Pin{pin: 1, port: &**gpioa};
    let in2 = Pin{pin: 2, port: &**gpioa};
    let in3 = Pin{pin: 3, port: &**gpioa};
    let in4 = Pin{pin: 4, port: &**gpioa};

    // set clock to 72Mhz
    frequency::init(&rcc, &flash, frequency::Speed::S72Mhz);

    // configure pins for output
    in1.init(&rcc, Mode::OUTPUT);
    in2.init(&rcc, Mode::OUTPUT);
    in3.init(&rcc, Mode::OUTPUT);
    in4.init(&rcc, Mode::OUTPUT);

    // Configure TIM2 for periodic update events
    timer.init(&rcc, TICKS);

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
    static STEP: Local<u16, Tim3> = Local::new(0);


    let tim3 = TIM3.access(priority, threshold);
    let timer = Timer{timer: &**tim3};
    let gpioa = GPIOA.access(priority, threshold);
    let in1 = Pin{pin: 1, port: &**gpioa};
    let in2 = Pin{pin: 2, port: &**gpioa};
    let in3 = Pin{pin: 3, port: &**gpioa};
    let in4 = Pin{pin: 4, port: &**gpioa};

    if timer.clear_update_flag().is_ok() {
        let step = STEP.borrow_mut(&mut task);
        let current = ORDER[*step as usize];

        
        match current[0]{
            true => in1.on(),
            false => in1.off(),
        }
        match current[1]{
            true => in2.on(),
            false => in2.off(),
        }
        match current[2]{
            true => in3.on(),
            false => in3.off(),
        }
        match current[3]{
            true => in4.on(),
            false => in4.off(),
        }

        if *step < 8 {
            *step += 1;
        } else {
            *step = 0;
        }
    } else {
        // Only reachable through `rtfm::request(periodic)`
        //#[cfg(debug_assertion)]
        //unreachable!()
    }
}
