use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Bank
pub struct Bank {
    pub id: u32,
    pub balance: Arc<RwLock<u32>>,
    pub clients: Arc<RwLock<HashMap<u32, Client>>>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            id: 0,
            balance: Arc::new(RwLock::new(0)),
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_client(&self, client: Client) {
        let mut clients = self.clients.write().unwrap();
        clients.insert(client.id, client);
    }

    pub fn get_balance(&self) -> u32 {
        *self.balance.read().unwrap()
    }
}

// Client
pub struct Client {
    pub id: u32,
    pub balance: Arc<RwLock<u32>>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            id: 0,
            balance: Arc::new(RwLock::new(0)),
        }
    }
}

// Transaction
pub struct Transaction {
    pub from_bank: u32,
    pub from_client: u32,
    pub to_bank: u32,
    pub to_client: u32,
    pub amount: u32,
}

impl Transaction {
    pub fn new(
        from_bank: u32,
        from_client: u32,
        to_bank: u32,
        to_client: u32,
        amount: u32,
    ) -> Self {
        Transaction {
            from_bank,
            from_client,
            to_bank,
            to_client,
            amount,
        }
    }
}
