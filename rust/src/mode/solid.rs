use anyhow::Result;
use tokio::task::JoinHandle;

use crate::lights::{LedColor, LightsRemote};

use super::{LightsMode, Param, Value, Meta};

#[derive(Clone)]
struct SolidMode {
    remote: LightsRemote,
    color: LedColor,
}

impl SolidMode {
    fn new(remote: LightsRemote) -> Self {
        // read params from file
        // set values from param
        SolidMode { remote, color: LedColor { r: 128, g: 0, b: 0 } }
    }
}

impl LightsMode for SolidMode {
    fn name(&self) -> &'static str {
        "Solid"
    }

    fn params(&self) -> Result<Vec<Param>> {
        let p = Param{ name: "color".into(), value: Value::Color(self.color), meta: Some(Meta::Color)};
        Ok(vec![p])
    }

    fn start(&mut self) -> Result<Vec<Param>> {
        self.params()
    }

    fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, params: Vec<Param>) -> Result<()> {
        Ok(())
    }
}
