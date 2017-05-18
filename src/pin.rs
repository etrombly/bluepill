//! User LEDs

use stm32f103xx::{GPIOC, Gpioc, Rcc};

/// pin mode, input or output
pub enum Mode {
    /// input mode
    INPUT, 
    /// output mode
    OUTPUT}

/// Represents a pin
pub trait Pin {
    /// Initializes the Pin
    fn init(gpioc: &Gpioc, rcc: &Rcc) {
        // Power up peripherals
        rcc.apb2enr.modify(|_, w| w.iopcen().enabled());

        // Configure pin 13 as output
        gpioc
            .crh
            .modify(
                |_, w| {
                    w.mode13()
                        .output()
                },
            );
    }
    
    /// Turns off the Pin
    fn off(&self) {
        // NOTE(safe) atomic write
        //unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the Pin
    fn on(&self) {
        // NOTE(safe) atomic write
        //unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}
