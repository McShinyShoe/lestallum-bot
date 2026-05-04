use crate::bot_task::BotTask;

pub enum BotStatus {
    Disconnected,
    Busy(BotTask),
    Idle,
}
