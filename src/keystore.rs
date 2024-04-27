use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer, WalletError};
use log::info;

#[derive(Debug)]
pub struct KeyStore {
    pub wallet: LocalWallet,
}

const KEYSTORE_FILE: &str = ".aeloc-keystore";

impl KeyStore {
    pub fn create(password: &String) -> Result<Self, WalletError> {
        let (wallet, _) =
            LocalWallet::new_keystore(".", &mut thread_rng(), password, Some(KEYSTORE_FILE))?;
        let addr = hex::encode(wallet.address());
        info!("Created new wallet address {}", addr);
        Ok(Self { wallet })
    }

    pub fn open(password: &String) -> Result<Self, WalletError> {
        let wallet = LocalWallet::decrypt_keystore(KEYSTORE_FILE, password)?;
        let addr = hex::encode(wallet.address());
        info!("Opened wallet address {}", addr);
        Ok(Self { wallet })
    }

    pub fn open_or_create(password: &String) -> Result<Self, WalletError> {
        let w = Self::open(password);
        if w.is_ok() {
            return w;
        }
        Self::create(password)
    }
}
