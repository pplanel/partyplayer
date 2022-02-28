use std::error::Error;
use std::future::Future;
use std::thread;
use std::time::Duration;

use log::{debug, error, info};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};

use crate::server::shutdown::Shutdown;
use crate::server::ServerEvents;
use crate::server::ServerEvents::Connected;

pub struct Listener<'a> {
    listener: &'a TcpListener,
    manager_chan: &'a mpsc::Sender<ServerEvents>,
    notify_shutdown: broadcast::Sender<()>,
}

impl Listener<'_> {
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let socket = self.accept().await?;
            let mut handler = Handler {
                socket,
                manager_chan: self.manager_chan.clone(),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
            };
            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!("Cnnection error {}", err);
                }
            });
        }
    }

    async fn accept(&mut self) -> Result<TcpStream, Box<dyn Error>> {
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => return Err(err.into()),
            }
        }
    }
}

pub struct Handler {
    socket: TcpStream,
    manager_chan: mpsc::Sender<ServerEvents>,
    shutdown: Shutdown,
}

impl Handler {
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while !self.shutdown.is_shutdown() {
            thread::sleep(Duration::from_secs(1));
            self.manager_chan
                .try_send(ServerEvents::Onboard {
                    client_id: "porra".to_string(),
                    addr: self.socket.peer_addr().unwrap().to_string(),
                })
                .unwrap_or_else(|_err| {
                    error!("cannot send msg");
                });
        }
        Ok(())
    }
}

pub struct Server {
    listener: TcpListener,
    manager_chan: mpsc::Sender<ServerEvents>,
}

impl<'a> Server {
    pub fn new(listener: TcpListener, manager_chan: mpsc::Sender<ServerEvents>) -> Self {
        Server {
            listener,
            manager_chan,
        }
    }

    pub async fn run(self, shutdown: impl Future) {
        let (notify_shutdown, _) = broadcast::channel(1);
        let mut server = Listener {
            listener: &self.listener,
            manager_chan: &self.manager_chan,
            notify_shutdown,
        };

        self.manager_chan.try_send(Connected).unwrap_or_else(|err| {
            debug!("cannot send message: {}", err);
        });

        tokio::select! {
            res = server.run() => {
                if let Err(err) = res {
                    error!("failed accept {}", err);
                }
            }
            _ = shutdown => {
                info!("Shutting down")
            }
        }

        let Listener {
            notify_shutdown, ..
        } = server;

        drop(notify_shutdown);
    }
}
