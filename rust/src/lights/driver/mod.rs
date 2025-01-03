#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use super::LedColor;

mod config;
pub use config::DriverConfig;

#[cfg(not(debug_assertions))]
mod led_driver;
#[cfg(not(debug_assertions))]
pub use led_driver::LedDriver;

#[cfg(debug_assertions)]
mod test_driver;
#[cfg(debug_assertions)]
pub use test_driver::LedDriver;

/// Iterator object for the Lights Driver
pub struct LedIterator<'a> {
    lc: &'a mut LedDriver,
    index: usize,
}
