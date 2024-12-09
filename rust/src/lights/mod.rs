#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use tokio::sync::mpsc;

mod color;
pub use color::LedColor;

mod driver;
use driver::LedDriver;
pub use driver::DriverConfig;

mod controller;
pub use controller::{LightsCommand, LightsRemote, LightsController};

/// Create a remote and controller task
/// 
/// This creates a controller for the lights. The controller owns the driver that
/// controls the LEDs, which is a bare pointer and not SYNC - so the start command
/// can only be run with tokio::task::spawn_local
/// 
/// The remote is a wrapper around an MPSC sender for sending commands to the
/// controller and is cloneable and safe to use in multiple threads.
pub fn new_lights(config: DriverConfig) -> (LightsRemote, LightsController) {
    trace!("Creating the lights remote and controller");
    let (sender, receiver) = mpsc::channel(10);
    let remote = LightsRemote::new(sender);
    let controller = LightsController::new(config, receiver);
    (remote, controller)
}


