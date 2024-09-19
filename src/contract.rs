#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response,
    StdResult,
};
use cw_ownable::{assert_owner, get_ownership, update_ownership};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, LAST_CLAIM};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:prefunded-faucet";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::InvalidFunds {
            reason: "Expected exactly one denom".to_string(),
        });
    }

    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;

    let denom = info.funds[0].denom.clone();

    CONFIG.save(
        deps.storage,
        &Config {
            denom,
            claim_size: msg.claim_size,
            claim_wait_time: msg.claim_wait_time,
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwnership(action) => {
            update_ownership(deps, &env.block, &info.sender, action)?;
            Ok(Response::new())
        }
        ExecuteMsg::Claim {} => execute_claim(deps, env, info),
        ExecuteMsg::UpdateConfig {
            claim_size,
            claim_wait_time,
        } => execute_update_config(deps, env, info, claim_size, claim_wait_time),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ownership {} => to_json_binary(&get_ownership(deps.storage)?),
    }
}

pub fn execute_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let last_claim = LAST_CLAIM.load(deps.storage, info.sender.clone())?;

    let block_time = env.block.time.seconds();
    if block_time - last_claim < config.claim_wait_time {
        return Err(ContractError::InsufficientWait {
            wait_time: config.claim_wait_time - (block_time - last_claim),
        });
    }

    // Update last claim time
    LAST_CLAIM.save(deps.storage, info.sender.clone(), &block_time)?;

    // Send tokens to user
    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![coin(config.claim_size, &config.denom)],
    };

    let event = Event::new("claim")
        .add_attribute("amount", config.claim_size.to_string())
        .add_attribute("denom", config.denom)
        .add_attribute("recipient", info.sender.as_str());

    Ok(Response::new().add_message(msg).add_event(event))
}

pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    claim_size: Option<u128>,
    claim_wait_time: Option<u64>,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let mut config = CONFIG.load(deps.storage)?;

    if let Some(claim_size) = claim_size {
        config.claim_size = claim_size;
    }
    if let Some(claim_wait_time) = claim_wait_time {
        config.claim_wait_time = claim_wait_time;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg(test)]
mod tests {}
