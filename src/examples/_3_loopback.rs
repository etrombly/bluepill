//! Serial interface loopback
//!
//! ```
//! 
//! #![feature(const_fn)]
//! #![feature(used)]
//! #![no_std]
//! 
//! // version = "0.2.0"
//! extern crate cortex_m_rt;
//! 
//! // version = "0.1.0"
//! #[macro_use]
//! extern crate cortex_m_rtfm as rtfm;
//! 
//! extern crate f3;
//! 
//! use f3::serial::Serial;
//! use f3::stm32f30x::interrupt::Usart1Exti25;
//! use f3::stm32f30x;
//! use rtfm::{P0, P1, T0, T1, TMax};
//! 
//! // CONFIGURATION
//! pub const BAUD_RATE: u32 = 115_200; // bits per second
//! 
//! // RESOURCES
//! peripherals!(stm32f30x, {
//!     GPIOA: Peripheral {
//!         register_block: Gpioa,
//!         ceiling: C0,
//!     },
//!     RCC: Peripheral {
//!         register_block: Rcc,
//!         ceiling: C0,
//!     },
//!     USART1: Peripheral {
//!         register_block: Usart1,
//!         ceiling: C1,
//!     },
//! });
//! 
//! // INITIALIZATION PHASE
//! fn init(ref priority: P0, threshold: &TMax) {
//!     let gpioa = GPIOA.access(priority, threshold);
//!     let rcc = RCC.access(priority, threshold);
//!     let usart1 = USART1.access(priority, threshold);
//! 
//!     let serial = Serial(&usart1);
//! 
//!     serial.init(&gpioa, &rcc, BAUD_RATE);
//! }
//! 
//! // IDLE LOOP
//! fn idle(_priority: P0, _threshold: T0) -> ! {
//!     // Sleep
//!     loop {
//!         rtfm::wfi();
//!     }
//! }
//! 
//! // TASKS
//! tasks!(stm32f30x, {
//!     loopback: Task {
//!         interrupt: Usart1Exti25,
//!         priority: P1,
//!         enabled: true,
//!     },
//! });
//! 
//! // Send back the received byte
//! fn loopback(_task: Usart1Exti25, ref priority: P1, ref threshold: T1) {
//!     let usart1 = USART1.access(priority, threshold);
//!     let serial = Serial(&usart1);
//! 
//!     if let Ok(byte) = serial.read() {
//!         if serial.write(byte).is_err() {
//!             // As we are echoing the bytes as soon as they arrive, it should
//!             // be impossible to have a TX buffer overrun
//!             #[cfg(debug_assertions)]
//!             unreachable!()
//!         }
//!     } else {
//!         // Only reachable through `rtfm::request(loopback)`
//!         #[cfg(debug_assertions)]
//!         unreachable!()
//!     }
//! }
//! ```
// Auto-generated. Do not modify.
