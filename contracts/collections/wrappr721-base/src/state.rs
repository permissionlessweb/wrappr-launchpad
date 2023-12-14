use cosmwasm_std::{Empty, Timestamp};
use cw_storage_plus::Item;
use serde::{de::DeserializeOwned, Serialize};
use wrappr721::{CollectionInfo, RoyaltyInfo};
use wrappr_utils::WrapprMsgWrapper;
use std::ops::Deref;

type Parent<'a, T> = cw721_base::Cw721Contract<'a, T, WrapprMsgWrapper, Empty, Empty>;
pub struct Wrappr721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub parent: Parent<'a, T>,
    pub collection_info: Item<'a, CollectionInfo<RoyaltyInfo>>,

    /// Instantiate set to false by the minter, then true by creator to freeze collection info
    pub frozen_collection_info: Item<'a, bool>,
    pub royalty_updated_at: Item<'a, Timestamp>,
}

impl<'a, T> Default for Wrappr721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Wrappr721Contract {
            parent: cw721_base::Cw721Contract::default(),
            collection_info: Item::new("collection_info"),
            frozen_collection_info: Item::new("frozen_collection_info"),
            royalty_updated_at: Item::new("royalty_updated_at"),
        }
    }
}

impl<'a, T> Deref for Wrappr721Contract<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    type Target = Parent<'a, T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
