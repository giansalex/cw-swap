use cosmwasm_std::{Addr, Coin, Uint128};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub swap_router: Addr,
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
    pub sequence: Uint128,
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
