//! System and Hardware Clocks

/// Internal Oscillator
pub const HSI: u32 = 8_000_000;
/// External clock
pub const HSE: u32 = 8_000_000;

use stm32f103xx::{Rcc, Flash, rcc};

/// Board clock speeds
pub struct ClockSpeeds{
    /// System Clock
    pub sysclk: u32,
    /// AHB peripheral clock
    pub hclk: u32,
    /// Low speed bus
    pub apb1: u32,
    /// high speed bus
    pub apb2: u32,
}

impl ClockSpeeds {
    /// Get clock speeds
    pub fn get(rcc: &Rcc) -> ClockSpeeds {
        let sysclk = match rcc.cfgr.read().sws(){
            rcc::cfgr::SwsR::Hse => HSE,
            rcc::cfgr::SwsR::Pll => Self::get_pll_speed(rcc),
            _ => HSI,
        };

        let hclk = match rcc.cfgr.read().hpre() {
            rcc::cfgr::HpreR::Div2 => sysclk / 2,
            rcc::cfgr::HpreR::Div4 => sysclk / 4,
            rcc::cfgr::HpreR::Div8 => sysclk / 8,
            rcc::cfgr::HpreR::Div16 => sysclk / 16,
            rcc::cfgr::HpreR::Div64 => sysclk / 64,
            rcc::cfgr::HpreR::Div128 => sysclk / 128,
            rcc::cfgr::HpreR::Div256 => sysclk / 256,
            rcc::cfgr::HpreR::Div512 => sysclk / 512,
            _ => sysclk,
        };

        let apb1 = match rcc.cfgr.read().ppre1() {
            rcc::cfgr::Ppre1R::Div2 => hclk / 2,
            rcc::cfgr::Ppre1R::Div4 => hclk / 4,
            rcc::cfgr::Ppre1R::Div8 => hclk / 8,
            rcc::cfgr::Ppre1R::Div16 => hclk / 16,
            _ => hclk,
        };

        let apb2 = match rcc.cfgr.read().ppre2() {
            rcc::cfgr::Ppre2R::Div2 => hclk / 2,
            rcc::cfgr::Ppre2R::Div4 => hclk / 4,
            rcc::cfgr::Ppre2R::Div8 => hclk / 8,
            rcc::cfgr::Ppre2R::Div16 => hclk / 16,
            _ => hclk,
        };

        ClockSpeeds{
            sysclk: sysclk,
            hclk: hclk,
            apb1: apb1,
            apb2: apb2,
        }
    }

    fn get_pll_speed(rcc: &Rcc) -> u32 {
        let hse_div = match rcc.cfgr.read().pllxtpre(){
            rcc::cfgr::PllxtpreR::Div1 => 1,
            rcc::cfgr::PllxtpreR::Div2 => 2,
        };

        let src_spd = match rcc.cfgr.read().pllsrc(){
            rcc::cfgr::PllsrcR::Internal => HSI / 2,
            rcc::cfgr::PllsrcR::External => HSE / hse_div,
        };

        match rcc.cfgr.read().pllmul() {
            rcc::cfgr::PllmulR::Mul2 => src_spd * 2,
            rcc::cfgr::PllmulR::Mul3 => src_spd * 3,
            rcc::cfgr::PllmulR::Mul4 => src_spd * 4,
            rcc::cfgr::PllmulR::Mul5 => src_spd * 5,
            rcc::cfgr::PllmulR::Mul6 => src_spd * 6,
            rcc::cfgr::PllmulR::Mul7 => src_spd * 7,
            rcc::cfgr::PllmulR::Mul8 => src_spd * 8,
            rcc::cfgr::PllmulR::Mul9 => src_spd * 9,
            rcc::cfgr::PllmulR::Mul10 => src_spd * 10,
            rcc::cfgr::PllmulR::Mul11 => src_spd * 11,
            rcc::cfgr::PllmulR::Mul12 => src_spd * 12,
            rcc::cfgr::PllmulR::Mul13 => src_spd * 13,
            rcc::cfgr::PllmulR::Mul14 => src_spd * 14,
            rcc::cfgr::PllmulR::Mul15 => src_spd * 15,
            rcc::cfgr::PllmulR::Mul16 => src_spd * 16,
            _ => src_spd,
        }
    }
}

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

/// Initializes SYSCLK to requested speed
/// uses external clock for all speeds
pub fn init(rcc: &Rcc, flash: &Flash, speed: Speed) {
    // enable external clock
    rcc.cr.modify(|_,w| w.hseon().enabled());
    while rcc.cr.read().hserdy().is_notready() {}

    // configure pll to external clock
    rcc.cfgr.modify(|_,w| w.pllsrc().external());

    // enable flash prefetch buffer
    flash.acr.modify(|_,w| w.prftbe().enabled());

    match speed {
        Speed::S8Mhz => rcc.cfgr.modify(|_,w| w.sw().hse()),
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