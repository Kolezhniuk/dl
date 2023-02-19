mod account;
mod crypto;

fn main() {
    let key_pair = crypto::ChainKeyPair::gen_key_pair();
    let pk = key_pair.public_key();
    let sk = key_pair.secret_key();
    println!("public key bytes {:?}", pk);
    println!("secret key bytes {:?}", sk);

    let message = b"test verification sign success";
    let message_failure = b"test verification sign failure";
    let sig_bytes = crypto::sign_data(sk, message);
    println!("signature bytes {:?}", sig_bytes);

    let verified = crypto::verify_data(&sig_bytes, message, pk);
    println!("verified message: {}", verified);
    let verified = crypto::verify_data(&sig_bytes, message_failure, pk);
    println!("verified message_failure: {}", verified);

    let mut account = account::Account::create_account(b"test_account_id".to_vec(), vec![], 100);

    println!("account: {:?}", account);

    account.add_key(*pk);

    account.update_balance(100);

    println!("account: {:?}", account);
}
