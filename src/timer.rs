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
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    ///
    /// NOTE After initialization, the timer will be in the paused state.
    fn init(&self, rcc: &Rcc, frequency: u32);
    /// Clears the update event flag
    ///
    /// Returns `Err` if no update event has occurred
    fn clear_update_flag(&self) -> Result<()> {
        if self.get_sr().read().uif().is_no_update() {
            Err(Error { _0: () })
        } else {
            self.get_sr().modify(|_, w| w.uif().clear());
            Ok(())
        }
    }
    /// Resumes the timer count
    fn resume(&self) {self.get_cr1().modify(|_, w| w.cen().enabled());}
    /// Pauses the timer
    fn pause(&self) {self.get_cr1().modify(|_, w| w.cen().disabled());}
    /// Returns a reference to the cr1 register
    fn get_cr1(&self) -> &tim2::Cr1;
    /// Returns a reference to the sr1 register
    fn get_sr(&self) -> &tim2::Sr;
}

impl Timer for Tim2{
    fn init(&self, rcc: &Rcc, frequency: u32) {
        // Power up peripherals
        rcc.apb1enr.modify(|_, w| w.tim2en().enabled());

        let speeds = frequency::ClockSpeeds::get(rcc);

        let ratio = speeds.apb1 / frequency;
        let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
        self.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        self.arr.write(|w| w.arr().bits(arr));

        self.dier.write(|w| unsafe { w.uie().bits(1) });
        self.cr1.write(|w| w.opm().continuous());
    }

    fn get_cr1(&self) -> &tim2::Cr1 {
        &self.cr1
    }

    fn get_sr(&self) -> &tim2::Sr {
        &self.sr
    }
}

impl Timer for Tim3{
    fn init(&self, rcc: &Rcc, frequency: u32) {
        // Power up peripherals
        rcc.apb1enr.modify(|_, w| w.tim3en().enabled());

        let speeds = frequency::ClockSpeeds::get(rcc);

        let ratio = speeds.apb1 / frequency;
        let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
        self.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        self.arr.write(|w| w.arr().bits(arr));

        self.dier.write(|w| unsafe { w.uie().bits(1) });
        self.cr1.write(|w| w.opm().continuous());
    }

    fn get_cr1(&self) -> &tim2::Cr1 {
        &self.cr1
    }

    fn get_sr(&self) -> &tim2::Sr {
        &self.sr
    }
}

impl Timer for Tim4{
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    ///
    /// NOTE After initialization, the timer will be in the paused state.
    fn init(&self, rcc: &Rcc, frequency: u32) {
        // Power up peripherals
        rcc.apb1enr.modify(|_, w| w.tim4en().enabled());

        let speeds = frequency::ClockSpeeds::get(rcc);

        let ratio = speeds.apb1 / frequency;
        let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
        self.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        self.arr.write(|w| w.arr().bits(arr));

        self.dier.write(|w| unsafe { w.uie().bits(1) });
        self.cr1.write(|w| w.opm().continuous());
    }

    fn get_cr1(&self) -> &tim2::Cr1 {
        &self.cr1
    }

    fn get_sr(&self) -> &tim2::Sr {
        &self.sr
    }
}

impl Timer for Tim5{
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    ///
    /// NOTE After initialization, the timer will be in the paused state.
    fn init(&self, rcc: &Rcc, frequency: u32) {
        // Power up peripherals
        rcc.apb1enr.modify(|_, w| w.tim5en().enabled());

        let speeds = frequency::ClockSpeeds::get(rcc);

        let ratio = speeds.apb1 / frequency;
        let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
        self.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        self.arr.write(|w| w.arr().bits(arr));

        self.dier.write(|w| unsafe { w.uie().bits(1) });
        self.cr1.write(|w| w.opm().continuous());
    }

    fn get_cr1(&self) -> &tim2::Cr1 {
        &self.cr1
    }

    fn get_sr(&self) -> &tim2::Sr {
        &self.sr
    }
}