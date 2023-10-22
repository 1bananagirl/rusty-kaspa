pub use crate::encryption::{Decrypted, Encryptable, Encrypted};

pub mod account;
pub mod address;
pub mod binding;
pub mod hint;
pub mod id;
pub mod interface;
pub mod keydata;
pub mod local;
pub mod metadata;
pub mod transaction;

pub use crate::runtime::{AccountId, AccountKind};
pub use account::{Account, AccountData, Bip32, Keypair, Legacy, MultiSig, Settings};
pub use address::AddressBookEntry;
pub use binding::Binding;
pub use hint::Hint;
pub use id::IdT;
pub use interface::{AccessContextT, AccountStore, Interface, PrvKeyDataStore, TransactionRecordStore, WalletDescriptor};
pub use keydata::{KeyCaps, PrvKeyData, PrvKeyDataId, PrvKeyDataInfo, PrvKeyDataMap, PrvKeyDataPayload};
pub use metadata::Metadata;
pub use transaction::{TransactionMetadata, TransactionRecord, TransactionType};

#[cfg(test)]
mod tests {

    use super::*;
    use crate::result::Result;
    use crate::secret::Secret;
    use crate::storage::local::Payload;
    use crate::storage::local::Wallet;
    use kaspa_bip32::{Language, Mnemonic};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_wallet_store_wallet_store_load() -> Result<()> {
        // This test creates a fake instance of keydata, stored account
        // instance and a wallet instance that owns them.  It then tests
        // loading of account references and a wallet instance and confirms
        // that the serialized data is as expected.

        let store = local::Storage::try_new("test-wallet-store")?;

        let mut payload = Payload::default();

        let wallet_secret = Secret::from("ABC-L4LXw2F7HEK3wJU-Rk4stbPy6c");
        let payment_secret = Secret::from("test-123-# L4LXw2F7HEK3wJU Rk4stbPy6c");
        let mnemonic1s = "caution guide valley easily latin already visual fancy fork car switch runway vicious polar surprise fence boil light nut invite fiction visa hamster coyote".to_string();
        let mnemonic2s = "fiber boy desk trip pitch snake table awkward endorse car learn forest solid ticket enemy pink gesture wealth iron chaos clock gather honey farm".to_string();

        let mnemonic1 = Mnemonic::new(mnemonic1s.clone(), Language::English)?;
        let prv_key_data1 = PrvKeyData::try_new_from_mnemonic(mnemonic1.clone(), Some(&payment_secret))?;

        let mnemonic2 = Mnemonic::new(mnemonic2s.clone(), Language::English)?;
        let prv_key_data2 = PrvKeyData::try_new_from_mnemonic(mnemonic2.clone(), Some(&payment_secret))?;

        let pub_key_data1 = Arc::new(vec!["abc".to_string()]);
        let pub_key_data2 = Arc::new(vec!["xyz".to_string()]);
        println!("keydata1 id: {:?}", prv_key_data1.id);
        //assert_eq!(prv_key_data.id.0, [79, 36, 5, 159, 220, 113, 179, 22]);
        payload.prv_key_data.push(prv_key_data1.clone());
        payload.prv_key_data.push(prv_key_data2.clone());

        let settings = Settings { name: Some("Wallet-A".to_string()), title: Some("Wallet A".to_string()), is_visible: false };
        let bip32 = Bip32::new(0, pub_key_data1.clone(), false);
        let id = AccountId::from_bip32(&prv_key_data1.id, &bip32);
        let account1 = Account::new(id, Some(prv_key_data1.id), settings, AccountData::Bip32(bip32));
        payload.accounts.push(account1);

        let settings = Settings { name: Some("Wallet-B".to_string()), title: Some("Wallet B".to_string()), is_visible: false };
        let bip32 = Bip32::new(0, pub_key_data2.clone(), false);
        let id = AccountId::from_bip32(&prv_key_data2.id, &bip32);
        let account2 = Account::new(id, Some(prv_key_data2.id), settings, AccountData::Bip32(bip32));
        payload.accounts.push(account2);

        let payload_json = serde_json::to_string(&payload).unwrap();
        // let settings = WalletSettings::new(account_id);

        let w1 = Wallet::try_new(None, None, &wallet_secret, payload, vec![])?;
        w1.try_store(&store).await?;
        // Wallet::try_store_payload(&store, &wallet_secret, payload).await?;

        let w2 = Wallet::try_load(&store).await?;
        let w2payload = w2.payload.decrypt::<Payload>(&wallet_secret).unwrap();
        println!("\n---\nwallet.metadata (plain): {:#?}\n\n", w2.metadata);
        // let w2payload_json = serde_json::to_string(w2payload.as_ref()).unwrap();
        println!("\n---nwallet.payload (decrypted): {:#?}\n\n", w2payload.as_ref());
        // purge the store
        store.purge().await?;

        assert_eq!(payload_json, serde_json::to_string(w2payload.as_ref())?);

        let w2keydata1 = w2payload.as_ref().prv_key_data.get(0).unwrap();
        let w2keydata1_payload = w2keydata1.payload.decrypt(Some(&payment_secret)).unwrap();
        let first_mnemonic = &w2keydata1_payload.as_ref().as_mnemonic()?.unwrap().phrase_string();
        // println!("first mnemonic (plain): {}", hex_string(first_mnemonic.as_ref()));
        println!("first mnemonic (plain): {first_mnemonic}");
        assert_eq!(&mnemonic1.phrase_string(), first_mnemonic);

        let w2keydata2 = w2payload.as_ref().prv_key_data.get(1).unwrap();
        let w2keydata2_payload = w2keydata2.payload.decrypt(Some(&payment_secret)).unwrap();
        let second_mnemonic = &w2keydata2_payload.as_ref().as_mnemonic()?.unwrap().phrase_string();
        println!("second mnemonic (encrypted): {second_mnemonic}");
        assert_eq!(&mnemonic2.phrase_string(), second_mnemonic);

        Ok(())
    }
}