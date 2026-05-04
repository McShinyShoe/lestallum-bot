use azalea::Client;
use azalea::prelude::*;
use shared::prelude::*;

use crate::bot_state::State;

pub async fn on_towny(client: Client, state: State) -> Result<()> {
    loop {
        client.wait_ticks(1);
    }
    Ok(())
}
