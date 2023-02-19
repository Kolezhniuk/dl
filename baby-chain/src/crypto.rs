use ed25519_dalek::{
    Digest, Keypair, PublicKey, SecretKey, Sha512, Signature, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
};
use rand::rngs::OsRng;

const CTX: &[u8; 10] = b"baby-chain";

#[derive(Debug)]
pub struct ChainKeyPair {
    key_pair: Keypair
}

impl ChainKeyPair {
    pub fn gen_key_pair() -> ChainKeyPair {
        let mut csprng = OsRng {};
        let key_pair = Keypair::generate(&mut csprng);
        ChainKeyPair { key_pair }
    }

    pub fn public_key(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.key_pair.public.as_bytes()
    }

    pub fn secret_key(&self) -> &[u8; SECRET_KEY_LENGTH] {
        &self.key_pair.secret.as_bytes()
    }
}

pub fn sign_data(sk_bytes: &[u8; SECRET_KEY_LENGTH], message: &[u8]) -> [u8; Signature::BYTE_SIZE] {
    let mut pre_hashed: Sha512 = Sha512::default();
    pre_hashed.update(message);
    let sk: SecretKey = SecretKey::from_bytes(sk_bytes).expect("secret key reading err");
    let pk: PublicKey = (&sk).into();
    let sig: Signature = Keypair {
        secret: sk,
        public: pk,
    }
    .sign_prehashed(pre_hashed, Some(CTX))
    .expect("error signing");
    sig.to_bytes()
}

pub fn verify_data(sig_bytes: &[u8; Signature::BYTE_SIZE], message: &[u8], pk: &[u8]) -> bool {
    let mut pre_hashed: Sha512 = Sha512::default();
    pre_hashed.update(message);
    let pk = PublicKey::from_bytes(pk).expect("public key read err");
    let sig = Signature::from_bytes(sig_bytes).expect("error reading signature from bytes");
    let verified = pk.verify_prehashed(pre_hashed, Some(CTX), &sig);
    verified.is_ok()
}
