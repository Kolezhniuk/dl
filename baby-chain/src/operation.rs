use crate::{account::Account, crypto};

#[derive(Debug, Hash, Clone)]
pub struct Operation {
    pub sender: Account,
    pub receiver: Account,
    pub amount: u64,
    pub signature: [u8; 64],
}

impl Operation {
    pub fn create_operation(
        sender: Account,
        receiver: Account,
        amount: u64,
        signature: [u8; 64],
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
        let signature = operation.signature;

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
