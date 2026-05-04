use std::sync::Arc;

use azalea::prelude::*;
use shared::prelude::*;

#[derive(Default, Clone, Component)]
pub struct State {
    pub config: Option<Arc<AppConfig>>,
}
