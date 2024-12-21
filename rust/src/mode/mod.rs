use anyhow::Result;
use serde::Serialize;
use dyn_clone::DynClone;

mod params;
pub use params::{Param, Value, Meta};

mod solid;

pub trait LightsMode: DynClone {
    // get the mode name
    fn name(&self) -> &'static str;

    // start the lights mode
    fn start(&mut self) -> Result<Vec<Param>>;

    // query the parameters
    fn params(&self) -> Result<Vec<Param>>;

    // stop the lights mode
    fn stop(&mut self) -> Result<()>;

    // update the parameters for the lights mode
    fn update(&mut self, params: Vec<Param>) -> Result<()>;
}

impl Serialize for dyn LightsMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(self.name())
    }
}

dyn_clone::clone_trait_object!(LightsMode);
