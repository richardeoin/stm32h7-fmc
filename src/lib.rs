//! Hardware Abstraction Layer for Flexible Memory Controller (FMC) on
//! STM32H7
//!
//! Currently only SDRAM functions are implemented.
//!
//! This crate depends on the GPIO, Clock and Delay functionality from
//! stm32h7xx-hal
//!
//! # SDRAM
//!
//! The H7 supports up to 2 external SDRAM devices. This library
//! currently only supports 1, although it may be on either bank 1 or
//! 2.
//!
//! ## IO Setup
//!
//! IO is constructed by configuring each pin as high speed and
//! assigning to the FMC block (usually AF12).
//!
//! ```rust
//!     let pa0 = gpioa.pa0.into_push_pull_output()
//!         .set_speed(Speed::VeryHigh)
//!         .into_alternate_af12()
//!         .internal_pull_up(true);
//! ```
//!
//! Then contruct a PinSdram type from the required pins. They must be
//! specified in the order given here.
//!
//! ```rust
//!     let fmc_io = stm32h7_fmc::PinsSdramBank1(
//!         (
//!             // A0-A11
//!             pa0, ...
//!             // BA0-BA1
//!             // D0-D31
//!             // NBL0 - NBL3
//!             // SDCKE
//!             // SDCLK
//!             // SDNCAS
//!             // SDNE
//!             // SDRAS
//!             // SDNWE
//!         )
//!     );
//! ```
//!
//! See the [examples](examples) for an ergonomic method using macros.
//!
//! ## Usage
//!
//! First create a new SDRAM from the FMC peripheral, IO and SDRAM
//! device constants.
//!
//! ```rust
//!     let mut sdram =
//!         stm32h7_fmc::Sdram::new(dp.FMC, fmc_io, is42s32800g_6::Is42s32800g {});
//! ```
//!
//! Then initialise the controller and the SDRAM device. Convert the
//! raw pointer to a sized slice using `from_raw_parts_mut`.
//!
//!
//! ```rust
//!     let ram = unsafe {
//!         // Initialise controller and SDRAM
//!         let ram_ptr = sdram.init(&mut delay, ccdr.clocks);
//!
//!         // 32 MByte = 256Mbit SDRAM
//!         slice::from_raw_parts_mut(ram_ptr, 32 * 1024 * 1024)
//!     };
//! ```
//!
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution
//! intentionally submitted for inclusion in the work by you, as
//! defined in the Apache-2.0 license, shall be dual licensed as
//! above, without any additional terms or conditions.
//!
#![no_std]
// rustc lints.
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[cfg(feature = "log")]
#[macro_use(trace)]
extern crate log;

#[macro_use]
mod macros;

use stm32h7xx_hal::hal;
use stm32h7xx_hal::stm32;

mod fmc;
pub use fmc::{PinsSdramBank1, PinsSdramBank2};

mod sdram;
pub use sdram::Sdram;

mod is42s32800g;
pub use is42s32800g::*;
mod is42s16400j;
pub use is42s16400j::*;
