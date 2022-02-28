use std::error::Error;

use log::info;

pub struct RestServer {}

impl RestServer {
    fn new() -> Self {
        RestServer {}
    }
    async fn run(self) -> Result<(), Box<dyn Error>> {
        info!("WebsocketServer running");
        Ok(())
    }
}
