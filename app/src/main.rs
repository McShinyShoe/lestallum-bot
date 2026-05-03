use std::sync::Arc;

use shared::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::new()?;
    let config_store = Arc::new(config);

    let bot_handle = tokio::spawn(bot::run(config_store.clone()));
    let api_handle = tokio::spawn(api::run(config_store.clone()));

    let (bot_res, api_res) = tokio::join!(bot_handle, api_handle);

    bot_res??;
    api_res??;

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
