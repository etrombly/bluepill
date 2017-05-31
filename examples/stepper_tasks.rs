// examples/stepper_tasks.rs
//! move x and y axis stepper motors with seperate controller task

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

// stepper update ticks, 500 pulse/sec is around the max
const TICKS: u32 = 144_000;
// controller update ticks still need to tweak for a good value
const TICKS2: u32 = 1_000_000;

// XPATTERN and YPATTERN are the path for the x and y axis
const XPATTERN: [i32; 5] = [4096, -2000, 500, -500, 200];
const YPATTERN: [i32; 5] = [1000, -2000, 1500, -500, 200];

fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

struct stepCount {
    steps: Cell<i32>,
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

// XSTEPS and YSTEPS hold the amount of steps for the current move
static XSTEPS: Resource<stepCount, C2> = Resource::new(stepCount::new());
static YSTEPS: Resource<stepCount, C2> = Resource::new(stepCount::new());

// INITIALIZATION PHASE
fn init(ref priority: P0, threshold: &TMax) {
    let gpioa = GPIOA.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let tim3 = TIM3.access(priority, threshold);
    let tim2 = TIM2.access(priority, threshold);
    let flash = FLASH.access(priority, threshold);
    let timer = Timer::new(&**tim3);
    let timer2 = Timer::new(&**tim2);
    // stepper pins
    let in1 = Pin::new(1, &**gpioa);
    let in2 = Pin::new(2, &**gpioa);
    let in3 = Pin::new(3, &**gpioa);
    let in4 = Pin::new(4, &**gpioa);
    let in5 = Pin::new(8, &**gpioa);
    let in6 = Pin::new(9, &**gpioa);
    let in7 = Pin::new(10, &**gpioa);
    let in8 = Pin::new(11, &**gpioa);

    // set clock to 32Mhz
    frequency::init(&rcc, &flash, frequency::Speed::S32Mhz);

    // configure pins for output
    in1.init(&rcc, Mode::OUTPUT);
    in2.init(&rcc, Mode::OUTPUT);
    in3.init(&rcc, Mode::OUTPUT);
    in4.init(&rcc, Mode::OUTPUT);
    in5.init(&rcc, Mode::OUTPUT);
    in6.init(&rcc, Mode::OUTPUT);
    in7.init(&rcc, Mode::OUTPUT);
    in8.init(&rcc, Mode::OUTPUT);

    // Configure TIM3 for periodic update events
    timer.init(&rcc, TICKS);

    // Configure TIM2 for periodic update events
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
    // a stepper between calls
    static XINDEX: Local<u8, Tim3> = Local::new(0);
    static YINDEX: Local<u8, Tim3> = Local::new(0);

    let tim3 = TIM3.access(priority, threshold);
    let timer = Timer{timer: &**tim3};
    let gpioa = GPIOA.access(priority, threshold);
    let in1 = Pin::new(1, &**gpioa);
    let in2 = Pin::new(2, &**gpioa);
    let in3 = Pin::new(3, &**gpioa);
    let in4 = Pin::new(4, &**gpioa);
    let in5 = Pin::new(8, &**gpioa);
    let in6 = Pin::new(9, &**gpioa);
    let in7 = Pin::new(10, &**gpioa);
    let in8 = Pin::new(11, &**gpioa);

    if timer.clear_update_flag().is_ok() {
        let xsteps = XSTEPS.access(priority, threshold);
        // if there are any current x steps to take
        if xsteps.steps.get() != 0 {
            let xindex = XINDEX.borrow_mut(&mut task);

            let mut stepper = Stepper{direction: Direction::RIGHT,
                index: *xindex,
                pin1: &in1,
                pin2: &in2,
                pin3: &in3,
                pin4: &in4,};

            stepper.step();

            // take one xstep
            if xsteps.steps.get() > 0 {
                *xindex = if *xindex < 3 { *xindex + 1 } else { 0 };
                xsteps.steps.set(xsteps.steps.get() - 1);
            } else {
                *xindex = if *xindex > 0 { *xindex - 1 } else { 3 };
                xsteps.steps.set(xsteps.steps.get() + 1);
            }
        }

        let ysteps = YSTEPS.access(priority, threshold);
        //if there are any ysteps to take
        if ysteps.steps.get() != 0 {
            let yindex = YINDEX.borrow_mut(&mut task);

            let mut stepper = Stepper{direction: Direction::RIGHT,
                index: *yindex,
                pin1: &in5,
                pin2: &in6,
                pin3: &in7,
                pin4: &in8,};

            stepper.step();

            // take one ystep
            if ysteps.steps.get() > 0 {
                *yindex = if *yindex < 3 { *yindex + 1 } else { 0 };
                ysteps.steps.set(ysteps.steps.get() - 1);
            } else {
                *yindex = if *yindex > 0 { *yindex -1 } else { 3 };
                ysteps.steps.set(ysteps.steps.get() + 1);
            }
        }
    } else {
        // Only reachable through `rtfm::request(periodic)`
        #[cfg(debug_assertion)]
        unreachable!()
    }
}

// send steps to the stepper task, eventually add some path planning and
// input for movement from serial
fn controller(mut task: Tim2, ref priority: P1, ref threshold: T1) {
    static XINDEX: Local<u8, Tim2> = Local::new(0);
    static YINDEX: Local<u8, Tim2> = Local::new(0);

    let tim2 = TIM2.access(priority, threshold);
    let timer = Timer{timer: &**tim2};


    if timer.clear_update_flag().is_ok() {
        // get current movement
        let xmove = XPATTERN[*XINDEX.borrow(&task) as usize];
        let ymove = YPATTERN[*YINDEX.borrow(&task) as usize];
        
        // figure out step size
        // probably need to add a check if either are 0
        let gcd = gcd(xmove, ymove);
        let xmove = xmove / gcd;
        let ymove = ymove / gcd;
        for _ in 0..gcd{
            // make sure current move is completed before sending next move
            // blocks on both x and y
            while threshold.raise(
                    &XSTEPS, |threshold| {
                        let xsteps = XSTEPS.access(priority, threshold);
                        xsteps.steps.get()
                    }
            ) != 0 {}
            while threshold.raise(
                    &YSTEPS, |threshold| {
                        let ysteps = YSTEPS.access(priority, threshold);
                        ysteps.steps.get()
                    }
            ) != 0 {}
            // send the next x move
            threshold.raise(
                &XSTEPS, |threshold| {
                    let xsteps = XSTEPS.access(priority, threshold);
                    xsteps.steps.set(xmove);
                }
            );
            // send the next y move
            threshold.raise(
                &YSTEPS, |threshold| {
                    let ysteps = YSTEPS.access(priority, threshold);
                    ysteps.steps.set(ymove);
                }
            );
        }
        // need to get mutable x index in it's own block due
        // to borrow checker being overly strick with borrow_mut
        {
            let xindex = XINDEX.borrow_mut(&mut task);
            *xindex = if *xindex < XPATTERN.len() as u8 - 1 { *xindex + 1 } else { 0 };
        }
        let yindex = YINDEX.borrow_mut(&mut task);
        *yindex = if *yindex < YPATTERN.len() as u8 - 1 { *yindex + 1 } else { 0 };
    } else {
        // Only reachable through `rtfm::request(periodic)`
        #[cfg(debug_assertion)]
        unreachable!()
    }
}