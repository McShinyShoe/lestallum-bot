pub enum ChatEvent {
    DirectMessage {
        sender: String,
        receiver: String,
        message: String,
    },
    Unknown,
}

impl ChatEvent {
    pub fn identify(input: String) -> Self {
        if input.starts_with("✉ ") {
            let Some(body) = input.strip_prefix("✉ [MSG] ") else {
                return ChatEvent::Unknown;
            };
            let Some((sender_part, rest)) = body.split_once(" → ") else {
                return ChatEvent::Unknown;
            };
            let Some((receiver_part, message_part)) = rest.split_once(' ') else {
                return ChatEvent::Unknown;
            };

            return ChatEvent::DirectMessage {
                sender: sender_part.trim().to_string(),
                receiver: receiver_part.trim().to_string(),
                message: message_part.trim().to_string(),
            };
        };
        if input.contains("»") {}
        ChatEvent::Unknown
    }
}
