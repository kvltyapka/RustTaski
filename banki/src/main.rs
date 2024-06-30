use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::RwLock;
use std::thread;

struct Bank {
    id: u32,
    balance: RwLock<i64>,
    accounts: RwLock<HashMap<u32, i64>>,
}

impl Bank {
    fn new(id: u32) -> Bank {
        Bank {
            id,
            balance: RwLock::new(0),
            accounts: RwLock::new(HashMap::new()),
        }
    }

    fn get_balance(&self, account_id: u32) -> i64 {
        let accounts = self.accounts.read().unwrap();
        *accounts.get(&account_id).unwrap_or(&0)
    }

    fn deposit(&self, account_id: u32, amount: i64) {
        let mut accounts = self.accounts.write().unwrap();
        let balance = accounts.entry(account_id).or_insert(0);
        *balance += amount;

        let mut bank_balance = self.balance.write().unwrap();
        *bank_balance += amount;
    }

    fn withdraw(&self, account_id: u32, amount: i64) -> bool {
        let mut accounts = self.accounts.write().unwrap();
        if let Some(balance) = accounts.get_mut(&account_id) {
            if *balance >= amount {
                *balance -= amount;

                let mut bank_balance = self.balance.write().unwrap();
                *bank_balance -= amount;
                return true;
            }
        }
        false
    }

    fn transfer(
        &self,
        to_bank: &Sender<Transaction>,
        from_account: u32,
        to_account: u32,
        amount: i64,
    ) -> bool {
        if self.withdraw(from_account, amount) {
            to_bank
                .send(Transaction::Deposit {
                    account_id: to_account,
                    amount,
                })
                .unwrap();
            return true;
        }
        false
    }
}

enum Transaction {
    Deposit {
        account_id: u32,
        amount: i64,
    },
    Withdraw {
        account_id: u32,
        amount: i64,
    },
    Transfer {
        to_bank_id: u32,
        from_account: u32,
        to_account: u32,
        amount: i64,
    },
    GetBalance {
        account_id: u32,
        response: Sender<i64>,
    },
}

fn bank_thread(
    bank: Bank,
    receiver: Receiver<Transaction>,
    banks: HashMap<u32, Sender<Transaction>>,
) {
    while let Ok(transaction) = receiver.recv() {
        match transaction {
            Transaction::Deposit { account_id, amount } => {
                bank.deposit(account_id, amount);
            }
            Transaction::Withdraw { account_id, amount } => {
                bank.withdraw(account_id, amount);
            }
            Transaction::Transfer {
                to_bank_id,
                from_account,
                to_account,
                amount,
            } => {
                if let Some(sender) = banks.get(&to_bank_id) {
                    bank.transfer(sender, from_account, to_account, amount);
                }
            }
            Transaction::GetBalance {
                account_id,
                response,
            } => {
                let balance = bank.get_balance(account_id);
                response.send(balance).unwrap();
            }
        }
    }
}

fn request_balance(sender: &Sender<Transaction>, account_id: u32) -> i64 {
    let (response_tx, response_rx) = mpsc::channel();
    sender
        .send(Transaction::GetBalance {
            account_id,
            response: response_tx,
        })
        .unwrap();
    response_rx.recv().unwrap()
}

fn main() {
    let bank1 = Bank::new(1);
    let bank2 = Bank::new(2);

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let mut banks = HashMap::new();
    banks.insert(1, tx1.clone());
    banks.insert(2, tx2.clone());

    let banks1 = banks.clone();
    let handle1 = thread::spawn(move || {
        bank_thread(bank1, rx1, banks1);
    });

    let banks2 = banks.clone();
    let handle2 = thread::spawn(move || {
        bank_thread(bank2, rx2, banks2);
    });

    println!(
        "Initial Balance of Bank 1, Account 1: {}",
        request_balance(&tx1, 1)
    );
    println!(
        "Initial Balance of Bank 2, Account 1: {}",
        request_balance(&tx2, 1)
    );

    // Транзакции
    tx1.send(Transaction::Deposit {
        account_id: 1,
        amount: 100,
    })
    .unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    println!(
        "Balance of Bank 1, Account 1 after deposit: {}",
        request_balance(&tx1, 1)
    );
    println!(
        "Balance of Bank 2, Account 1 after deposit: {}",
        request_balance(&tx2, 1)
    );

    tx1.send(Transaction::Transfer {
        to_bank_id: 2,
        from_account: 1,
        to_account: 1,
        amount: 50,
    })
    .unwrap();
    thread::sleep(std::time::Duration::from_millis(100));
    println!(
        "Final Balance of Bank 1, Account 1: {}",
        request_balance(&tx1, 1)
    );
    println!(
        "Final Balance of Bank 2, Account 1: {}",
        request_balance(&tx2, 1)
    );

    handle1.join().unwrap();
    handle2.join().unwrap();
}
