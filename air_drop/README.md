# SubDAO airdrop Module
airdrop is a tool module for batch transfer of erc20 protocol tokens.

## Modules
### AirDropManager
```
pub struct AirDropManager {
    owner: Option<AccountId>,
    index: u64,
    // the list of token standard ins address
    token_standard_map: StorageHashMap<AccountId, u64>,
}
```

## interfaces
### instance module
```
type: tx
definition: pub fn new() -> Self
```

### register token standard instance
register ERC20 contract instance
```
type: tx
definition: pub fn register_token_standard_ins(&mut self, token_standard_ins: AccountId) -> Result<()>
```

### action
Send a list of airdrops to a registered erc20 contract instance
```
type: tx
definition: pub fn action(&mut self, token_standard_ins: AccountId, air_drop_list: BTreeMap<AccountId, u64>) -> Result<()>
```

### action all
Send a list of airdrops to all registered erc20 contract instances
```
type: tx
definition: pub fn action_all(&mut self, air_drop_list: BTreeMap<AccountId, u64>) -> Result<()>
```

### get
Get the number of registered erc20 contract instances
```
type: query
definition: pub fn get(&self) -> AirDropInfo
```
