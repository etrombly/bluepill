//! Periodic timer
use core::u16;

use cast::{u16, u32};
use stm32f103xx::{Rcc, tim2, TIM2, TIM3, TIM4, TIM5};

use frequency;

pub use hal::timer::Timer as halTimer;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// General purpose timer
pub struct Timer<'a>{
    /// general purpose timer
    pub timer: &'a tim2::RegisterBlock,
}

impl<'a> Timer<'a>{
    /// return new timer
    pub fn new(timer: &'a tim2::RegisterBlock) -> Timer {
        Timer{timer}
    }

    /// initialize timer to frequency
    pub fn init(&self, rcc: &Rcc, ticks: u32) {
        // Power up peripherals
        // check which memory block this timer is pointing to
        match &*self.timer as *const _{
            x if x == TIM2.get() as *const _ => rcc.apb1enr.modify(|_, w| w.tim2en().enabled()),
            x if x == TIM3.get() as *const _ => rcc.apb1enr.modify(|_, w| w.tim3en().enabled()),
            x if x == TIM4.get() as *const _ => rcc.apb1enr.modify(|_, w| w.tim4en().enabled()),
            x if x == TIM5.get() as *const _ => rcc.apb1enr.modify(|_, w| w.tim5en().enabled()),
            _ => {},
        }

        let speeds = frequency::ClockSpeeds::get(rcc);

        //let ratio = speeds.apb1 / frequency;
        let psc = u16((ticks - 1) / u32(u16::MAX)).unwrap();
        self.timer.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ticks / u32(psc + 1)).unwrap();
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

impl<'a> halTimer for Timer<'a>{
    fn pause(&self){
            self.timer.cr1.modify(|_, w| w.cen().disabled());
    }

    fn resume(&self){
            self.timer.cr1.modify(|_, w| w.cen().enabled());
    }
}