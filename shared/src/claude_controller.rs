use std::collections::HashMap;

use crate::prelude::*;
use claude_sdk::{ClaudeClient, ContentBlock, ConversationBuilder};

pub struct ClaudeController {
    pub client: ClaudeClient,
    pub context: String,
    pub history: HashMap<String, Vec<(String, String)>>,
}

impl ClaudeController {
    pub fn new(api_key: impl Into<String>, context: impl Into<String>) -> Result<Self> {
        let client = ClaudeClient::anthropic(api_key);
        let context = context.into();
        let history = HashMap::new();

        Ok(Self {
            client,
            context,
            history,
        })
    }
    pub async fn ask(
        self: &mut Self,
        user: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Option<String>> {
        let mut conversation = ConversationBuilder::new().with_cached_system(&self.context);

        let user = user.into();
        let message = message.into();

        if let Some(messages) = self.history.get(&user) {
            for (user_msg, assistant_msg) in messages {
                conversation.add_user_message(user_msg);
                conversation.add_assistant_message(assistant_msg);
            }
        }

        conversation.add_user_message(&message);

        let request = conversation.build(claude_sdk::models::CLAUDE_HAIKU_4_5.anthropic_id, 1024);
        let response = self.client.send_message(request).await?;
        let message_response = response
            .content
            .iter()
            .filter_map(|block| {
                if let ContentBlock::Text {
                    text,
                    cache_control,
                    citations,
                } = block
                {
                    Some(text.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("");

        self.history
            .entry(user)
            .or_insert_with(Vec::new)
            .push((message, message_response.clone()));
        Ok(Some(message_response))
    }
}
