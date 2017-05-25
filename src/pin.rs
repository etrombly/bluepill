//! GPIO pin
use core::u16;

use cast::{u16, u32};

use stm32f103xx::{GPIOA, GPIOB, GPIOC, GPIOD, gpioa, Rcc, adc1, tim2};
pub use hal::pin::Pin as halPin;
pub use hal::pin::{State, Mode};

/// GPIO pin
pub struct Pin<'a>{
    /// gpio pin
    pub pin: u8,
    /// gpio port
    pub port: &'a gpioa::RegisterBlock,
    adc: Option<&'a adc1::RegisterBlock>,
    timer: Option<&'a tim2::RegisterBlock>,
}

impl<'a> Pin<'a>{
    /// returns a digital pin
    pub fn new(pin: u8, port: &'a gpioa::RegisterBlock) -> Pin {
        Pin{pin, port, adc: None, timer: None}
    }

    /// returns an analog input pin
    pub fn new_analog_in(pin: u8, port: &'a gpioa::RegisterBlock, adc: &'a adc1::RegisterBlock) -> Pin<'a> {
        Pin{pin, port, adc: Some(adc), timer: None}
    }

    /// returns an analog output pin
    pub fn new_analog_out(pin: u8, port: &'a gpioa::RegisterBlock, timer: &'a tim2::RegisterBlock) -> Pin<'a> {
        Pin{pin, port, adc: None, timer: Some(timer)}
    }

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

        // Configure pin to input/output
        // still need to handle analog input mode, and possibly setting output frequency
        // defaults to 10Mhz output

        match mode {
            Mode::INPUT =>
                match self.pin {
                    // cnf alt_push mode is actually input pullup/pulldown, since cnf is shared 
                    // for output mode the names are confusing
                    0 => self.port.crl.modify(|_,w| w.mode0().input()
                                                    .cnf0().alt_push()),
                    1 => self.port.crl.modify(|_,w| w.mode1().input()
                                                    .cnf1().alt_push()),
                    2 => self.port.crl.modify(|_,w| w.mode2().input()
                                                    .cnf2().alt_push()),
                    3 => self.port.crl.modify(|_,w| w.mode3().input()
                                                    .cnf3().alt_push()),
                    4 => self.port.crl.modify(|_,w| w.mode4().input()
                                                    .cnf4().alt_push()),
                    5 => self.port.crl.modify(|_,w| w.mode5().input()
                                                    .cnf5().alt_push()),
                    6 => self.port.crl.modify(|_,w| w.mode6().input()
                                                    .cnf6().alt_push()),
                    7 => self.port.crl.modify(|_,w| w.mode7().input()
                                                    .cnf7().alt_push()),
                    8 => self.port.crh.modify(|_,w| w.mode8().input()
                                                    .cnf8().alt_push()),
                    9 => self.port.crh.modify(|_,w| w.mode9().input()
                                                    .cnf9().alt_push()),
                    10 => self.port.crh.modify(|_,w| w.mode10().input()
                                                    .cnf10().alt_push()),
                    11 => self.port.crh.modify(|_,w| w.mode11().input()
                                                    .cnf11().alt_push()),
                    12 => self.port.crh.modify(|_,w| w.mode12().input()
                                                    .cnf12().alt_push()),
                    13 => self.port.crh.modify(|_,w| w.mode13().input()
                                                    .cnf13().alt_push()),
                    14 => self.port.crh.modify(|_,w| w.mode14().input()
                                                    .cnf14().alt_push()),
                    15 => self.port.crh.modify(|_,w| w.mode15().input()
                                                    .cnf15().alt_push()),
                    _ => {},
                },
            Mode::ANALOG_INPUT => {},
            Mode::OUTPUT =>
                match self.pin {
                    0 => self.port.crl.modify(|_,w| w.mode0().output()
                                                        .cnf0().push()),
                    1 => self.port.crl.modify(|_,w| w.mode1().output()
                                                        .cnf1().push()),
                    2 => self.port.crl.modify(|_,w| w.mode2().output()
                                                        .cnf2().push()),
                    3 => self.port.crl.modify(|_,w| w.mode3().output()
                                                        .cnf3().push()),
                    4 => self.port.crl.modify(|_,w| w.mode4().output()
                                                        .cnf4().push()),
                    5 => self.port.crl.modify(|_,w| w.mode5().output()
                                                        .cnf5().push()),
                    6 => self.port.crl.modify(|_,w| w.mode6().output()
                                                        .cnf6().push()),
                    7 => self.port.crl.modify(|_,w| w.mode7().output()
                                                        .cnf7().push()),
                    8 => self.port.crh.modify(|_,w| w.mode8().output()
                                                        .cnf8().push()),
                    9 => self.port.crh.modify(|_,w| w.mode9().output()
                                                        .cnf9().push()),
                    10 => self.port.crh.modify(|_,w| w.mode10().output()
                                                        .cnf10().push()),
                    11 => self.port.crh.modify(|_,w| w.mode11().output()
                                                        .cnf11().push()),
                    12 => self.port.crh.modify(|_,w| w.mode12().output()
                                                        .cnf12().push()),
                    13 => self.port.crh.modify(|_,w| w.mode13().output()
                                                        .cnf13().push()),
                    14 => self.port.crh.modify(|_,w| w.mode14().output()
                                                        .cnf14().push()),
                    15 => self.port.crh.modify(|_,w| w.mode15().output()
                                                        .cnf15().push()),
                    _ => {},
                },
            Mode::ANALOG_OUTPUT => {
                if let Some(timer) = self.timer {
                    // using arbitrary values for testing now, will figure out good defaults later
                    // should be roughly 1hz
                    timer.psc.write(|w| w.psc().bits(6000));

                    // set frequency
                    timer.arr.write(|w| w.arr().bits(12000));

                    //set duty cycle
                    timer.ccr4.write(|w| unsafe{ w.ccr4().bits(0)} );

                    //enable pwm mode on timer
                    //timer.ccmr1.modify(|_,w| w.oc1m().pwm1());
                    unsafe{ timer.ccmr2_output.modify(|_,w| w.oc4m().bits(6)) };

                    // enable output, high polarity
                    timer.ccer.modify(|_, w| unsafe{ w.cc4e().bits(1)
                                               .cc4p().bits(0)});

                    //enable timer
                    timer.dier.write(|w| unsafe { w.uie().bits(1) });
                    timer.cr1.write(|w| w.opm().continuous());
                }
                // Valid pins are PA0,1,2,3 on timer 2
                //                PA6,7 PB0,1 on timer 3
                //                PB6,7,8,9 on timer 4
                //                PA8,9,10 on timer 1
                match self.pin {
                    0 => self.port.crl.modify(|_,w| w.mode0().output50()
                                                        .cnf0().alt_push()),
                    1 => self.port.crl.modify(|_,w| w.mode1().output50()
                                                        .cnf1().alt_push()),
                    2 => self.port.crl.modify(|_,w| w.mode2().output50()
                                                        .cnf2().alt_push()),
                    3 => self.port.crl.modify(|_,w| w.mode3().output50()
                                                        .cnf3().alt_push()),
                    4 => self.port.crl.modify(|_,w| w.mode4().output50()
                                                        .cnf4().alt_push()),
                    5 => self.port.crl.modify(|_,w| w.mode5().output50()
                                                        .cnf5().alt_push()),
                    6 => self.port.crl.modify(|_,w| w.mode6().output50()
                                                        .cnf6().alt_push()),
                    7 => self.port.crl.modify(|_,w| w.mode7().output50()
                                                        .cnf7().alt_push()),
                    8 => self.port.crh.modify(|_,w| w.mode8().output50()
                                                        .cnf8().alt_push()),
                    9 => self.port.crh.modify(|_,w| w.mode9().output50()
                                                        .cnf9().alt_push()),
                    10 => self.port.crh.modify(|_,w| w.mode10().output50()
                                                        .cnf10().alt_push()),
                    11 => self.port.crh.modify(|_,w| w.mode11().output50()
                                                        .cnf11().alt_push()),
                    12 => self.port.crh.modify(|_,w| w.mode12().output50()
                                                        .cnf12().alt_push()),
                    13 => self.port.crh.modify(|_,w| w.mode13().output50()
                                                        .cnf13().alt_push()),
                    14 => self.port.crh.modify(|_,w| w.mode14().output50()
                                                        .cnf14().alt_push()),
                    15 => self.port.crh.modify(|_,w| w.mode15().output50()
                                                        .cnf15().alt_push()),
                    _ => {},
                }
            },
        };
    }
}

impl<'a> halPin<u16> for Pin<'a>{
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

    fn analog_read(&self) -> u16 {
        1
    }

    fn analog_write(&self, duty_cycle: u16){
        if let Some(timer) = self.timer {
            timer.ccr4.write(|w| unsafe{ w.ccr4().bits(duty_cycle) });;
        }
    }
}