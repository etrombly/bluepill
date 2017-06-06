//! Serial interface buffered loopback

#![feature(const_fn)]
#![feature(used)]
#![no_std]

// version = "0.2.0"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate bluepill;
extern crate heapless;

use bluepill::serial::Serial;
use bluepill::stm32f103xx::interrupt::Usart3;
use bluepill::stm32f103xx;
use rtfm::{Resource, P0, P1, T0, T1, TMax, C1};
use core::cell::RefCell;
use heapless::Vec;

// CONFIGURATION
pub const BAUD_RATE: u32 = 115_200; // bits per second

struct Buffer {
    buff: RefCell<Vec<u8, [u8; 20]>>,
}

impl Buffer {
    const fn new() -> Self {
        Buffer {buff: RefCell::new(Vec::new([0; 20]))}
    }
}

static RXQ: Resource<Buffer, C1> = Resource::new(Buffer::new());

// RESOURCES
peripherals!(stm32f103xx, {
    GPIOB: Peripheral {
        register_block: Gpiob,
        ceiling: C0,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
    USART3: Peripheral {
        register_block: Usart3,
        ceiling: C1,
    },
});

// INITIALIZATION PHASE
fn init(ref priority: P0, threshold: &TMax) {
    let gpiob = GPIOB.access(priority, threshold);
    let rcc = RCC.access(priority, threshold);
    let usart3 = USART3.access(priority, threshold);

    let serial = Serial{usart: &**usart3};

    serial.init(&gpiob, &rcc, BAUD_RATE);
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
    loopback: Task {
        interrupt: Usart3,
        priority: P1,
        enabled: true,
    },
});

// Send back the received byte
fn loopback(mut _task: Usart3, ref priority: P1, ref threshold: T1) {
    let rxq = RXQ.access(priority, threshold);
    let mut buff = rxq.buff.borrow_mut();

    let usart3 = USART3.access(priority, threshold);
    let serial = Serial{usart: &**usart3};

    if let Some(byte) = serial.read() {
        if buff.push(byte).is_err() {
                // error: buffer full
                // KISS: we just clear the buffer when it gets full
                buff.clear();
            }
        // Carriage return
        if byte == 13 {
            while let Some(x) = buff.pop(){
                while serial.write(x).is_err() {
                    // resend if tx buffer is full
                    // should put a timeout in here later
                }
            }
        }
    } else {
        // Only reachable through `rtfm::request(loopback)`
        #[cfg(debug_assertions)]
        unreachable!()
    }
}
