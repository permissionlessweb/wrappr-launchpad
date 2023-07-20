# Cw-Wrapper Contract

Welcome! This repo contains the [CosmWasm](https://cosmwasm.com/) implementation of [Wrappr](https://www.wrappr.wtf/).

## Workflow

1. Cw-factory is instantiated with an admin
2. Cw-factory is called only by admin for executing functions:
    - 2.1: Mint instantiates up to 3 contracts in one message (admins_cw4_group, managers_cw4_group, cw-wrappr)

## Contributors
- Hard-nett, Permissionlessweb

