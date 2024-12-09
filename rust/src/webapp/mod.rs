use std::path::PathBuf;

use anyhow::{Context, Result};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use axum_server::{
    Handle,
    tls_rustls::RustlsConfig
};

use crate::lights::LightsRemote;

mod redirect;
pub use redirect::redirect_http_to_https;

mod shutdown;
pub use shutdown::shutdown_signal;

mod main_app;

/// start the web-app by starting the web-server
pub async fn start(handle: Handle, remote: &LightsRemote) -> Result<()> {
    debug!("Starting the lights controller web-server");

    // set up the TLS config
    let config = RustlsConfig::from_pem_file(
        PathBuf::from("secrets/lights.crt"),
        PathBuf::from("secrets/lights.key"),
    )
    .await
    .context("Failed to load TLS config")?;

    let app = main_app::build(remote).await?;

    axum_server::bind_rustls(([0,0,0,0],443).into(), config)
    .handle(handle)
    .serve(app.into_make_service())
    .await.context("Error starting web-server")?;

    Ok(())
}