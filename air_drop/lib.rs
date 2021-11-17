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
    use ink_prelude::vec::Vec;

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
        owner: Option<AccountId>,
        count: u64,
        token_standard_map: BTreeMap<AccountId, u64>,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    pub enum AirDropRespStatus {
        Success,
        Failed,
        Interrupt,
    }

    #[ink(storage)]
    pub struct AirDropManager {
        owner: Option<AccountId>,
        count: u64,
        // the list of token standard ins address
        token_standard_map: BTreeMap<AccountId, u64>,
    }

    #[ink(event)]
    pub struct RegisterTokenStandardIns {
        #[ink(topic)]
        owner: AccountId,
        index: u64,
        token_standard_info: AccountId,
    }

    #[ink(event)]
    pub struct AirDropTransfer {
        #[ink(topic)]
        from: Option<AccountId>,
        // token standard instance count
        token_standard_ins_count: u64,
        transfer_address_count: u64,
        // the total value of air drop token
        total_value: u64,
        response_status: BTreeMap<AccountId, AirDropRespStatus>,
        unsuccessful_transfer_address_map: BTreeMap<AccountId, Vec<AccountId>>,

    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        RepeatRegisterTokenStandardIns,
        // RegisterTokenStandardInsError,
        TokenStandardInsNotFound,
        AirDropAccountBalanceInsufficient,
        TotalOfAirDropAccountBalanceInsufficient,
    }

    // result type
    pub type Result<T> = core::result::Result<T, Error>;

    impl AirDropManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self {
                owner: Some(Self::env().caller()),
                count: 0,
                token_standard_map: BTreeMap::new() 
            };
            instance
        }

        #[ink(message)]
        pub fn register_token_standard_ins(&mut self, token_standard_ins: AccountId) -> Result<()> {
            // self.owner = Some(self.env().caller());
            // Determine whether the instance is registered repeatedly
            if self.token_standard_map.contains_key(&token_standard_ins) {
                return Err(Error::RepeatRegisterTokenStandardIns);
            }
            self.count += 1;
            self.token_standard_map.insert(token_standard_ins, self.count - 1);

            // trigger event
            self.env().emit_event(RegisterTokenStandardIns {
                owner: self.owner.clone().unwrap_or(AccountId::from([0x00; 32])),
                index: self.count - 1,
                token_standard_info: token_standard_ins,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn action(&mut self, token_standard_ins: AccountId, air_drop_list: BTreeMap<AccountId, u64>) -> Result<()> {
            // Determine whether the contract is registered
            if !self.token_standard_map.contains_key(&token_standard_ins) {
                return Err(Error::TokenStandardInsNotFound);
            }

            let mut token: Erc20 = ink_env::call::FromAccountId::from_account_id(token_standard_ins);
            let count = air_drop_list.len() as u64;
            // let mut value: u64 = 0;
            let mut total_value: u64 = 0;
            let mut response_status: BTreeMap<AccountId, AirDropRespStatus> = BTreeMap::new();
            let mut unsuccessful_transfer_address_list: Vec<AccountId> = Vec::new();
            let mut unsuccessful_transfer_address_map: BTreeMap<AccountId, Vec<AccountId>> = BTreeMap::new();

            // compute the total value of the airdrop 
            for (_, v) in &air_drop_list {
                total_value += *v;
            }
            if total_value > token.balance_of(self.env().account_id()) {
                response_status.insert(token_standard_ins, AirDropRespStatus::Failed);
                return Err(Error::AirDropAccountBalanceInsufficient);
            } else {
                // invoke air_drop
                for (to, value) in &air_drop_list {
                    let result = token.transfer(*to, *value);
                    if !result {
                        // Record the address where the airdrop failed
                        unsuccessful_transfer_address_list.push(*to);
                    }
                }
                unsuccessful_transfer_address_map.insert(token_standard_ins, unsuccessful_transfer_address_list);
                match unsuccessful_transfer_address_map.get(&token_standard_ins) {
                    Some(v) => {
                        if v.len() == 0 {
                            response_status.insert(token_standard_ins, AirDropRespStatus::Success);
                        } else {
                            response_status.insert(token_standard_ins, AirDropRespStatus::Interrupt);
                        }
                    },
                    None => {
                        response_status.insert(token_standard_ins, AirDropRespStatus::Success);
                    },
                } 
            }

            // trigger event
            self.env().emit_event(AirDropTransfer {
                from: Some(self.env().account_id()),
                token_standard_ins_count: 1,
                transfer_address_count: count,
                total_value,
                response_status,
                unsuccessful_transfer_address_map,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn action_all(&mut self, air_drop_list: BTreeMap<AccountId, u64>) -> Result<()> { 
            let token_standard_ins_count = self.token_standard_map.len() as u64;
            if token_standard_ins_count == 0 {
                return Err(Error::TokenStandardInsNotFound);
            }

            let count = air_drop_list.len() as u64;
            // let mut value: u64 = 0;
            let mut total_value: u64 = 0;
            let mut response_status: BTreeMap<AccountId, AirDropRespStatus> = BTreeMap::new();
            let mut unsuccessful_transfer_address_map: BTreeMap<AccountId, Vec<AccountId>> = BTreeMap::new();

            for (token_standard_ins, _) in self.token_standard_map.iter() {
                let mut token: Erc20 = ink_env::call::FromAccountId::from_account_id(*token_standard_ins);
                let mut unsuccessful_transfer_address_list: Vec<AccountId> = Vec::new();

                // compute the total value of the airdrop 
                for (_, value) in &air_drop_list {
                    total_value += *value;
                }

                if total_value > token.balance_of(self.env().account_id()) {
                    response_status.insert(*token_standard_ins, AirDropRespStatus::Failed);
                    // return Err(Error::AirDropAccountBalanceInsufficient);
                } else {
                    // invoke air_drop
                    for (to, value) in &air_drop_list {
                        let result = token.transfer(*to, *value);
                        if !result {
                            // Record the address where the airdrop failed
                            unsuccessful_transfer_address_list.push(*to);
                        }
                    }
                    unsuccessful_transfer_address_map.insert(*token_standard_ins, unsuccessful_transfer_address_list);
                    match unsuccessful_transfer_address_map.get(&token_standard_ins) {
                        Some(v) => {
                            if v.len() == 0 {
                                response_status.insert(*token_standard_ins, AirDropRespStatus::Success);
                            } else {
                                response_status.insert(*token_standard_ins, AirDropRespStatus::Interrupt);
                            }
                        },
                        None => {
                            response_status.insert(*token_standard_ins, AirDropRespStatus::Success);
                        },
                    }
                }
            }
            let mut failed_num: u64 = 0;
            for (_, v) in response_status.iter() {
                match v {
                    AirDropRespStatus::Success => {
                        break;
                    },
                    AirDropRespStatus::Interrupt => {
                        break;
                    },
                    AirDropRespStatus::Failed => {
                        failed_num += 1;
                    },
                }
            }
            if failed_num == token_standard_ins_count {
                return Err(Error::TotalOfAirDropAccountBalanceInsufficient);
            }

            // trigger event
            self.env().emit_event(AirDropTransfer {
                from: Some(self.env().account_id()),
                token_standard_ins_count,
                transfer_address_count: count,
                total_value,
                response_status,
                unsuccessful_transfer_address_map,
            });
            Ok(())
        }

        
        #[ink(message)]
        pub fn get(&self) -> AirDropInfo {
            AirDropInfo {
                owner: self.owner,
                count: self.count,
                token_standard_map: self.token_standard_map.clone(),
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
