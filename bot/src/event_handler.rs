use std::time::Duration;

use azalea::prelude::*;
use azalea::{Client, Event};
use shared::bot_status::BotStatus;
use shared::prelude::*;

use crate::bot_state::State;

pub async fn event_handler(client: Client, event: Event, state: State) -> Result<()> {
    match event {
        Event::Login => {
            tracing::info!("Bot logged in as {}", client.profile().name);
            {
                let mut status = state.status.write().await;
                *status = BotStatus::Starting;
            }
            loop {
                let mut status = state.status.read().await;
                if *status == BotStatus::Starting {
                    client.chat("/server towny");
                    break;
                }
                client.wait_ticks(200);
            }
        }
        Event::Chat(chat_packet) => {
            if (chat_packet.content() == format!("[+] {}", client.profile().name)) {
                tracing::info!("Logged in on towny");
                let mut status = state.status.write().await;
                *status = BotStatus::Idle;
            };
        }
        Event::Disconnect(reason) => {
            tracing::info!("Disconnected{}", {
                if let Some(reason_text) = reason {
                    format!(": {}", reason_text)
                } else {
                    "".to_string()
                }
            });
            let mut status = state.status.write().await;
            *status = BotStatus::Disconnected;
        }
        _ => {}
    }
    Ok(())
}
