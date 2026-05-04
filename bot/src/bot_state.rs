use std::sync::Arc;

use azalea::prelude::*;
use shared::{bot_status::BotStatus, prelude::*};
use tokio::sync::RwLock;

#[derive(Default, Clone, Component)]
pub struct State {
    pub config: Option<Arc<AppConfig>>,
    pub status: Arc<RwLock<BotStatus>>,
}
