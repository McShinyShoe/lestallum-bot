use azalea::{Client, client_chat::ChatPacket};
use shared::prelude::*;

use crate::{bot_state::State, chat_event::ChatEvent};

pub async fn chat_event_handler(
    client: Client,
    event: ChatEvent,
    chat_packet: ChatPacket,
    state: State,
) -> Result<()> {
    match event {
        ChatEvent::DirectMessage {
            sender,
            receiver,
            message,
        } => {
            client.chat(format!("/msg {} OK: {}", sender, message));
        }
        ChatEvent::Unknown => {}
    }
    Ok(())
}
