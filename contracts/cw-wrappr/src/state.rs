use cosmwasm_std::{Empty, Timestamp,CustomMsg};
use cw_storage_plus::Item;
use serde::{de::DeserializeOwned, Serialize};
use wrappr_utils::CollectionInfo;
use std::ops::Deref;

pub const NATIVE_DENOM: &str = "ujuno";

type Parent<'a, T> = cw721_base::Cw721Contract<'a, T, Empty, Empty, Empty>;
pub struct Cw721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub parent: Parent<'a, T>,
    pub collection_info: Item<'a, CollectionInfo>,

    /// Instantiate set to false by the minter, then true by creator to freeze collection info
    pub frozen_collection_info: Item<'a, bool>,
    // pub royalty_updated_at: Item<'a, Timestamp>,
}

impl<'a, T> Default for Cw721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Cw721Contract {
            parent: cw721_base::Cw721Contract::default(),
            collection_info: Item::new("collection_info"),
            frozen_collection_info: Item::new("frozen_collection_info"),
        }
    }
}

impl<'a, T> Deref for Cw721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    type Target = Parent<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
