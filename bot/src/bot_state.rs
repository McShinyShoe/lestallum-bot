use std::sync::Arc;

use azalea::prelude::*;
use shared::{bot_status::BotStatus, claude_controller::ClaudeController, prelude::*};
use tokio::sync::{Mutex, RwLock};

#[derive(Default, Clone, Component)]
pub struct State {
    pub config: Option<Arc<AppConfig>>,
    pub status: Arc<RwLock<BotStatus>>,
    pub claude_controller: Option<Arc<Mutex<ClaudeController>>>,
}
