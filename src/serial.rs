//! Serial interface
//!
//! - TX - PA9
//! - RX - PA10

use core::ptr;

use stm32f103xx::{gpioa, GPIOA, GPIOB, Rcc, usart1, USART1, USART2, USART3};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// Serial interface
///
/// # Interrupts
///
/// - `Usart1Exti25` - RXNE (RX buffer not empty)
#[derive(Clone, Copy)]
pub struct Serial<'a>{
    /// attached usart
    pub usart: &'a usart1::RegisterBlock,
}

impl<'a> Serial<'a> {
    /// Initializes the serial interface with a baud rate of `baut_rate` bits
    /// per second
    pub fn init(self, port: &'a gpioa::RegisterBlock, rcc: &Rcc, baud_rate: u32) {
        // Power up peripherals
        // check which memory block this port is pointing to
        match &*self.usart as *const _{
            x if x == USART1.get() as *const _ => {
                rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
                port.crh.modify(|_, w| w.mode9().output()
                                        .cnf9().alt_push()
                                        .mode10().input()
                                        .cnf10().alt_push());
            },
            x if x == USART2.get() as *const _ => {
                rcc.apb1enr.modify(|_, w| w.usart2en().enabled());
                port.crl.modify(|_, w| w.mode2().output()
                                        .cnf2().alt_push()
                                        .mode3().input()
                                        .cnf3().alt_push());
            },
            x if x == USART3.get() as *const _ => {
                rcc.apb1enr.modify(|_, w| w.usart3en().enabled());
                port.crh.modify(|_, w| w.mode10().output()
                                        .cnf10().alt_push()
                                        .mode11().input()
                                        .cnf11().alt_push());
            },
            _ => {},
        }

        match &*port as *const _{
            x if x == GPIOA.get() as *const _ => rcc.apb2enr.modify(|_, w| w.iopaen().enabled()),
            x if x == GPIOB.get() as *const _ => rcc.apb2enr.modify(|_, w| w.iopben().enabled()),
            _ => {},
        }

        // 1 stop bits
        self.usart.cr2.write(|w| unsafe { w.stop().bits(0b00) });

        // Disable hardware flow control
        self.usart
            .cr3
            .write(|w| unsafe { w.rtse().bits(0).ctse().bits(0) });

        let speeds = frequency::ClockSpeeds::get(rcc);

        // set baud rate
        let brr = (speeds.apb2 / baud_rate) as u16;
        let fraction = (brr & 0b1111) as u8;
        let mantissa = brr >> 4;
        self.usart
            .brr
            .write(
                |w| unsafe {
                    w.div_fraction()
                        .bits(fraction)
                        .div_mantissa()
                        .bits(mantissa)
                },
            );
        
        // enable peripheral, transmitter, receiver
        // enable RXNE event
        self.usart
            .cr1
            .write(
                |w| unsafe {
                    w.ue()
                        .bits(1)
                        .re()
                        .bits(1)
                        .te()
                        .bits(1)
                        .pce()
                        .bits(0)
                        .m()
                        .bits(0)
                        .rxneie()
                        .bits(1)
                },
            );
    }

    /// Reads a byte from the RX buffer
    ///
    /// Returns `None` if the buffer is empty
    pub fn read(self) -> Option<u8> {
        if self.usart.sr.read().rxne().bits() == 1 {
            // NOTE(read_volatile) the register is 9 bits big but we'll only
            // work with the first 8 bits
            Some(
                unsafe {
                    ptr::read_volatile(&self.usart.dr as *const _ as *const u8)
                },
            )
        } else {
            None
        }
    }

    /// Writes byte into the TX buffer
    ///
    /// Returns `Err` if the buffer is already full
    pub fn write(self, byte: u8) -> Result<()> {
        if self.usart.sr.read().txe().bits() == 1 {
            unsafe {
                ptr::write_volatile(&self.usart.dr as *const _ as *mut u8, byte)
            }
            Ok(())
        } else {
            Err(Error { _0: () })
        }
    }
}