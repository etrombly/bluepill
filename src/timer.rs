//! Periodic timer

use core::u16;

use cast::{u16, u32};
use stm32f103xx::{Rcc, Tim2, tim2, Tim3, Tim4, Tim5};


use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// General use timer
pub trait Timer {
    /// Resumes the timer count
    fn resume(&self);
    /// Pauses the timer
    fn pause(&self);
}

/// General purpose timer
pub struct genTimer<'a>{
    /// general purpose timer
    pub timer: &'a tim2::RegisterBlock,
}

impl<'a> genTimer<'a>{
    /// initialize timer to frequency
    pub fn init(&self, rcc: &Rcc, frequency: u32) {
        // Power up peripherals
        //not working this way, matches on tim2 always
        match *self{
            ref Tim2 => rcc.apb1enr.modify(|_, w| w.tim2en().enabled()),
            ref Tim3 => rcc.apb1enr.modify(|_, w| w.tim3en().enabled()),
            ref Tim4 => rcc.apb1enr.modify(|_, w| w.tim4en().enabled()),
            ref Tim5 => rcc.apb1enr.modify(|_, w| w.tim5en().enabled()),
        }

        //manually enable tim3 for testing
        rcc.apb1enr.modify(|_, w| w.tim3en().enabled());

        let speeds = frequency::ClockSpeeds::get(rcc);

        let ratio = speeds.apb1 / frequency;
        let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
        self.timer.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        self.timer.arr.write(|w| w.arr().bits(arr));

        self.timer.dier.write(|w| unsafe { w.uie().bits(1) });
        self.timer.cr1.write(|w| w.opm().continuous());
    }

    /// Clears the update event flag
    ///
    /// Returns `Err` if no update event has occurred
    pub fn clear_update_flag(&self) -> Result<()> {
        if self.timer.sr.read().uif().is_no_update() {
            Err(Error { _0: () })
        } else {
            self.timer.sr.modify(|_, w| w.uif().clear());
            Ok(())
        }
    }
}

impl<'a> Timer for genTimer<'a>{
    fn pause(&self){
            self.timer.cr1.modify(|_, w| w.cen().disabled());
    }

    fn resume(&self){
            self.timer.cr1.modify(|_, w| w.cen().enabled());
            let test = self.timer.cr1.read().cen();
            let test = self.timer.cr1.read().cen();
    }
}