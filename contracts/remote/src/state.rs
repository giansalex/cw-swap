use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    /// Remote contract address
    pub swap_contract: String,
    pub default_timeout: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const CHANNEL_DENOM: Map<&str, String> = Map::new("channel_denom");
