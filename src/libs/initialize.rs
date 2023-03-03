use super::config::Config;
#[path = "../client/mod.rs"]
mod client;
use client::marvinclient::MarvinClient;


pub fn boot() {
    let config: Config = Config::new();
    let marvin_client: MarvinClient = MarvinClient::new(config.token, config.discord_intents);
    marvin_client.start();
}