use cosmwasm_std::Uint128;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    /// Remote contract address
    pub swap_contract: String,
    pub default_timeout: u64,
    pub allowed_list: Vec<AllowedDenom>,
}

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
pub struct AllowedDenom {
    pub denom: String,
    pub channel: String,
}

#[cw_serde]
pub struct SwapMsg {
    pub channel: String,
    pub denom: String,
    pub min_amount: Uint128,
    pub timeout: Option<u64>,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}
