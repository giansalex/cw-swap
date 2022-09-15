use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, IbcEndpoint};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    /// Remote contract address
    pub swap_contract: String,
    pub default_timeout: u64,
}

#[cw_serde]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");

pub const CHANNEL_DENOM: Map<&str, String> = Map::new("channel_denom");
