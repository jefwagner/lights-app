use anyhow::Result;
use tokio::task::JoinHandle;

use crate::lights::{LedColor, LightsRemote};

use super::{LightsMode, Param, Value, Meta};

struct SolidMode {
    remote: LightsRemote,
    color: LedColor,
    task: Option<JoinHandle<()>>,
}

impl LightsMode for SolidMode {
    fn new(remote: LightsRemote) -> Self {
        // read params from file
        // set values from param
        SolidMode { remote, color: LedColor { r: 128, g: 0, b: 0 }, task: None }
    }

    fn params(&self) -> Result<Vec<Param>> {
        let p = Param{ name: "color".into(), value: Value::Color(self.color), meta: Some(Meta::Color)};
        Ok(vec![p])
    }

    fn start(&mut self) -> Result<Vec<Param>> {
        self.task = Some(tokio::spawn( async move {

        }));
        self.params()
    }

    fn stop(&mut self) -> Result<()> {
        if let Some(task) = self.task.take() {
            // send stop command
            // wait for task to finish
            // send off-command
        }
        Ok(())
    }

    fn update(&mut self, params: Vec<Param>) -> Result<()> {
        Ok(())
    }
}
