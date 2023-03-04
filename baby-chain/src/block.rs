use crate::transaction::Transaction;
use sha256::digest;

#[derive(Debug)]
pub struct Block {
    pub block_id: Vec<u8>,
    pub prev_hash: Vec<u8>,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn create_block(prev_hash: Vec<u8>, transactions: Vec<Transaction>) -> Block {
        let transaction_bytes = bincode::serialize(&transactions).unwrap();
        let combined_bytes = prev_hash
            .to_vec()
            .into_iter()
            .chain(transaction_bytes.into_iter())
            .collect::<Vec<u8>>();
        let string_from_bytes = String::from_utf8(combined_bytes).expect("Found invalid UTF-8");
        let block_id_str = digest(string_from_bytes);
        let block_id = block_id_str.as_bytes();
        Block {
            block_id: block_id.to_vec(),
            prev_hash: prev_hash,
            transactions,
        }
    }
}
