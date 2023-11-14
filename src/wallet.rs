use crate::tezos::TezosWallet;

#[derive(Debug)]
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub public_address: String,
    pub kind: WalletCrypto,
}

impl Wallet {
    pub fn generate(crypto: impl Crypto) -> Self {
        let (secret_key, public_key) = crypto.generate_keypair(None);
        let public_address = crypto.public_key_hash(public_key.clone());
        Wallet {
            kind: crypto.kind(),
            secret_key,
            public_key,
            public_address,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WalletCrypto {
    Tezos(TezosWallet),
    Other,
}

pub trait Crypto {
    fn generate_keypair(&self, seed: Option<Vec<u8>>) -> (Vec<u8>, Vec<u8>);
    fn secret_uncrypted_key_base58(&self, secret_key: Vec<u8>) -> String;
    fn secret_encrypted_key_base58(&self, secret_key: Vec<u8>) -> String;
    fn public_key_base58(&self, public_key: Vec<u8>) -> String;
    fn public_key_hash(&self, public_key: Vec<u8>) -> String;
    fn kind(&self) -> WalletCrypto;
    fn from_unencrypted_secret(&self, secret_key: String) -> Wallet;
    fn from_encrypted_secret(&self, secret_key: String) -> Wallet;
}
