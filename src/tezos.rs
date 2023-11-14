use ed25519_dalek::SecretKey;
use rand::rngs::OsRng;
use tezos_core::types::encoded::{
    Ed25519PublicKey, Ed25519PublicKeyHash, Ed25519SecretKey, Ed25519Seed, Encoded, PublicKey,
};

use crate::wallet::{Crypto, Wallet, WalletCrypto};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TezosWallet {
    TZ1,
    TZ2,
    TZ3,
}

impl Crypto for TezosWallet {
    fn generate_keypair(&self, seed: Option<Vec<u8>>) -> (Vec<u8>, Vec<u8>) {
        match seed {
            Some(_seed) => {
                todo!()
            }
            None => match self {
                TezosWallet::TZ1 => {
                    let pair_key = ed25519_dalek::SigningKey::generate(&mut OsRng);
                    let public_key_bytes = pair_key.verifying_key().to_bytes().to_vec();
                    let secret_key_bytes = pair_key.to_bytes().to_vec();
                    (secret_key_bytes, public_key_bytes)
                }
                TezosWallet::TZ2 => todo!(),
                TezosWallet::TZ3 => todo!(),
            },
        }
    }

    fn secret_encrypted_key_base58(&self, secret_key: Vec<u8>) -> String {
        match self {
            TezosWallet::TZ1 => {
                let public_key = Ed25519SecretKey::from_bytes(secret_key.as_slice()).unwrap();
                public_key.into_string()
            }
            TezosWallet::TZ2 => todo!(),
            TezosWallet::TZ3 => todo!(),
        }
    }

    fn secret_uncrypted_key_base58(&self, secret_key: Vec<u8>) -> String {
        match self {
            TezosWallet::TZ1 => Ed25519Seed::from_bytes(secret_key.as_slice())
                .unwrap()
                .into_string(),
            TezosWallet::TZ2 => todo!(),
            TezosWallet::TZ3 => todo!(),
        }
    }

    fn public_key_base58(&self, public_key: Vec<u8>) -> String {
        match self {
            TezosWallet::TZ1 => Ed25519PublicKeyHash::from_bytes(public_key.as_slice())
                .unwrap()
                .into_string(),
            TezosWallet::TZ2 => todo!(),
            TezosWallet::TZ3 => todo!(),
        }
    }

    fn public_key_hash(&self, public_key: Vec<u8>) -> String {
        let bytes = match self {
            TezosWallet::TZ1 => Ed25519PublicKey::from_bytes(public_key.as_slice()).unwrap(),
            TezosWallet::TZ2 => todo!(),
            TezosWallet::TZ3 => todo!(),
        };
        let key: PublicKey = bytes.value().try_into().unwrap();
        key.bs58_address().unwrap()
    }

    fn kind(&self) -> WalletCrypto {
        WalletCrypto::Tezos(*self)
    }

    fn from_unencrypted_secret(&self, secret: String) -> Wallet {
        match self {
            TezosWallet::TZ1 => {
                let secret = Ed25519Seed::try_from(secret).unwrap();
                let bytes: SecretKey = secret.to_bytes().unwrap().try_into().unwrap();
                let pair_key = ed25519_dalek::SigningKey::from_bytes(&bytes);
                let public_key = pair_key.verifying_key().to_bytes().to_vec();
                let secret_key = pair_key.to_bytes().to_vec();
                let public_address = self.public_key_hash(public_key.clone());

                Wallet {
                    kind: self.kind(),
                    secret_key,
                    public_key,
                    public_address,
                }
            }
            TezosWallet::TZ2 => todo!(),
            TezosWallet::TZ3 => todo!(),
        }
    }

    fn from_encrypted_secret(&self, secret: String) -> Wallet {
        match self {
            TezosWallet::TZ1 => {
                let bytes = Ed25519SecretKey::try_from(secret)
                    .unwrap()
                    .to_bytes()
                    .unwrap();
                let public_key = bytes[32..].to_vec();
                let secret_key = bytes[..32].to_vec();
                let public_address = self.public_key_hash(public_key.clone());

                Wallet {
                    kind: self.kind(),
                    secret_key,
                    public_key,
                    public_address,
                }
            }
            TezosWallet::TZ2 => todo!(),
            TezosWallet::TZ3 => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tz1_generate() {
        let wallet = Wallet::generate(TezosWallet::TZ1);
        assert!(wallet.public_address.starts_with("tz1"));
        assert_eq!(wallet.public_key.len(), 32);
        assert_eq!(wallet.secret_key.len(), 32);
        assert_eq!(wallet.kind, WalletCrypto::Tezos(TezosWallet::TZ1));
    }

    #[test]
    fn tz1_from_unencrypted_secret() {
        let secret = "edsk2hP48izVsHsXtqguwiNt5wq1qXdwLyxFQC8Qc72KuyKS9q88XS";
        let wallet = TezosWallet::TZ1.from_unencrypted_secret(secret.to_string());
        assert_eq!(wallet.public_key.len(), 32);
        assert_eq!(wallet.secret_key.len(), 32);
        assert_eq!(
            wallet.public_address,
            "tz1dcv5NSS2Fbs2dW9pRDhi6KJTBAXqiJKBP"
        );
        assert_eq!(wallet.kind, WalletCrypto::Tezos(TezosWallet::TZ1));
    }

    #[test]
    fn tz1_from_encrypted_secret() {
        let secret = "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR";
        let wallet = TezosWallet::TZ1.from_encrypted_secret(secret.to_string());
        assert_eq!(wallet.public_key.len(), 32);
        assert_eq!(wallet.secret_key.len(), 32);
        assert_eq!(
            wallet.public_address,
            "tz1eDTA63431CjNN9xbTi1wuFRmeeZcUNAML"
        );
        assert_eq!(wallet.kind, WalletCrypto::Tezos(TezosWallet::TZ1));
    }

    #[test]
    fn tz1_secret_uncrypted_key_base58() {
        let secret = "edsk2ym28S1pkkCgG5KqWTWCNu5VVUMm6yKgRn7drdixVeppRQvqvb";
        let bytes = Ed25519Seed::try_from(secret).unwrap().to_bytes().unwrap();
        let secret_key = TezosWallet::TZ1.secret_uncrypted_key_base58(bytes);
        assert_eq!(
            secret_key,
            "edsk2ym28S1pkkCgG5KqWTWCNu5VVUMm6yKgRn7drdixVeppRQvqvb"
        );
    }

    #[test]
    fn tz1_secret_encrypted_key_base58() {
        let secret = "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR";
        let bytes = Ed25519SecretKey::try_from(secret)
            .unwrap()
            .to_bytes()
            .unwrap();
        let secret_key = TezosWallet::TZ1.secret_encrypted_key_base58(bytes);
        assert_eq!(secret_key, "edskRhKTQkgxb7CNTr31rzy3xdkyKaYX9hySAnZYJTPmUzPB7WU4NL7C8pmtQDgRqQ4jDw4Ugh6Y1UW5nvo7UYrRbyhVYK1YuR");
    }

    #[test]
    fn tz1_public_key_base58() {
        let public_key = "tz1dcv5NSS2Fbs2dW9pRDhi6KJTBAXqiJKBP";
        let bytes = Ed25519PublicKeyHash::try_from(public_key)
            .unwrap()
            .to_bytes()
            .unwrap();
        let secret_key = TezosWallet::TZ1.public_key_base58(bytes);
        assert_eq!(secret_key, "tz1dcv5NSS2Fbs2dW9pRDhi6KJTBAXqiJKBP");
    }
}
