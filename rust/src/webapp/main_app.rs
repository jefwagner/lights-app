use anyhow::{Context, Result};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

use crate::lights::{LightsCommand, LightsRemote};

pub async fn build(remote: &LightsRemote) -> Result<Router> {
    trace!("Building Axum web-app objects");
    // clone the remote to pass to router
    let remote_on = remote.clone();
    let remote_off = remote.clone();
    // read in the html for a single page applications
    let index = tokio::fs::read_to_string("www/index.html")
        .await
        .context("Failed to read index.html")?;

    // build our application
    Ok(Router::new()
        // default route
        .route("/", get(|| async { Html(index).into_response() }))
        // static files
        .nest_service("/assets", ServeDir::new("www/assets"))
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
