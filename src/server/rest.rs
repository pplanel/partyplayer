#![allow(dead_code)]

use log::debug;
use tokio::sync::mpsc::Sender;
use warp::Filter;

use crate::server::ServerEvents;
use crate::server::ServerEvents::RestConnected;

pub struct RestServer {
    pub(crate) manager_chan: Sender<ServerEvents>,
}

impl RestServer {
    pub fn new(manager_chan: Sender<ServerEvents>) -> Self {
        RestServer { manager_chan }
    }
    pub async fn run(self) {
        self.manager_chan
            .try_send(RestConnected)
            .unwrap_or_else(|err| {
                debug!("cannot send message: {}", err);
            });

        let hello = warp::path!("hello" / String).map(|name| format!("Hello {}!", name));
        warp::serve(hello).run(([127, 0, 0, 1], 9001)).await;
    }
}
