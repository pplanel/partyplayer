extern crate core;

use std::io::Error;
use std::process;

use log::error;
use tokio::net::TcpListener;

use partyplayer::server::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let port = "9000";
    let listener = TcpListener::bind(&format!("127.0.0.1:{}", port)).await?;
    let server = Server::new(listener);
    server.run().await.unwrap_or_else(|err| {
        error!("Cannot run server: {}", err);
        process::exit(1);
    });
    Ok(())
}
