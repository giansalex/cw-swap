#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AdminResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TransferMsg};
use crate::state::{State, STATE};

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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer(transfer) => execute_transfer(deps, info, transfer),
    }
}

pub fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    transfer: TransferMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    if state.owner.ne(&info.sender) {
        return Err(ContractError::Unauthorized {});
    }

    let msg: CosmosMsg = BankMsg::Send {
        amount: coins(transfer.amount.into(), transfer.denom.to_owned()),
        to_address: transfer.to,
    }
    .into();

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "transfer")
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
