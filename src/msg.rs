use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

use crate::state::{Amounts,Ids};

#[cw_serde]
pub struct InstantiateMsg {

    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {

    Mint { 
        to: Addr,          // address to,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,
                           // bytes calldata data,
        token_uri: String, // string calldata tokenURI,
        owner: Addr,       // address owner
    },

    /// MANAGEMENT FUNCTIONS

    Burn{
        from: Addr,        // address from,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,
    },

    ManageMint{
        to: Addr,          // address to,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,
                           // bytes calldata data,
        token_uri: String, // string calldata tokenURI,
        owner: Addr,       // address owner
    },

    ManageBurn{
        from: Addr,        // address from,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,
    },

    /// OWNER FUNCTIONS

    SetOwnerOf{           
        to: Addr,          // address to,
        id: u64,           // uint256 id,
    },

    SetTransferability{
        id: u64,           // uint256 id,
        set: bool,         // bool set
        
    },

    SetPermissions{
        id:  u64,          // uint256 id,
        set: bool,         // bool set
    },

    SetUserPermissions{
        to: Addr,     // address to,
        id: u64,           // uint256 id,
        set: bool,         // bool set
    },
    SetURI{
        id: u64,            // uint256 id, 
        token_uri: String,  // string calldata tokenURI
    },
    SetUserURI{
        to: Addr,           // address to,
        id: u64,            // uint256 id, 
        user_uri: String    // string calldata uuri
    },

    /// ADMIN FUNCTIONS

    SetManager{
        to: Addr,           // address to,
        manager: bool       // bool set
    },

    SetAdmin{
        to:    Addr,        // address to,
        admin: Addr         // address _admin
    },
    SetBaseURI{
        base_uri: String,   // string calldata _baseURI,
    },

    SetMintFee{
        mint_fee: u64,      // uint256 _mintFee
    },

    ClaimFee{
        to:     Addr,       // address to,
        amount: u64,        // uint256 amount,
    },

    /// TRANSFER FUNCTIONS

    SafeTransferFrom{
        from:   Addr,       // address from,
        to:     Addr,       // address to,
        id:     u64,        // uint256 id,
        amount: u64,        // uint256 amount,
                            // bytes calldata data,
    },

    SafeBatchTransferFrom{
        from:      Addr,           // address from,
        to:        Addr,           // address to,
        ids:       Ids,       // uint256[] calldata ids,
        amounts:   Amounts,   // uint256[] calldata amounts,
                                   // bytes calldata data,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
