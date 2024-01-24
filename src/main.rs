use anyhow::Result;
use remote_types::telemetry;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber =
        telemetry::get_subscriber("remote_types".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let port = std::env::var("PORT").unwrap_or("3000".to_string());

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    remote_types::run(listener).await?;

    Ok(())
}
