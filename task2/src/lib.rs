use std::collections::HashMap;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

pub struct Bank {
    pub id: u32,
    pub balacne: Arc<RwLock<u32>>,
    pub clients: Arc<RwLock<HashMap<u32, Client>>>,
}

pub struct Client {
    pub id: u32,
    pub balance: Arc<RwLock<u32>>,
}

pub struct Transaction {
    pub client_id: u32,
    pub amount: u32,
}
