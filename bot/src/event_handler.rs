use std::time::Duration;

use azalea::prelude::*;
use azalea::{Client, Event};
use shared::prelude::*;

use crate::bot_state::State;

pub async fn event_handler(client: Client, event: Event, state: State) -> Result<()> {
    if let Event::Login = event {
        tracing::info!("Bot logged in");
        tokio::time::sleep(Duration::from_secs(3)).await;
        tracing::info!("Bot disconnecting");
        client.disconnect();
        client.wait_ticks(1).await;
        client.exit();
    }
    Ok(())
}
