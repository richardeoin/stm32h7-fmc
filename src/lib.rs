//! ## This crate has been replaced by [FMC support within
//! stm32h7xx-hal](https://docs.rs/stm32h7xx-hal/latest/stm32h7xx_hal/fmc/index.html).
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
//! [`stm32h7xx-hal`]: https://crates.io/crates/stm32h7xx-hal
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
