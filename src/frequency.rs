//! System and Hardware Clocks

/// Internal Oscillator
pub const HSI: u32 = 8_000_000;
/// External clock
pub const HSE: u32 = 8_000_000;
/// Low Speed bus
pub const APB1: u32 = 36_000_000;

use stm32f103xx::{Rcc, Flash};

/// Preconfigured clock speed options
/// enum fields can't start with a number makes the names not look as good
pub enum Speed {
    /// 8Mhz
    S8Mhz, 
    /// 16 Mhz
    S16Mhz, 
    /// 32Mhz
    S32Mhz, 
    /// 72 Mhz
    S72Mhz,}

/// Initializes SYSCLK to 72Mhz
pub fn init(rcc: &Rcc, flash: &Flash, speed: Speed) {
    // enable external clock
    rcc.cr.modify(|_,w| w.hseon().enabled());
    while rcc.cr.read().hserdy().is_notready() {}

    // configure pll to external clock
    rcc.cfgr.modify(|_,w| w.pllsrc().external());

    // enable flash prefetch buffer
    flash.acr.modify(|_,w| w.prftbe().enabled());

    match speed {
        Speed::S8Mhz => {},
        Speed::S16Mhz => {
            rcc.cfgr.modify(|_,w| w.pllmul().mul2());
            use_pll(rcc);
        }
        Speed::S32Mhz => {
            rcc.cfgr.modify(|_,w| w.pllmul().mul4());

            // set flash latency to one
            flash.acr.modify(|_,w| w.latency().one());
            use_pll(rcc);
        }
        Speed::S72Mhz => {
            rcc.cfgr.modify(|_,w| w.pllmul().mul9());

            // set apb1 to hclk / 2
            rcc.cfgr.modify(|_,w| w.ppre1().div2());

            // set flash latency to two
            flash.acr.modify(|_,w| w.latency().two());
            use_pll(rcc);
        }
    }
}

fn use_pll(rcc: &Rcc) {
    // enable pll
    rcc.cr.modify(|_,w| w.pllon().enabled());

    while rcc.cr.read().pllrdy().is_unlocked() {}

    // set system clock to pll
    rcc.cfgr.modify(|_,w| w.sw().pll());
}