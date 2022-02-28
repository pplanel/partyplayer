use std::borrow::Borrow;
use std::error::Error;

use log::info;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};

use manager::ServerManager;

mod manager;
mod rest;
mod shutdown;
mod websocket;

#[allow(dead_code)]
pub enum ServerEvents {
    Connected,
    Onboard { client_id: String, addr: String },
    Shutdown,
}

pub struct Server {
    manager: ServerManager,
    wss_server: websocket::Server,
    rest_server: rest::RestServer,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        let (server_sender, server_recv) = mpsc::channel::<ServerEvents>(32);

        let (kill_sender, kill_recv) = oneshot::channel::<ServerEvents>();

        let manager = ServerManager::new(server_recv, kill_sender);
        let websocker_server = websocket::Server::new(listener, server_sender);

        Server {
            manager,
            wss_server: websocker_server,
            rest_server: rest::RestServer {},
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let (manager, wss_server) = tokio::join!(
            self.wss_server.run(tokio::signal::ctrl_c()),
            self.manager.run(),
        );

        Ok(())
    }
}
