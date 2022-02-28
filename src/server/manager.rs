use std::error::Error;

use log::info;
use tokio::sync::mpsc;

use crate::server::ServerEvents;

pub struct ServerManager {
    recv_chan: mpsc::Receiver<ServerEvents>,
}

impl ServerManager {
    pub fn new(state_recv_chan: mpsc::Receiver<ServerEvents>) -> Self {
        ServerManager {
            recv_chan: state_recv_chan,
        }
    }
    pub async fn run(mut self) -> Result<(), Box<dyn Error>> {
        info!("ServerManager running");
        while let Some(event) = self.recv_chan.recv().await {
            match event {
                ServerEvents::Connected => {
                    info!("ServerManager::Connected")
                }
                ServerEvents::Onboard { client_id, addr } => {
                    info!(
                        "ServerManager::Onboard: \nClient ID: {}\tPeer Addr: {}\n",
                        client_id, addr
                    )
                }
                ServerEvents::Shutdown => {}
            }
        }
        Ok(())
    }
}
