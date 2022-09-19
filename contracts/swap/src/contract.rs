#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg, coin, Reply, SubMsg, SubMsgResult, IbcMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SwapMsg, SwapRouterMsg};
use crate::parse::parse_token_out;
use crate::state::{CHANNEL_DENOM, CHANNEL_INFO, MsgReplyState, ORDERS, REPLY_STATES, State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    let reply_state = REPLY_STATES.load(deps.storage, reply.id)?;
    let k = (reply_state.channel.as_str(), reply.id);
    let order = ORDERS.load(deps.storage, k)?;
    REPLY_STATES.remove(deps.storage, reply.id);
    ORDERS.remove(deps.storage, k);

    let amount = match reply.result {
        SubMsgResult::Ok(msg) => {
            let token_out = parse_token_out(msg, "wasm", "token_out_amount")?;

            coin(token_out.into(), order.out_denom)
        }
        SubMsgResult::Err(_) => {
            coin(order.amount.into(), order.denom)
        }
    };
    let channel_id = CHANNEL_DENOM.load(deps.storage, amount.denom.as_str())?;
    let state = STATE.load(deps.storage)?;
    let timeout = env.block.time.plus_seconds(state.transfer_timeout);
    let transfer_msg: CosmosMsg = IbcMsg::Transfer {
        channel_id,
        amount,
        to_address: order.sender,
        timeout: timeout.into(),
    }
    .into();

    Ok(Response::new().add_message(transfer_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State {
        owner: info.sender.clone(),
        swap_router: msg.swap_router.to_string(),
        transfer_timeout: msg.transfer_timeout,
    };
    STATE.save(deps.storage, &state)?;

    for item in msg.allowed_list {
        CHANNEL_DENOM.save(deps.storage, item.denom.as_str(), &item.channel)?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CompleteSwap(swap) => execute_complete_swap(deps, info, swap),
    }
}

pub fn execute_complete_swap(
    deps: DepsMut,
    _info: MessageInfo,
    msg: SwapMsg,
) -> Result<Response, ContractError> {
    // TODO: validate user address

    if !CHANNEL_INFO.has(deps.storage, msg.channel.as_str()) {
        return Err(ContractError::ChannelNotFound {});
    }

    // TODO: verify ibc transfer to complete swap
    // PATH /ibc/core/channel/v1/channels/{channel}/ports/{port}/packet_receipts/{sequence}
    let k = (msg.channel.as_str(), msg.sequence.u64());
    let order = ORDERS.load(deps.storage, k).map_err(|_| ContractError::OrderNotFound {})?;
    let state = STATE.load(deps.storage)?;

    let data = SwapRouterMsg::Swap {
        input_coin: coin(order.amount.u128(), order.denom.to_owned()),
        minimum_output_amount: order.min_amount,
        output_denom: order.out_denom,
    };
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: state.swap_router,
        msg: to_binary(&data)?,
        funds: coins(order.amount.u128(), order.denom)
    }.into();

    REPLY_STATES.save(deps.storage, msg.sequence.u64(), &MsgReplyState{
        channel: msg.channel,
    })?;
    let submsg = SubMsg::reply_always(swap_msg, msg.sequence.u64());
    Ok(Response::new()
        .add_submessage(submsg)
        .add_attribute("method", "complete_swap"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {} => to_binary(&query_admin(deps)?),
    }
}

fn query_admin(deps: Deps) -> StdResult<AdminResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(AdminResponse {
        admin: state.owner.into(),
    })
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Api;
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info,
    };

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            swap_router: deps.api.addr_validate("swap-router-addr").unwrap(),
            transfer_timeout: 100u64,
            allowed_list: vec![],
        };

        let info = mock_info("anyone", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
