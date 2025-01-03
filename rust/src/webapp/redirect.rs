//! A simple redirect from port 80 to 443
use std::future::Future;
use anyhow::{Context, Result};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use axum::{
    handler::HandlerWithoutStateExt,
    extract::Host,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError,
};

/// The address for the webserver redirect
pub const ADDR: &'static str = if cfg!(debug_assertions) { "127.0.0.1" } else { "0.0.0.0" };
/// The insecure (http) port
const HTTP_PORT: &'static str = if cfg!(debug_assertions) { "8080" } else { "80" };
/// The secure (https) port
pub const HTTPS_PORT: &'static str = if cfg!(debug_assertions) { "8443" } else { "443" };

pub async fn redirect_http_to_https<F>(signal: F) -> Result<()> 
where 
    F: Future<Output = ()> + Send + 'static
{
    fn make_https(host: String, uri: Uri) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(HTTP_PORT, HTTPS_PORT);
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(_error) => {
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = ADDR.to_string() + ":" + HTTP_PORT;
    info!("Starting redirect servce on {ADDR} from http port {HTTP_PORT} to https port {HTTPS_PORT}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, redirect.into_make_service())
    .with_graceful_shutdown(signal)
    .await
    .context("Redirect service failed")?;

    Ok(())
}
