use async_trait::async_trait;

use crate::lights::{DriverConfig, Param};
use super::{LightsMode, MODES_PARAMS_DIR, DriverCommand};

pub struct SetDriver {
    brightness: u8,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,    
    r: u8,
    g: u8,
    b: u8,
}

impl SetDriver {
    pub fn new() -> Self {
        SetDriver { brightness: 255, red_shift: 0, green_shift: 0, blue_shift: 0, r: 255, g: 0, b: 0 }
    }
}

#[async_trait]
impl LightsMode for SetDriver {
    async fn init(&mut self) -> DriverCommand {
        // read file
        DriverCommand::Clear
    }

    fn get_name(&self) -> String {
        "Configure Driver".into()
    }

    /// Get a list of current parameters
    fn get_params(&self) -> Vec<Param>{
        vec![]
    }

    /// Turn the lights on (true) or off (false)
    async fn on_off(&mut self, on_off: bool) -> DriverCommand {
        DriverCommand::Clear
    }

    /// Update a single mode parameter
    async fn update_param(&mut self, param: Param) -> (Option<DriverConfig>, DriverCommand) {
        (None, DriverCommand::Clear)
    }

    /// Advance the mode one tick
    async fn tick(&mut self) -> DriverCommand {
        DriverCommand::Clear
    }

    /// Stop the lights mode
    async fn stop(&mut self) {
        // save file
    }
}