use std::sync::Arc;

use shared::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let config: AppConfig = AppConfig::new()?;
    let config_store = Arc::new(config);

    let bot_config = config_store.clone();
    let bot_thread = std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(bot::run(bot_config))
    });

    let api_config = config_store.clone();
    let api_thread = std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(api::run(api_config))
    });

    api_thread.join()?;
    bot_thread.join()?;

    tokio::signal::ctrl_c().await.unwrap();

    Ok(())
}
