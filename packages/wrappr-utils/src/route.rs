use cosmwasm_schema::cw_serde;

/// WrapprRoute is enum type to represent stargaze query route path
#[cw_serde]
pub enum WrapprRoute {
    Distribution,
}
