use crate::bot_task::BotTask;

#[derive(Default, Clone)]
pub enum BotStatus {
    #[default]
    Disconnected,
    Busy(BotTask),
    Idle,
}
