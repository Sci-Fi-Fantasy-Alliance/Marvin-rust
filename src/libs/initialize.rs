use super::config::Config;
use super::marvinclient::MarvinClient;


pub fn boot() {
    let config = Config::new();
    let marvin_client = MarvinClient::new(config.token, config.discord_intents);
    marvin_client.start();
}
