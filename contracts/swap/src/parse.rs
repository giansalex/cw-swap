use crate::ContractError;
use cosmwasm_std::{Attribute, Event, SubMsgResponse, Uint128};

pub fn find_event_type(events: Vec<Event>, key: &str) -> Option<Event> {
    for ev in events {
        if ev.ty.eq(&key) {
            return Some(ev);
        }
    }

    None
}

pub fn find_attributes(attributes: Vec<Attribute>, key: &str) -> Vec<String> {
    let mut values = vec![];
    for attr in attributes {
        if attr.key.eq(&key) {
            values.push(attr.value)
        }
    }

    values
}

pub fn parse_token_out(
    msg: SubMsgResponse,
    event: &str,
    attribute: &str,
) -> Result<Uint128, ContractError> {
    let event = find_event_type(msg.events, event);
    if event.is_none() {
        return Err(ContractError::TokenResultNotFound {});
    }

    let values = find_attributes(event.unwrap().attributes, attribute);
    if values.is_empty() {
        return Err(ContractError::TokenResultNotFound {});
    }

    let token_out = values.last().unwrap();
    let amount = token_out
        .parse::<u128>()
        .map_err(|_| ContractError::InvalidAmountValue {})?;

    Ok(amount.into())
}
