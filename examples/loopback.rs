//! Serial interface loopback

#![feature(const_fn)]
#![feature(used)]
#![no_std]

// version = "0.2.0"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate bluepill;

use bluepill::serial::Serial;
use bluepill::stm32f103xx::interrupt::Usart3;
use bluepill::stm32f103xx;
use rtfm::{P0, P1, T0, T1, TMax};

// CONFIGURATION
pub const BAUD_RATE: u32 = 115_200; // bits per second

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

    let serial = Serial{usart: &usart3};

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
fn loopback(_task: Usart3, ref priority: P1, ref threshold: T1) {
    let usart3 = USART3.access(priority, threshold);
    let serial = Serial{usart: &usart3};

    if let Some(byte) = serial.read() {
        if serial.write(byte).is_err() {
            // As we are echoing the bytes as soon as they arrive, it should
            // be impossible to have a TX buffer overrun
            #[cfg(debug_assertions)]
            unreachable!()
        }
    } else {
        // Only reachable through `rtfm::request(loopback)`
        #[cfg(debug_assertions)]
        unreachable!()
    }
}
