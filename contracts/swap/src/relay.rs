use cosmwasm_std::{Binary, DepsMut, Env, from_binary, IbcPacket, IbcReceiveResponse, to_binary};
use crate::ContractError;
use crate::ibc_msg::Ics20Ack;
use crate::state::{Order, ORDERS};

// create a serialized success message
pub fn ack_success() -> Binary {
    let res = Ics20Ack::Result(b"1".into());
    to_binary(&res).unwrap()
}

// create a serialized error message
pub fn ack_fail(err: String) -> Binary {
    let res = Ics20Ack::Error(err);
    to_binary(&res).unwrap()
}

pub fn handle_ibc_receive(
    deps: DepsMut,
    _env: Env,
    packet: &IbcPacket,
) -> Result<IbcReceiveResponse, ContractError> {
    let data: Order = from_binary(&packet.data)?;
    // TODO: verify ibc transfer to complete swap
    // PATH /ibc/core/channel/v1/channels/{channel}/ports/{port}/packet_receipts/{sequence}
    let k = (packet.dest.channel_id.as_ref(), data.sequence.u64());
    ORDERS.save(deps.storage, k, &data)?;

    let res = IbcReceiveResponse::new()
        .set_ack(ack_success())
        .add_attribute("action", "receive_swap")
        .add_attribute("sender", data.sender)
        .add_attribute("denom", data.denom)
        .add_attribute("amount", data.amount)
        .add_attribute("success", "true");

    Ok(res)
}
