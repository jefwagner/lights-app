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

        let https_host = host.replace("80", "443");
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

    // let addr = SocketAddr::from(([127, 0, 0, 1], ports.http));
    debug!("Starting redirect servce from http port 80 to https port 443");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, redirect.into_make_service())
    .with_graceful_shutdown(signal)
    .await
    .context("Redirect service failed")?;

    Ok(())
}
