use anyhow::{Context, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use axum::{
    response::{Html, IntoResponse},
    routing::{any, get},
    Router,
};
use tower_http::services::ServeDir;

use super::ws_handler;
use crate::{lights::{LightsCommand, LightsRemote}, mode::{LightsMode, Param}};


pub async fn build(remote: &LightsRemote) -> Result<Router> {
    trace!("Building Axum web-app objects");
    // clone the remote to pass to router
    let remote_on = remote.clone();
    let remote_off = remote.clone();

    let static_index = tokio::fs::read_to_string("www/index.html").await
    .context("Failed to read index.html")?;

    // build our application
    Ok(Router::new()
        // static files
        .nest_service("/assets", ServeDir::new("www/assets"))
        // default route to server main page:
        .route("/", get(|| async move { 
            // read in the html for a single page applications
            let index = tokio::fs::read_to_string("www/index.html").await
            .unwrap_or(static_index);
            Html(index).into_response()
        }))
        // websocket handler
        .route("/ws", any(ws_handler))
        // quick and dirty on-off switch
        .route(
            "/on",
            get(|| async move { remote_on.send(LightsCommand::On).await.unwrap() }),
        )
        .route(
            "/off",
            get(|| async move { remote_off.send(LightsCommand::Off).await.unwrap() }),
        ))
}
