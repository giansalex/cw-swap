use cosmwasm_std::Uint128;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Swap(SwapMsg),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AdminResponse)]
    Admin {},
}

#[cw_serde]
pub struct SwapMsg {
    pub channel: String,
    pub denom: String,
    pub min_amount: Uint128,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}
