use crate::msg::CollectionParams;
use cosmwasm_std::Decimal;
use wrappr721::{CollectionInfo, RoyaltyInfoResponse};

pub fn mock_collection_params() -> CollectionParams {
    CollectionParams {
        code_id: 1,
        name: "Collection Name".to_string(),
        symbol: "COL".to_string(),
        jurisdiction: String::from("Mars"),
        entity: String:: from("Red Rock"),
        info: CollectionInfo {
            creator: "creator".to_string(),
            description: String::from("Flower Flamingos"),
            image: "https://example.com/image.png".to_string(),
            external_link: Some("https://example.com/external.html".to_string()),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "creator".to_string(),
                share: Decimal::percent(10),
            }),
            jurisdiction: String::from("Mars"),
            entity: String:: from("Red Rock"),
        },
    }
}

pub fn mock_collection_params_1() -> CollectionParams {
    CollectionParams {
        code_id: 1,
        name: "Collection Name".to_string(),
        symbol: "COL".to_string(),
        jurisdiction: String::from("Moon"),
        entity: String:: from("Dusty LLC"),
        info: CollectionInfo {
            creator: "creator".to_string(),
            description: String::from("Flower Flamingos"),
            image: "https://example.com/image.png".to_string(),
            external_link: Some("https://example.com/external.html".to_string()),
            jurisdiction: String::from("Moon"),
            entity: String:: from("Dusty LLC"),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "creator".to_string(),
                share: Decimal::percent(10),
            }),
            
        },
    }
}

pub fn mock_curator_payment_address() -> CollectionParams {
    CollectionParams {
        code_id: 1,
        name: String::from("Test Coin"),
        symbol: String::from("TEST"),
        jurisdiction: String::from("Moon"),
        entity: String:: from("Dusty LLC"),
        info: CollectionInfo {
            creator: "creator".to_string(),
            description: String::from("Flower Flamingos"),
            image: "https://example.com/image.png".to_string(),
            external_link: Some("https://example.com/external.html".to_string()),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "curator".to_string(),
                share: Decimal::percent(10),
            }),
            jurisdiction: String::from("Mars"),
            entity: String:: from("Red Rock Specialist Certificate"),
        },
    }
}

pub fn mock_collection_params_high_fee() -> CollectionParams {
    CollectionParams {
        code_id: 1,
        name: String::from("Test Coin"),
        symbol: String::from("TEST"),
        jurisdiction: String::from("Moon"),
        entity: String:: from("Dusty LLC"),
        info: CollectionInfo {
            creator: "creator".to_string(),
            description: String::from("Flower Flamingos"),
            image:
                "ipfs://bafybeigi3bwpvyvsmnbj46ra4hyffcxdeaj6ntfk5jpic5mx27x6ih2qvq/images/1.png"
                    .to_string(),
            external_link: Some("https://example.com/external.html".to_string()),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "creator".to_string(),
                share: Decimal::percent(100),
            }),
            jurisdiction: String::from("Jupiter"),
            entity: String:: from("Big Gas Non-Profit Cooperation"),
        },
    }
}

pub fn mock_collection_two() -> CollectionParams {
    CollectionParams {
        code_id: 1,
        name: String::from("Test Collection 2"),
        symbol: String::from("TEST 2"),
        jurisdiction: String::from("Pluto"),
        entity: String:: from("Independent"),
        info: CollectionInfo {
            creator: "creator".to_string(),
            description: String::from("Flower Flamingos 2"),
            image:
                "ipfs://bafybeigi3bwpvyvsmnbj46ra4hyffcxdeaj6ntfk5jpic5mx27x6ih2qvq/images/1.png"
                    .to_string(),
            external_link: Some("https://example.com/external.html".to_string()),
            royalty_info: Some(RoyaltyInfoResponse {
                payment_address: "creator".to_string(),
                share: Decimal::percent(10),
            }),
            jurisdiction: String::from("Pluto"),
            entity: String:: from("Independent"),
        },
    }
}
