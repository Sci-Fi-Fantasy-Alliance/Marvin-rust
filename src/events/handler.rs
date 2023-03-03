use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::{EventHandler, Context};

pub struct MarvinEventsHandler;

#[async_trait]
impl EventHandler for MarvinEventsHandler {

    // Set a handler to be called on the `message` event. This is called when a
    // message is created in a channel the bot has access to.
    async fn message(&self, _ctx: Context, _msg: Message) {
        
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}