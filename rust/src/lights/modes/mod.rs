use async_trait::async_trait;

use super::{driver::DriverConfig, LedColor, Param};

mod set_driver;
pub use set_driver::SetDriver;

pub const MODES_PARAMS_DIR: &'static str = if cfg!(debug_assertions) {
    "tests/modes"
} else {
    "settings/modes"
};

pub enum DriverCommand {
    Clear,
    Fill(LedColor),
    SetAll(Vec<LedColor>),
    SetSome(Vec<(usize, LedColor)>)
}

#[async_trait]
pub trait LightsMode: Send + Sync {
    /// Initialize the lights mode
    async fn init(&mut self) -> DriverCommand;

    /// Foo
    fn get_name(&self) -> String;

    /// Get a list of current parameters
    fn get_params(&self) -> Vec<Param>;

    /// Turn the lights on (true) or off (false)
    async fn on_off(&mut self, on_off: bool) -> DriverCommand;

    /// Update a single mode parameter
    async fn update_param(&mut self, param: Param) -> (Option<DriverConfig>, DriverCommand);

    /// Advance the mode one tick
    async fn tick(&mut self) -> DriverCommand;

    /// Stop the lights mode
    async fn stop(&mut self);
}
 