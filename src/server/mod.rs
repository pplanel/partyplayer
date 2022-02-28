use std::error::Error;

use tokio::net::TcpListener;
use tokio::sync::mpsc;

use manager::ServerManager;

mod manager;
mod rest;
mod shutdown;
mod websocket;

pub enum ServerEvents {
    Connected,
    Onboard { client_id: String, addr: String },
    Shutdown,
}

pub struct Server {
    manager: ServerManager,
    wss_server: websocket::Server,
    _rest_server: rest::RestServer,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        let (server_sender, server_recv) = mpsc::channel::<ServerEvents>(32);

        let manager = ServerManager::new(server_recv);
        let websocker_server = websocket::Server::new(listener, server_sender);

        Server {
            manager,
            wss_server: websocker_server,
            _rest_server: rest::RestServer {},
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let (_manager, _wss_server) = tokio::join!(
            self.wss_server.run(tokio::signal::ctrl_c()),
            self.manager.run(),
        );

        Ok(())
    }
}
