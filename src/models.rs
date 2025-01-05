use crate::HashMap;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Id {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub name: String,
    pub quantity: i32,
}

#[derive(Clone)]
pub struct Store {
  pub grocery_list: Arc<RwLock<Items>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            grocery_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}