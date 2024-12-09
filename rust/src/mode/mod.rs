use anyhow::Result;

mod params;
pub use params::{Param, Value, Meta};

use crate::lights::LightsRemote;

mod solid;

pub trait LightsMode {
    //
    fn new(remote: LightsRemote) -> impl LightsMode;

    // start the lights mode
    fn start(&mut self) -> Result<Vec<Param>>;

    // query the parameters
    fn params(&self) -> Result<Vec<Param>>;

    // stop the lights mode
    fn stop(&mut self) -> Result<()>;

    // update the parameters for the lights mode
    fn update(&mut self, params: Vec<Param>) -> Result<()>;
}