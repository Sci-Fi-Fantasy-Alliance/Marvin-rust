
use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
pub struct MarvinShardManager;

impl TypeMapKey for MarvinShardManager {
    type Value = Arc<Mutex<MarvinShardManager>>;
}