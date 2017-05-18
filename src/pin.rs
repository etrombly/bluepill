//! GPIO pin

use stm32f103xx::{GPIOA, GPIOB, GPIOC, GPIOD, gpioa, Rcc};
pub use hal::pin::Pin as halPin;
pub use hal::pin::{State, Mode};

/// GPIO pin
pub struct Pin<'a>{
    /// gpio pin
    pub pin: u8,
    /// gpio port
    pub port: &'a gpioa::RegisterBlock,
}

impl<'a> Pin<'a>{
    /// Initializes the Pin
    pub fn init(&self, rcc: &Rcc, mode: Mode) {
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
        // still need to set cnf bits and handle analog/digital

        match mode {
            Mode::INPUT =>
                match self.pin {
                    0 => self.port.crl.modify(|_,w| w.mode0().input()),
                    1 => self.port.crl.modify(|_,w| w.mode1().input()),
                    2 => self.port.crl.modify(|_,w| w.mode2().input()),
                    3 => self.port.crl.modify(|_,w| w.mode3().input()),
                    4 => self.port.crl.modify(|_,w| w.mode4().input()),
                    5 => self.port.crl.modify(|_,w| w.mode5().input()),
                    6 => self.port.crl.modify(|_,w| w.mode6().input()),
                    7 => self.port.crl.modify(|_,w| w.mode7().input()),
                    8 => self.port.crh.modify(|_,w| w.mode8().input()),
                    9 => self.port.crh.modify(|_,w| w.mode9().input()),
                    10 => self.port.crh.modify(|_,w| w.mode10().input()),
                    11 => self.port.crh.modify(|_,w| w.mode11().input()),
                    12 => self.port.crh.modify(|_,w| w.mode12().input()),
                    13 => self.port.crh.modify(|_,w| w.mode13().input()),
                    14 => self.port.crh.modify(|_,w| w.mode14().input()),
                    15 => self.port.crh.modify(|_,w| w.mode15().input()),
                    _ => {},
                },
            Mode::OUTPUT =>
                match self.pin {
                    0 => self.port.crl.modify(|_,w| w.mode0().output()),
                    1 => self.port.crl.modify(|_,w| w.mode1().output()),
                    2 => self.port.crl.modify(|_,w| w.mode2().output()),
                    3 => self.port.crl.modify(|_,w| w.mode3().output()),
                    4 => self.port.crl.modify(|_,w| w.mode4().output()),
                    5 => self.port.crl.modify(|_,w| w.mode5().output()),
                    6 => self.port.crl.modify(|_,w| w.mode6().output()),
                    7 => self.port.crl.modify(|_,w| w.mode7().output()),
                    8 => self.port.crh.modify(|_,w| w.mode8().output()),
                    9 => self.port.crh.modify(|_,w| w.mode9().output()),
                    10 => self.port.crh.modify(|_,w| w.mode10().output()),
                    11 => self.port.crh.modify(|_,w| w.mode11().output()),
                    12 => self.port.crh.modify(|_,w| w.mode12().output()),
                    13 => self.port.crh.modify(|_,w| w.mode13().output()),
                    14 => self.port.crh.modify(|_,w| w.mode14().output()),
                    15 => self.port.crh.modify(|_,w| w.mode15().output()),
                    _ => {},
                },
        };
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

    // return state of pin
    fn digital_read(&self) -> State {
        match self.port.idr.read().bits() & (1 << self.pin){
            0 => State::LOW,
            _ => State::HIGH
        }
    }
}