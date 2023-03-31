use crate::{crypto::ChainKeyPair, transaction::Transaction, account::Account};

#[derive(Debug, Clone)]
pub enum Currency {
    Bitcoin,
    Ethereum,
}

#[derive(Debug, Clone)]
pub struct Wallet {
    pub account: Vec<ChainKeyPair>,
    pub user_txs: Vec<Transaction>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {
            account: vec![],
            user_txs: vec![],
        }
    }

    pub fn generate_new_key_pair(&mut self) {
        let key_pair = ChainKeyPair::gen_key_pair();
        self.account.push(key_pair);
    }

    //    pub fn get_balance() -> u64 {
    //        0
    //    }

    //afunction that forms, completes, signs, and initiates the propagation of a transaction through a full network node.
    pub fn create_transaction(&mut self, tx: Transaction) {}
}
