mod config;
mod http;

use std::{env, error::Error, process::ExitCode};

use tokio::{net::TcpListener, sync::oneshot};
use tracing_subscriber::EnvFilter;

const USAGE: &str = "Usage: palaver serve\n\nPALAVER_PORT sets the loopback port (default: 8080).\nRUST_LOG sets the console log filter (default: info).";

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("palaver: {error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args = env::args_os().skip(1);
    match (args.next(), args.next()) {
        (Some(command), None) if command == "--help" || command == "-h" => {
            println!("{USAGE}");
            return Ok(());
        }
        (Some(command), None) if command == "serve" => {}
        _ => return Err(format!("expected the serve command\n\n{USAGE}").into()),
    }

    let address = config::listen_address(env::var_os("PALAVER_PORT").as_deref())?;
    let filter = match env::var("RUST_LOG") {
        Ok(value) => EnvFilter::try_new(value)
            .map_err(|error| format!("invalid RUST_LOG filter: {error}"))?,
        Err(env::VarError::NotPresent) => EnvFilter::new("info"),
        Err(error) => return Err(format!("could not read RUST_LOG: {error}").into()),
    };
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()?;

    let listener = TcpListener::bind(address)
        .await
        .map_err(|error| format!("could not listen on {address}: {error}"))?;
    tracing::info!(%address, "listening");

    let (shutdown, signal) = oneshot::channel();
    let server = axum::serve(listener, http::router()).with_graceful_shutdown(async move {
        let _ = signal.await;
    });
    let mut server = std::pin::pin!(std::future::IntoFuture::into_future(server));

    tokio::select! {
        result = tokio::signal::ctrl_c() => {
            result.map_err(|error| format!("could not wait for Ctrl+C: {error}"))?;
            tracing::info!("shutting down");
            let _ = shutdown.send(());
            server.await?;
        }
        result = server.as_mut() => result?,
    }
    Ok(())
}
