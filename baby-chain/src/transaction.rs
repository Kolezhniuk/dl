use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{operation::Operation};
use ed25519_dalek::{Digest, Sha512};

#[derive(Debug)]
pub struct Transaction {
    pub transaction_id: Vec<u8>,
    pub initiator: Vec<u8>,
    pub operations: Vec<Operation>,
    pub nonce: u64,
}

impl Transaction {
    pub fn create_transaction(
        initiator: Vec<u8>,
        operations: Vec<Operation>,
        nonce: u64,
    ) -> Transaction {
        let mut operations_hasher = DefaultHasher::new();
        operations.hash(&mut operations_hasher);
        let operations_hash = operations_hasher.finish();
        let mut initiator_hasher = DefaultHasher::new();
        initiator.hash(&mut initiator_hasher);
        let initiator_hash = initiator_hasher.finish();
        let mut sha512_hash: Sha512 = Sha512::default();
        let mut bytes_to_hash = operations_hash.to_be_bytes().to_vec();
        bytes_to_hash.append(&mut initiator_hash.to_be_bytes().to_vec());
        bytes_to_hash.append(&mut nonce.to_be_bytes().to_vec());
        sha512_hash.update(&bytes_to_hash);

        let transaction_id = sha512_hash.finalize().as_slice().to_vec();

        Transaction {
            transaction_id,
            initiator,
            operations,
            nonce,
        }
    }
}
