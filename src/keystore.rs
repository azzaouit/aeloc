use dirs::config_dir;
use ethers_core::rand::thread_rng;
use ethers_signers::{LocalWallet, Signer, WalletError};
use log::info;
use std::path::PathBuf;

const KS_NAME: &str = "aeloc.ks";

#[derive(Debug)]
pub struct KeyStore {
    pub wallet: LocalWallet,
}

impl KeyStore {
    pub fn ks_dir() -> PathBuf {
        config_dir().unwrap().join("aeloc")
    }

    pub fn create(password: &String) -> Result<Self, WalletError> {
        let ks_dir = Self::ks_dir();
        let ks_path = ks_dir.join(KS_NAME).into_os_string().into_string().unwrap();
        let _ = std::fs::create_dir(ks_dir);
        let (wallet, _) =
            LocalWallet::new_keystore(&ks_path, &mut thread_rng(), password, Some(&ks_path))?;
        let addr = hex::encode(wallet.address());
        info!("Created new wallet address {}", addr);
        Ok(Self { wallet })
    }

    pub fn open(password: &String) -> Result<Self, WalletError> {
        let ks_dir = Self::ks_dir();
        let ks_path = ks_dir.join(KS_NAME).into_os_string().into_string().unwrap();
        let wallet = LocalWallet::decrypt_keystore(ks_path, password)?;
        let addr = hex::encode(wallet.address());
        info!("Opened wallet address {}", addr);
        Ok(Self { wallet })
    }
}
