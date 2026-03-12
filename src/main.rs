mod app;
mod content;
mod handler;
mod terminal;
mod theme;
mod ui;

use std::sync::Arc;

use russh::keys::ssh_key::rand_core::OsRng;

use handler::AppServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(2222);

    let key = if let Ok(key_pem) = std::env::var("SSH_HOST_KEY") {
        log::info!("Loading SSH host key from SSH_HOST_KEY env var...");
        russh::keys::decode_secret_key(&key_pem, None)
            .expect("Failed to decode SSH_HOST_KEY")
    } else {
        log::info!("No SSH_HOST_KEY set, generating ephemeral host key...");
        russh::keys::PrivateKey::random(&mut OsRng, russh::keys::Algorithm::Ed25519)
            .expect("Failed to generate host key")
    };

    let config = russh::server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(300)),
        auth_rejection_time: std::time::Duration::from_secs(1),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![key],
        nodelay: true,
        ..Default::default()
    };

    let mut server = AppServer::new();

    log::info!("SSH portfolio server listening on 0.0.0.0:{port}");
    log::info!("Connect with: ssh localhost -p {port}");

    server.run(Arc::new(config), ("0.0.0.0", port)).await?;

    Ok(())
}
