use serenity::prelude::*;

#[path ="../events/mod.rs"]
mod events;
use events::handler::Handler;

pub struct MarvinClient {
    token: String,
    intents: GatewayIntents,
}



impl MarvinClient {
   pub fn new(marvin_token: String, marvin_intents: GatewayIntents) -> Self {
        Self {
            token : marvin_token,
            intents : marvin_intents,
        }


    }
    #[tokio::main]                                          
    pub async fn start(&self) {
        let mut client =
        Client::builder(&self.token, self.intents).event_handler(Handler).await.expect("Err creating client");
         
        // Finally, start a single shard                    , and start listening to events.
        //
        // Shards will automatically attempt to reconnect, and will perform
        // exponential backoff until it reconnects.
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }

}