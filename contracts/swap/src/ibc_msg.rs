use cosmwasm_std::Binary;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Ics20Ack {
    Result(Binary),
    Error(String),
}
