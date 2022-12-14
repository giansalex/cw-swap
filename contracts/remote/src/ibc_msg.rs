use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Uint128, Uint64};

#[cw_serde]
pub struct SwapPacket {
    pub sender: String,
    pub amount: Uint128,
    pub denom: String,
    pub out_denom: String,
    pub min_amount: Uint128,
    /// Transfer sequence
    pub sequence: Uint64,
}
