# Cw-Wrapper Contract

Welcome! This repo contains the [CosmWasm](https://cosmwasm.com/) implementation of [Wrappr](https://www.wrappr.wtf/).

## TODO:
- Verify data types [u64,Addr,bool]
- Decide how to implement managers, permissions, and registered
- Finish Execute Message Logic
    - requre message sender to only be owner of, or admin.
    - require only the admin for this msg. 
    - validate token id is transferrable.
    - check for permissions
- Setup contract queries
- Setup contract state & storage
- Create Tests

## Contributors
- Hard-nett, Permissionlessweb