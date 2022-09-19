use cosmwasm_std::{Addr, Coin, Uint128, Uint64};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub swap_router: Addr,
    pub transfer_timeout: u64,
    pub allowed_list: Vec<AllowedDenom>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CompleteSwap(SwapMsg),
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
    pub sequence: Uint64,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}

#[cw_serde]
pub enum SwapRouterMsg {
    Swap {
        input_coin: Coin,
        output_denom: String,
        minimum_output_amount: Uint128,
    },
}

#[cw_serde]
pub struct AllowedDenom {
    pub denom: String,
    pub channel: String,
}
