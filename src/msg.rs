use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {

    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {

    Mint( 
    address: Addr,     // address to,
    id: u64,           // uint256 id,
    amount: u64,       // uint256 amount,
                       // bytes calldata data,
    token_uri: String, // string calldata tokenURI,
    owner: Addr,       // address owner
    )

    /// MANAGEMENT FUNCTIONS

    Burn(
        address: Addr,     // address from,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,
    )

    ManageMint(
        address: Addr,     // address to,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,
                           // bytes calldata data,
        token_uri: String, // string calldata tokenURI,
        owner: Addr,       // address owner
    )

    ManageBurn(
        address: Addr,     // address from,
        id: u64,           // uint256 id,
        amount: u64,       // uint256 amount,

    )

    /// OWNER FUNCTIONS

    SetOwnerOf(            
        address: Addr,     // address to,
        id: u64,           // uint256 id,
    )

    SetTransferability(
        address: Addr,     // address to,
        id: u64,           // uint256 id,
        
    )

    SetPermissions(
        id:  u64,          // uint256 id,
        set: bool,         // bool set
    )

    SetUserPermissions(
        address: Addr,   // address to,
        id: u64,           // uint256 id,
        set: bool,         // bool set
    )

    SetUserURI(
        address: Addr,     // address to,
        to: Addr,          // address to,
        user_uri: String   // string calldata uuri


    )

    /// ADMIN FUNCTIONS

    SetManager(
        to: Addr,           // address to,
        manager: bool       // bool set

    )

    SetAdmin(
        to:    Addr,       // address to,
        admin: Addr        // address _admin
    )

    SetBaseURI(
        base_uri: String,   // string calldata _baseURI,
    )

    SetMintFee(
        mint_fee: u64,       // uint256 _mintFee
    )

    ClaimFee(
        to:     Addr,       // address to,
        amount: u64,        // uint256 amount,
    )

    /// TRANSFER FUNCTIONS

    SafeTransferFrom(
        from:   Addr,       // address from,
        to:     Addr,       // address to,
        id:     u64,        // uint256 id,
        amount: u64,        // uint256 amount,
                            // bytes calldata data,
    )

    SafeBatchTransferFrom(
        from:      Addr,           // address from,
        to:        Addr,           // address to,
        ids:       Vec<Ids>,       // uint256[] calldata ids,
        amounts:   Vec<Amounts>,   // uint256[] calldata amounts,
                                   // bytes calldata data,
    )
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
