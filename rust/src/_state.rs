use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use axum::response::{Json, IntoResponse};

use crate::lights::{LedColor, LedDriver};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct LightsState {
    on: bool,
    color: LedColor,
}

#[derive(Clone)]
pub struct LightsStateRef{
    pub state: Arc<RwLock<LightsState>>,
    pub controller: Arc<RwLock<LedDriver>>
}

impl LightsStateRef {
    pub fn new(left: usize, right: usize) -> Self {
        LightsStateRef{
            state: Arc::new(RwLock::new(LightsState { on: true, color: (64,0,0).into() })),
            controller: Arc::new(RwLock::new(LedDriver::new(left, right))),
        }
    }

    pub async fn get_state(&self) -> LightsState {
        *self.state.read().await
    }

    pub async fn set_off(&mut self) {
        self.controller.write().await.clear();
        self.state.write().await.on = true;        
    }

    pub async fn set_on(&mut self) {
        let color = self.state.read().await.color;
        self.controller.write().await.fill(color);
        self.state.write().await.on = true;
    }

    pub async fn set_color_on<A: Copy + Into<LedColor>>(&mut self, color: A) {
        self.controller.write().await.fill(color.into());
        let mut state = self.state.write().await;
        state.color = color.into();
        state.on = true;
    }

    pub async fn respond(&self) -> impl IntoResponse {
        let state = self.get_state().await;
        let json = serde_json::to_string(&state).unwrap();
        Json(json).into_response()
    }
}