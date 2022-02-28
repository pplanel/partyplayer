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
    WebsocketConnected,
    RestConnected,
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
        let rest_sender = server_sender.clone();

        let manager = ServerManager::new(server_recv);
        let websocker_server = websocket::Server::new(listener, server_sender);
        let rest_server = rest::RestServer::new(rest_sender);

        Server {
            manager,
            wss_server: websocker_server,
            rest_server,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let (_rest, _manager, _wss_server) = tokio::join!(
            self.wss_server.run(tokio::signal::ctrl_c()),
            self.manager.run(),
            self.rest_server.run()
        );

        Ok(())
    }
}
