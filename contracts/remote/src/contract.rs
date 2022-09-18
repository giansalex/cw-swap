#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, IbcMsg, coin, Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::ibc_msg::SwapPacket;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SwapMsg};
use crate::state::{CHANNEL_DENOM, Config, CONFIG};
use cw_utils::one_coin;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-swap-remote";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let cfg = Config {
        owner: info.sender.clone(),
        swap_contract: msg.swap_contract,
        default_timeout: msg.default_timeout,
    };
    CONFIG.save(deps.storage, &cfg)?;

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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: SwapMsg,
    transfer: Coin,
) -> Result<Response, ContractError> {
    let swap_channel = CHANNEL_DENOM.load(deps.storage, transfer.denom.as_str())?;
    let cfg = CONFIG.load(deps.storage)?;

    let timeout_delta = match msg.timeout {
        Some(t) => t,
        None => cfg.default_timeout,
    };
    let timeout = env.block.time.plus_seconds(timeout_delta);
    let packet = SwapPacket {
        sender: info.sender.to_string(),
        amount: transfer.amount,
        denom: transfer.denom.to_owned(),
        out_denom: msg.denom,
        min_amount: msg.min_amount,
        sequence: 1u128.into(), // get from IBC data or query channel next_sequence
    };
    // TODO: validate whitelist transfer channels
    let transfer_msg: CosmosMsg = IbcMsg::Transfer {
        channel_id: msg.channel,
        amount: coin(transfer.amount.into(), transfer.denom.to_owned()),
        to_address: cfg.swap_contract,
        timeout: timeout.into(),
    }
    .into();

    let swap_msg = IbcMsg::SendPacket {
        channel_id: swap_channel,
        data: to_binary(&packet)?,
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
    let cfg = CONFIG.load(deps.storage)?;

    Ok(AdminResponse {
        admin: cfg.owner.into(),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info,
    };
    use crate::msg::AllowedDenom;

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies();

        let osmo_denom = AllowedDenom {
            denom: "ibc/AAAAA".into(),
            channel: "channel-0".into(),
        };
        let msg = InstantiateMsg {
            swap_contract: "swap-addr".into(),
            default_timeout: 100,
            allowed_list: vec![osmo_denom],
        };

        let info = mock_info("anyone", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
