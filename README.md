# Cw-Wrapper Contract

Welcome! This repo contains the [CosmWasm](https://cosmwasm.com/) implementation of [Wrappr](https://www.wrappr.wtf/).

## TODO:
- Verify data types [u64,Addr,bool]
- Decide how to implement managers, permissions, and registered
- Implement ExecuteMsg Logic
- Setup contract state & storage
- Setup contract queries
- Create Tests

### ExecuteMsgs
- `execute_mint`
- `execute_burn`
- `execute_manage_mint`
- `execute_manage_burn`
- `execute_set_owner_of`
- `execute_set_transferability`
- `execute_set_permissions`
- `execute_set_user_permissions`
- `execute_set_uri`
- `execute_set_user_uri`
- `execute_set_manager`
- `execute_set_admin`
- `execute_set_base_uri`
- `execute_set_mint_fee`
- `execute_claim_fee`
- `execute_safe_transfer_from`
- `execute_safe_batch_transfer_from`

### Queries
- `admin`
- `manager`
- `name`
- `ownerOf`
- `permissioned`
- `symbol`
- `totalSupply`
- `transferrable`
- `uri`
- `userURI`
- `userPermissioned`

## Contributors
- Hard-nett, Permissionlessweb

