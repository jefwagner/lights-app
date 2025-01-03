use std::time::Duration;
use tokio::sync::{mpsc, watch};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use log::LevelFilter;

mod mylog;
mod appstate;
use appstate::{AppState, AppStateChange};
mod lights;
use lights::{
    LightsController, 
    modes::{
        SetDriver,
    }
};
mod webapp;

const CHANNEL_SIZE: usize = 10;

#[tokio::main]
async fn main() {
    // initilize logging 
    // - swap log_level define if you need to override default filter level
    // let log_level = if cfg!(debug_assertions) { LevelFilter::Info } else { LevelFilter::Debug };
    let log_level = LevelFilter::Debug;
    if let Err(e) = mylog::init_log(log_level) {
        eprintln!("Unable to initialize logging: {e:?}");
    }
 
    // create the watch (single producer, multiple consumer) ipc channel. This is
    // for the light controller to send updates to the webapp websocket clients
    let (state_tx, start_rx) = watch::channel(AppState::init());
    // create the multipel producer, single consumer ipc channel. This is for the 
    // websocket clients to send commands to the lights controller
    let (cmd_tx, cmd_rx) = mpsc::channel::<AppStateChange>(CHANNEL_SIZE);

    let (mut lights_controller, mut lights_modes) = LightsController::new(state_tx, cmd_rx).expect("foo");
    lights_modes.register(SetDriver::new());

    // create handle for the axum server
    let app_handle = axum_server::Handle::new();
    // create the signal handler for safe-shutdown
    let signal = webapp::shutdown_signal(app_handle.clone());
    // start the redirect server
    let _redirect_task = tokio::spawn(webapp::redirect_http_to_https(signal));
    // Start the server
    let _webapp_task = tokio::spawn(async move { 
        if let Err(e) = webapp::start(app_handle.clone()).await {
            error!("Webapp failed with error: {e:?}");
        }
    });
    // Start the lights controller
    if let Err(e) = lights_controller.run(lights_modes).await {
        error!("Lights controller failed with error: {e:?}");
    }
}

