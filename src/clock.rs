//! Set Clock Speed

use stm32f103xx::{Rcc, Flash};

/// Initializes SYSCLK to 72Mhz
pub fn init(rcc: &Rcc, flash: &Flash) {
    // enable external clock
    rcc.cr.modify(|_,w| w.hseon().enabled());
    while rcc.cr.read().hserdy().is_notready() {}

    // configure pll to external clock * 9
    rcc.cfgr.modify(|_,w| w.pllsrc().external());
    rcc.cfgr.modify(|_,w| w.pllmul().mul9());

    // set apb1 to hclk / 2
    rcc.cfgr.modify(|_,w| w.ppre1().div2());

    // enable flash prefetch buffer
    flash.acr.modify(|_,w| w.prftbe().enabled());

    // set flash latency to two
    flash.acr.modify(|_,w| w.latency().two());

    // enable pll
    rcc.cr.modify(|_,w| w.pllon().enabled());

    while rcc.cr.read().pllrdy().is_unlocked() {}

    // set system clock to pll
    rcc.cfgr.modify(|_,w| w.sw().pll());
}