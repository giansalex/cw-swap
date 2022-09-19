use cosmwasm_std::{Binary, DepsMut, Env, from_binary, IbcPacket, IbcReceiveResponse, to_binary};
use crate::ContractError;
use crate::ibc_msg::Ics20Ack;
use crate::state::{CHANNEL_DENOM, Order, ORDERS};

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
    let order: Order = from_binary(&packet.data)?;
    // TODO: complete swap here? prev verify ibc transfer

    if !CHANNEL_DENOM.has(deps.storage, &order.denom) {
        return Err(ContractError::DenomNotAllowed {denom: order.denom});
    }
    if !CHANNEL_DENOM.has(deps.storage, &order.out_denom) {
        return Err(ContractError::DenomNotAllowed {denom: order.out_denom});
    }

    let k = (packet.dest.channel_id.as_ref(), order.sequence.u64());
    ORDERS.save(deps.storage, k, &order)?;

    let res = IbcReceiveResponse::new()
        .set_ack(ack_success())
        .add_attribute("action", "receive_swap")
        .add_attribute("sender", order.sender)
        .add_attribute("denom", order.denom)
        .add_attribute("amount", order.amount)
        .add_attribute("success", "true");

    Ok(res)
}
