use poise::{serenity_prelude as serenity};

use niihama_zaurus::common::types;
use niihama_zaurus::slash_commands;
use niihama_zaurus::utils;

#[tokio::main]
pub async fn run() {

    let config = utils::config::Config::new();

    let framework = poise::Framework::builder()     
        .options(poise::FrameworkOptions {
            commands: vec![slash_commands::age::age()],
            ..Default::default()
        })
        .token(config.token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(types::Data {  })
            })
        });

    framework.run().await.unwrap();
}