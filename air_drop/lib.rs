#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod air_drop {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        collections::HashMap as StorageHashMap,
    };
    use erc20::Erc20;
    use ink_prelude::collections::BTreeMap;

    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct TokenStandardIns {
        token_standard_addr: AccountId,
    }
    
    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub struct AirDropInfo {
        index: u64,
    }

    #[ink(storage)]
    pub struct AirDropManager {
        owner: Option<AccountId>,
        index: u64,
        // the list of token standard ins address
        token_standard_map: StorageHashMap<AccountId, u64>,
    }

    #[ink(event)]
    pub struct RegisterTokenStandardIns {
        #[ink(topic)]
        owner: AccountId,
        index: u64,
    }

    #[ink(event)]
    pub struct AirDropTransfer {
        #[ink(topic)]
        from: Option<AccountId>,
        // token standard instance count
        token_standard_ins_count: u64,
        count: u64,
        // By default, the funds transferred from the airdrop to each address are the same. 
        value: u64,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        RegisterTokenStandardInsError,
        TokenStandardInsNotFound,
    }

    // result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl AirDropManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                owner: None,
                index: 0,
                token_standard_map: StorageHashMap::new() 
            };
            instance
        }

        #[ink(message)]
        pub fn register_token_standard_ins(&mut self, token_standard_ins: AccountId) -> Result<()> {
            self.owner = Some(self.env().caller());
            self.index += 1;

            self.token_standard_map.insert(token_standard_ins, self.index);

            // trigger event
            self.env().emit_event(RegisterTokenStandardIns {
                owner: self.owner.clone().unwrap(),
                index: self.index,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn action(&mut self, token_standard_ins: AccountId, air_drop_list: BTreeMap<AccountId, u64>) -> Result<()> {
            // Determine whether the contract is registered
            let _index = self.token_standard_map.get(&token_standard_ins).ok_or(Error::TokenStandardInsNotFound)?;

            let mut token: Erc20 = ink_env::call::FromAccountId::from_account_id(token_standard_ins);
            let count = air_drop_list.len() as u64;
            let mut value: u64 = 0;

            for (to, v) in air_drop_list {
                value = v;
                token.transfer(to, value);
            }

            // trigger event
            self.env().emit_event(AirDropTransfer {
                from: self.owner,
                token_standard_ins_count: 1,
                count,
                value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn action_all(&mut self, air_drop_list: BTreeMap<AccountId, u64>) -> Result<()> { 
            let token_standard_ins_count = self.token_standard_map.len() as u64;
            let count = air_drop_list.len() as u64;
            let mut value = 0;
            let _from = self.env().caller();

            for (token_standard_ins, _) in self.token_standard_map.into_iter() {
                let mut token: Erc20 = ink_env::call::FromAccountId::from_account_id(*token_standard_ins);
                for (to, v) in &air_drop_list {
                    let to = *to;
                    value = *v;
                    token.transfer(to, value);
                }
            }
            // trigger event
            self.env().emit_event(AirDropTransfer {
                from: self.owner,
                token_standard_ins_count,
                count,
                value,
            });
            Ok(())
        }

        
        #[ink(message)]
        pub fn get(&self) -> AirDropInfo {
            AirDropInfo {
                // owner: self.owner.clone().unwrap(),
                index: self.index,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn test_register_token_standard_ins() {
            // let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");
            let mut manager = AirDropManager::new();

            assert!(manager.register_token_standard_ins(AccountId::from([0x00; 32])).is_ok());
        }
    }

}
