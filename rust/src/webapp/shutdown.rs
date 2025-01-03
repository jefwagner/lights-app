use std::time::Duration;
use anyhow::{Context, anyhow};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use tokio::signal;
use axum_server::Handle;

// use crate::lights::{LightsCommand, LightsRemote};

pub async fn shutdown_signal(handle: Handle) {
    let mut sigterm = match signal::unix::signal(signal::unix::SignalKind::terminate()) {
        Ok(sigterm) => {sigterm},
        Err(e) => {
            error!("Failed to install sigterm handler: {e:?}");
            return;
        }
    };

    info!("Starting shutdown signal monitor");
    let res = tokio::select! {
        res = signal::ctrl_c() => {res.context("Error with ctrl-c handler.")},
        res = sigterm.recv() => {res.ok_or(anyhow!("Something happend with sigterm"))},
    };
    if let Err(e) = res {
        error!("Failed to install signal handlers: {e:?}");
    }

    info!("Received termination signal shutting down");
    // let _ = remote.send(LightsCommand::Stop).await;
    handle.graceful_shutdown(Some(Duration::from_secs(10)));
}
