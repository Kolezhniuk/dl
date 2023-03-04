use std::clone;

use ed25519_dalek::PUBLIC_KEY_LENGTH;
use serde::{Deserialize, Serialize};
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Vec<u8>,
    pub pub_keys: Vec<[u8; PUBLIC_KEY_LENGTH]>,
    pub balance: u64,
}

impl Account {
    pub fn create_account(
        account_id: Vec<u8>,
        pub_keys: Vec<[u8; PUBLIC_KEY_LENGTH]>,
        balance: u64,
    ) -> Account {
        Account {
            account_id,
            pub_keys,
            balance,
        }
    }

    pub fn add_key(&mut self, pub_key: [u8; PUBLIC_KEY_LENGTH]) {
        self.pub_keys.push(pub_key);
    }

    pub fn update_balance(&mut self, amount: u64) {
        if let None = self.balance.checked_add(amount) {
            panic!("Balance overflow")
        }
        self.balance += amount;
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}
//implement copy and clone trait
impl clone::Clone for Account {
    fn clone(&self) -> Self {
        Account {
            account_id: self.account_id.clone(),
            pub_keys: self.pub_keys.clone(),
            balance: self.balance,
        }
    }
}

