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
            if (receiver == client.profile().name) {
                let answer = if let Some(claude_controller) = state.claude_controller {
                    let mut cc = claude_controller.lock().await;
                    cc.ask(&sender, &message).await?
                } else {
                    None
                };
                let claude_answer = answer.unwrap_or(String::from("Hi"));
                tracing::info!("Got reply from claude: {}", &claude_answer);
                let lines = wrap_text(claude_answer.as_str(), 240);
                for (i, line) in lines.iter().enumerate() {
                    let chat = format!("/msg {} {}", sender, line);
                    tracing::info!("Sending: {}", &chat);
                    client.chat(chat);
                    client.wait_ticks(20).await;
                }
            }
        }
        ChatEvent::Unknown => {}
    }
    Ok(())
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split(' ') {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.len() + 1 + word.len() <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
