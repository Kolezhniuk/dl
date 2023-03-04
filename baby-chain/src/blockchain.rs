use std::collections::{HashMap, HashSet};

use crate::{account, block::{Block, self}, operation, transaction::Transaction};

#[derive(Debug)]
pub struct Blockchain {
    // account_id -> balance
    pub coin_database: HashMap<Vec<u8>, u64>,
    pub blocks: Vec<Block>,
    pub tx_database: Vec<Transaction>,
    pub faucet_coins: u64,
}

impl Blockchain {
    pub fn init(faucet_coins: u64) -> Blockchain {
        let coin_database = HashMap::new();
        let mut blocks = Vec::new();
        let zero_hash = vec![0; 32];
        let genesis_block = Block::create_block(zero_hash, Vec::new());
        blocks.push(genesis_block);
        Blockchain {
            coin_database,
            blocks,
            tx_database: Vec::new(),
            faucet_coins,
        }
    }

    pub fn get_token_from_faucet(&mut self, account_id: Vec<u8>) {
        self.update_balance(account_id, self.faucet_coins);
    }

    pub fn add_block(&mut self, block: Block) {
        if !self.validate_block(&block) {
            panic!("Invalid block");
        }
        self.blocks.push(block);
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        let prev_block = self.blocks.last().unwrap();
        if prev_block.block_id != block.prev_hash {
            return false;
        }
        let transactions = block.transactions.to_vec();
        //Checking that transactions in the block have not been added to history yet;
        for tx in &transactions {
            if self.tx_database.contains(tx) {
                return false;
            }
            //Checking every operation in a transaction
            for operation in &tx.operations {
                if !operation::Operation::verify_operation(operation.clone()) {
                    return false;
                }
            }
        }
        //Checking that the block does not contain conflicting transactions;
        if !Blockchain::has_unique_elements(transactions) {
            return false;
        }

        true
    }

    fn has_unique_elements(transactions: Vec<Transaction>) -> bool {
        let mut uniq = HashSet::new();
        transactions
            .into_iter()
            .map(|x| x.transaction_id)
            .all(move |x| uniq.insert(x))
    }

    fn get_balance(&self, account_id: Vec<u8>) -> u64 {
        match self.coin_database.get(&account_id) {
            Some(balance) => *balance,
            None => 0,
        }
    }

    pub fn update_balance(&mut self, account_id: Vec<u8>, amount: u64) {
        let balance = self.get_balance(account_id.clone());
        if let None = balance.checked_add(amount) {
            panic!("Balance overflow")
        }
        self.coin_database.insert(account_id, balance + amount);
    }
}
