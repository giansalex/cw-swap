#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg, coin, Reply, SubMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SwapMsg, SwapRouterMsg};
use crate::state::{CHANNEL_INFO, ORDERS, State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const SWAP_ID: u64 = 0x1237;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _reply: Reply) -> Result<Response, ContractError> {
    Ok(Response::new())
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
    };
    STATE.save(deps.storage, &state)?;

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

    let k = (msg.channel.as_str(), msg.sequence.u128());
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

    let submsg = SubMsg::reply_always(swap_msg, SWAP_ID);
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
        };

        let info = mock_info("anyone", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
