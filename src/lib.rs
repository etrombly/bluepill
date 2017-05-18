//! Board Support Crate for the bluepill
//!
//! # Usage
//!
//! Follow `cortex-m-quickstart` [instructions][i] but remove the `memory.x`
//! linker script and the `build.rs` build script file as part of the
//! configuration of the quickstart crate. Additionally, uncomment the "if using
//! ITM" block in the `.gdbinit` file.
//!
//! [i]: https://docs.rs/cortex-m-quickstart/0.1.1/cortex_m_quickstart/
//!
//! # Examples
//!
//! Check the [examples] module.
//!
//! [examples]: ./examples/index.html

#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]
#![feature(associated_type_defaults)]

extern crate cast;
pub extern crate stm32f103xx;

// For documentation only
pub mod examples;

pub mod led;
//pub mod serial;
pub mod timer;
pub mod clock;
pub mod pin;

pub mod frequency;
