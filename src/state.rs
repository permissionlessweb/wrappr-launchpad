use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item,Map};

#[cw_serde]
pub struct Config {
    // cw4 contract address for admins of a wrappr.
    pub admins_cw4_group: Addr,
    // cw4 contract address for managers of a wrappr.
    pub managers_cw4_group: Addr
}
pub const CONFIG: Item<Config> = Item::new("config");

/// Store the balance map, `(owner, token_id) -> balance`
pub const OWNER_OF: Map<(&str, &[u64]), Uint128> = Map::new("owner_of");

/// The price to mint a new NFT
pub const MINT_FEE: Item<Coin> = Item::new("mint_fee");

/// The base url used for token metadata
pub const BASE_URI: Item<String> = Item::new("base_uri");


/// Previous token id, represent the last NFT token ID that was minted
pub const ID: Item<u64> = Item::new("token_id");