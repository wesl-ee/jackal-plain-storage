use crate::Config;
use cw_storage_plus::{Item, Map};

pub const CONFIG: Item<Config> = Item::new("config");
pub const COLLECTION: Map<u32, String> = Map::new("collection");
