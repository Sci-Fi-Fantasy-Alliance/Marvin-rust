use std::env;

use serenity::prelude::GatewayIntents;

pub struct Config {
    pub token: String,
    pub discord_intents: GatewayIntents,
}

impl Config {
    pub fn new() -> Self {
        Self {
            token: env::var("MARVIN_BETA_TOKEN").expect("Expected a token in the environment"),
            discord_intents: GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT,
        }
    }
}