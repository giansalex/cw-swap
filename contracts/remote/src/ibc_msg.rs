use cosmwasm_std::Uint128;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct SwapPacket {
    pub sender: String,
    pub amount: Uint128,
    pub denom: String,
    pub out_denom: String,
    pub min_amount: Uint128,
    /// Transfer sequence
    pub sequence: Uint128,
}
