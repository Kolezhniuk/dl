mod account;
mod block;
mod blockchain;
mod connection;
mod crypto;
mod node;
mod operation;
mod transaction;
mod user_application;
mod wallet;
fn main() {
    println!("Hello, Baby chain!");
}

pub fn get_key_pair() -> ([u8; 32], [u8; 32]) {
    let key_pair = crypto::ChainKeyPair::gen_key_pair();
    let pk = key_pair.public_key();
    let sk = key_pair.secret_key();
    (*pk, *sk)
}

#[cfg(test)]
mod tests {

    use std::io::Chain;

    use super::*;
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    #[test]
    fn test_stage_2_create_keys_and_sign_data() {
        let (pk, sk) = get_key_pair();
        let message = b"test verification sign success";
        let message_failure = b"test verification sign failure";
        let sig_bytes = crypto::sign_data(&sk, message);
        let verified = crypto::verify_data(&sig_bytes, message, &pk);
        assert!(verified == true);
        let verified = crypto::verify_data(&sig_bytes, message_failure, &pk);
        assert!(verified == false);
    }

    #[test]
    fn test_stage_3_create_account_and_update_balance() {
        let (pk, _) = get_key_pair();
        let mut account =
            account::Account::create_account(b"test_sender_account_id".to_vec(), vec![pk], 100);

        account.update_balance(100);

        assert_eq!(account.get_balance(), 200);
    }

    fn create_operation(
        amount_to_transfer: u64,
        amount_to_sign: u64,
    ) -> (Vec<u8>, operation::Operation) {
        let (sender_pk, sender_sk) = get_key_pair();
        let (receiver_pk, _) = get_key_pair();
        let sender = account::Account::create_account(
            b"test_sender_account_id".to_vec(),
            vec![sender_pk],
            100,
        );
        let sender_id = sender.account_id.clone();
        let receiver = account::Account::create_account(
            b"test_receiver_account_id".to_vec(),
            vec![receiver_pk],
            50,
        );

        let sig_bytes = crypto::sign_data(&sender_sk, &amount_to_sign.to_string().as_bytes());

        let operation = operation::Operation::create_operation(
            sender,
            receiver,
            amount_to_transfer,
            sig_bytes.to_vec(),
        );

        (sender_id, operation)
    }

    #[test]
    fn test_stage_4_create_operation_should_be_verified() {
        let (_, operation) = create_operation(100, 100);
        assert!(operation::Operation::verify_operation(operation) == true);
    }

    #[test]
    fn test_stage_4_create_operation_should_be_verified_insufficient_amount() {
        let (_, operation) = create_operation(200, 200);
        assert!(operation::Operation::verify_operation(operation) == false);
    }

    #[test]
    fn test_stage_4_create_operation_should_not_be_verified_invalid_signature() {
        let (_, operation) = create_operation(100, 10);
        assert!(operation::Operation::verify_operation(operation) == false);
    }

    #[test]
    fn test_stage_4_create_transaction_ids_should_be_eq() {
        let (account_id, operation): (Vec<u8>, operation::Operation) = create_operation(10, 10);
        let account_id2 = account_id.clone();
        let operation_2 = operation.clone();
        let transaction1 =
            transaction::Transaction::create_transaction(account_id, vec![operation], 12345);
        let transaction2 =
            transaction::Transaction::create_transaction(account_id2, vec![operation_2], 12345);
        assert_eq!(transaction1.transaction_id, transaction2.transaction_id);
    }

    #[test]
    fn test_init_blockchain() {
        let chain = blockchain::Blockchain::init(10_000);
        print!("{:?}", chain);
        assert_eq!(chain.blocks.len(), 1);
    }
}
