use crate::{account::Account, crypto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub sender: Account,
    pub receiver: Account,
    pub amount: u64,
    pub signature: Vec<u8>,
}

fn to_signature_bytes<T>(v: Vec<T>) -> [T; 64]
where
    T: Copy,
{
    let slice = v.as_slice();
    let array: [T; 64] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 32, v.len()),
    };
    array
}

impl Operation {
    pub fn create_operation(
        sender: Account,
        receiver: Account,
        amount: u64,
        signature: Vec<u8>,
    ) -> Operation {
        Operation {
            sender,
            receiver,
            amount,
            signature,
        }
    }

    pub fn verify_operation(operation: Operation) -> bool {
        let sender = operation.sender;
        let amount = operation.amount;
        let signature = to_signature_bytes(operation.signature);

        let sender_balance = sender.get_balance();
        if sender_balance < amount {
            return false;
        }

        let sender_pub_keys = sender.pub_keys;
        let mut verified = false;
        for pk in sender_pub_keys {
            verified = crypto::verify_data(&signature, &amount.to_string().as_bytes(), &pk);
            if verified {
                break;
            }
        }
        verified
    }
}
