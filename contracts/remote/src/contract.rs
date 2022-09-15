#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, IbcMsg, coin, Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::ibc_msg::SwapPacket;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SwapMsg};
use crate::state::{State, STATE};
use cw_utils::one_coin;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-swap-remote";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let state = State {
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Swap(swap) => {
            let transfer = one_coin(&info)?;
            execute_swap(deps, env, info, swap, transfer)
        },
    }
}

pub fn execute_swap(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SwapMsg,
    transfer: Coin,
) -> Result<Response, ContractError> {
    // let state = STATE.load(deps.storage)?;

    let timeout_delta = 600u64;
    let timeout = env.block.time.plus_seconds(timeout_delta);
    let packet = SwapPacket {
        sender: info.sender.to_string(),
        amount: transfer.amount,
        denom: transfer.denom.to_owned(),
        out_denom: msg.denom,
        min_amount: msg.min_amount,
    };

    let swap_msg = IbcMsg::SendPacket {
        channel_id: msg.channel,
        data: to_binary(&packet)?,
        timeout: timeout.into(),
    }
    .into();

    let transfer_msg: CosmosMsg = IbcMsg::Transfer {
        channel_id: "channel-1".into(),
        amount: coin(transfer.amount.into(), transfer.denom.to_owned()),
        to_address: "".into(),
        timeout: timeout.into(),
    }
    .into();

    Ok(Response::new()
        .add_messages(vec![transfer_msg, swap_msg])
        .add_attribute("method", "swap")
        .add_attribute("amount", transfer.amount)
        .add_attribute("denom", transfer.denom))
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
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info,
    };

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
        };

        let info = mock_info("anyone", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
