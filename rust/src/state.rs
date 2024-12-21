use serde::Serialize;

use crate::{lights::LightsRemote, mode::{LightsMode, Param}};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    #[serde(skip)]
    pub remote: LightsRemote,
    pub on_off: bool,
    pub modes: Vec<Box<dyn LightsMode>>,
    pub selected: String,
    pub params: Vec<Param>    
}