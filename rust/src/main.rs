use log::LevelFilter;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

mod mylog;

mod lights;
use lights::{DriverConfig, new_lights};

mod webapp;

mod mode;

#[tokio::main]
async fn main() {
    // initilize logging
    if let Err(e) = mylog::init_log(LevelFilter::Debug) {
        eprintln!("Unable to initialize logging: {e:?}");
    }

    // create lights object as part of our state
    let config = DriverConfig {left: 100, right: 300, brightness: 255};
    let (lights_remote, mut lights_controller) = new_lights(config);

    // create handle for the axum server
    let app_handle = axum_server::Handle::new();
    // create the signal handler
    let signal = webapp::shutdown_signal(app_handle.clone(), lights_remote.clone());
    // start the redirect server
    let _redirect_task = tokio::spawn(webapp::redirect_http_to_https(signal));
    // Start the server
    let _webapp_task = tokio::spawn(async move { webapp::start(app_handle.clone(), &lights_remote).await });

    // start the lights task in the main loop this handles the LED driver, which
    // is a bare pointer and can't be moved (easily... by me... cause I'm not
    // good with handling pointers in rust)
    if let Err(e) = lights_controller.start().await {
        error!("Error with lights controller: {e:?}");
    }
 
}

