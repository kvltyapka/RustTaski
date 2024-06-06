use chrono::{DateTime, Utc};
use std::collections::HashMap;
pub struct Post {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub content: String,
}

pub struct Blog {
    pub posts: HashMap<u32, Post>,
    pub next_id: u32,
}

impl Blog {
    pub fn new() -> Self {
        Blog {
            posts: HashMap::new(),
            next_id: 1,
        }
    }
}
