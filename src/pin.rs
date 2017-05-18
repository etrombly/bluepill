//! GPIO pin

use stm32f103xx::{GPIOA, GPIOB, GPIOC, GPIOD, gpioa, Rcc};
pub use hal::pin::Pin as halPin;

/// GPIO pin
pub struct Pin<'a>{
    /// gpio pin
    pub pin: u8,
    /// gpio port
    pub port: &'a gpioa::RegisterBlock,
}

impl<'a> Pin<'a>{
    /// Initializes the Pin
    pub fn init(&self, rcc: &Rcc) {
        // Power up peripherals
        // check which memory block this port is pointing to
        match &*self.port as *const _{
            x if x == GPIOA.get() as *const _ => rcc.apb2enr.modify(|_, w| w.iopaen().enabled()),
            x if x == GPIOB.get() as *const _ => rcc.apb2enr.modify(|_, w| w.iopben().enabled()),
            x if x == GPIOC.get() as *const _ => rcc.apb2enr.modify(|_, w| w.iopcen().enabled()),
            x if x == GPIOD.get() as *const _ => rcc.apb2enr.modify(|_, w| w.iopden().enabled()),
            _ => {},
        }

        // Configure pin 13 as output
        // need to allow other pins to be configured and be able to change pin mode
        self
            .port
            .crh
            .modify(
                |_, w| {
                    w.mode13()
                        .output()
                },
            );
    }
}

impl<'a> halPin for Pin<'a>{
    /// Turns off the Pin
    fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { self.port.bsrr.write(|w| w.bits(1 << (self.pin + 16))) }
    }

    /// Turns on the Pin
    fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { self.port.bsrr.write(|w| w.bits(1 << self.pin)) }
    }
}