use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, IbcEndpoint, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct State {
    pub owner: Addr,
    pub swap_router: String,
    pub transfer_timeout: u64,
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

#[cw_serde]
pub struct Order {
    pub sender: String,
    pub amount: Uint128,
    pub denom: String,
    pub out_denom: String,
    pub min_amount: Uint128,
    /// Transfer sequence
    pub sequence: Uint64,
}

#[cw_serde]
pub struct MsgReplyState {
    pub channel: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const CHANNEL_INFO: Map<&str, ChannelInfo> = Map::new("channel_info");
pub const ORDERS: Map<(&str, u64), Order> = Map::new("swap_orders");
pub const REPLY_STATES: Map<u64, MsgReplyState> = Map::new("reply_states");
pub const CHANNEL_DENOM: Map<&str, String> = Map::new("channel_denom");
