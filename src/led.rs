//! User LEDs

use stm32f103xx::{GPIOC, Gpioc, Rcc};

/// All the user LEDs
pub static LEDS: [Led; 1] = [
    Led { i: 13 },
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOC.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpioc: &Gpioc, rcc: &Rcc) {
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
