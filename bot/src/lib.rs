mod bot_state;
mod chat_event;
mod chat_event_handler;
mod event_handler;
mod on_towny;

use azalea::prelude::*;
use azalea::{Client, ClientBuilder, Event};
use azalea_viaversion::ViaVersionPlugin;
use shared::prelude::*;
use std::{sync::Arc, time::Duration};

use crate::bot_state::State;
use crate::event_handler::event_handler;

pub async fn run(config: Arc<AppConfig>) -> Result<()> {
    let account = Account::microsoft(&config.email).await?;

    let state: State = State {
        config: Some(config.clone()),
        ..Default::default()
    };
    let mc_version = config.mc_version.clone();

    ClientBuilder::new()
        .add_plugins(ViaVersionPlugin::start(mc_version).await)
        .set_handler(event_handler)
        .set_state(state)
        .reconnect_after(None)
        .start(account, "mc.thecavern.net")
        .await;

    Ok(())
}
