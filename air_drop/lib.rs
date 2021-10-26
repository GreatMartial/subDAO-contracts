#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod air_drop {
    use ink_lang::EmitEvent;
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
    
    #[ink(storage)]
    pub struct AirDropManager {
        /// token standard name and contract code hash
        owner: Option<AccountId>,
        index: u64,
        token_standard_map: StorageHashMap<u64, TokenStandardIns>,
    }

/*     #[ink(event)]
    pub struct AirDropTransfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: u64,
    }

    #[ink(event)]
    pub struct AirDropApproval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: u64,
    }

    #[ink(event)]
    pub struct AirDropInstance {
        #[ink(topic)]
        token_standard_name: Option<String>,
    }
*/
    #[ink(event)]
    pub struct AirDropTransfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        count: u64,
        #[ink(topic)]
        // By default, the funds transferred from the airdrop to each address are the same. 
        value: u64,
    }

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
        pub fn create_token_standard_ins(&mut self, token_addr: AccountId) -> u64 {
            self.owner = Some(self.env().caller());
            self.index += 1;

            let token_standard_ins = TokenStandardIns{token_standard_addr: token_addr,};
            self.token_standard_map.insert(self.index, token_standard_ins);

            self.index
        }

        #[ink(message)]
        pub fn action(&mut self, token_addr: AccountId, air_drop_list: BTreeMap<AccountId, u64>) -> bool {
            let mut token: Erc20 = ink_env::call::FromAccountId::from_account_id(token_addr);
            let count = air_drop_list.len() as u64;
            for (to, value) in air_drop_list {
                token.transfer(to, value);
            }

            self.env().emit_event(AirDropTransfer {
                from: self.owner,
                count,
                value: 0
            });
            true
        }

        #[ink(message)]
        pub fn action_all(&mut self, air_drop_list: BTreeMap<AccountId, u64>) -> bool { 
            for (_, token_standard_ins) in self.token_standard_map.into_iter() {
                let mut token: Erc20 = ink_env::call::FromAccountId::from_account_id(token_standard_ins.token_standard_addr);
                for (to, value) in &air_drop_list {
                    token.transfer(*to, *value);
                }
            }
            true
        }
    }

/* 
    impl AirDrop {
        // new function
        #[ink(constructor)]
        pub fn new(
            token_standard: String,
            erc20_code_hash: Hash,
            name: String,
            symbol: String,
            initial_supply: u64, 
            decimals: u8, 
            controller: AccountId
        ) -> Self {
            let owner = Self::env().caller();
            let total_balance = Self::env().balance();
            let mut info = TokenStandardInstance{ erc20: None };
            let token_standard_name: String;
            // match &token_standard.to_string()[..] {
            match &token_standard.to_string()[..] {    
                "erc20" | "ERC20" => {
                    let erc20_instance = Erc20::new(name, symbol, initial_supply, decimals, controller)
                        .endowment(total_balance / 4)
                        .code_hash(erc20_code_hash)
                        //.salt_bytes(1_i32.to_le_bytes())
                        .instantiate()
                        .expect("failed at instantiating the `Erc20` contract.");
                        info.erc20 = Some(erc20_instance);

                        token_standard_name = "ERC20".to_string();
                },
                // "erc721" | "ERC721" => {
                //    TODO:
                // },
                _ => {
                    // TODO:
                },
            }    
            // trigger event
            self.env().emit_event(AirDropInstance {
                token_standard_name: Some(token_standard_name),
            });
            //info.insert(token_standard, token_standard_instance);
            Self { owner, info }  
        }

        // query airDrop info
        #[ink(message)]
        pub fn get(&self, account_id: AccountId) -> u64 {
            if let Some(er20_instance) = self.info.erc20.clone() {
                return er20_instance.balance_of(account_id);
            }
            0
        }

        // do function
        #[ink(message)]
        pub fn invoke_list(&mut self, _token_standard: String, address_list: BTreeMap<AccountId, u64>) -> bool {
            if let Some(mut erc20_instance) = self.info.erc20.clone() {
                let mut total_value = 0 as u64;
                for (_, value) in &address_list {
                    total_value += value
                }

                erc20_instance.approve_from(self.env().caller(), self.env().account_id(), total_value);

                let count: u64;
                let amount: u64;
                for (spender, value) in &address_list {
                    let spender = *spender;
                    let value = *value;
                    count += 1;

                    ink_env::debug_println!("wasm contract is air_drop: address {:?}, value {}", spender, value);
                    erc20_instance.transfer_from(self.env().caller(), spender, value);
                }
                // ink_env::debug_println!("wasm contract is air_drop: contract_id {:?}", self.env().account_id());

                // trigger event
                self.env().emit_event(AirDropTransfer {
                    from: Some(self.env().caller()),
                    count,
                    value: amount,

                });
                return true;
            }
                // TODO: erc721 tranfer
                // if let Some(erc721_instance) = v.erc721 {
                    // for (spender, value) in address_list {
                        // erc721_instancev.transfer_from(Self.env().caller(), spender, value)
                    // }
                // }
            false    
        }
    }
    */
}

