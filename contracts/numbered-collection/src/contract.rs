#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use jackal_pub_storage::numbered_collection::response::ItemResponse;

use crate::error::ContractError;
use crate::state::{CONFIG, COLLECTION};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use jackal_bindings::JackalMsg;
use crate::response::ConfigResponse;

use semver::Version;

const CONTRACT_NAME: &str = "levana.finance:market";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<JackalMsg>, ContractError> {
    CONFIG.save(deps.storage, &msg.config)?;

    let res = Response::new()
       .add_message(JackalMsg::BuyStorage {
           bytes: msg.bytes,
           duration: msg.duration,
           for_address: info.sender.to_string(),
           payment_denom: "ujkl".to_string(),
           // Storage is owned by the contract so as to not mess with a
           // wallet's existing owned storage. User owns this storage by proxy
           // by being the owner of this contract
           creator: env.contract.address.to_string(),
       });

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RenewCollection {  } => {
            Ok(Response::new()) // TODO
        },
        ExecuteMsg::Store { cid_map, } => {
            for e in &cid_map {
                COLLECTION.save(deps.storage, e.0, &e.1)?;
            }

            Ok(Response::new())
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Item {index} => to_binary(&query_item(deps, index)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    CONFIG.load(deps.storage)
}

fn query_item(deps: Deps, index: Vec<u32>) -> StdResult<ItemResponse> {
    index
        .into_iter()
        .map(|index| {
            let cid = COLLECTION.load(deps.storage, index)?;
            Ok(crate::response::QueryItem {
                index,
                cid,
                provider: vec![], // TODO
            })
        })
        .collect()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<Response> {
    let old_cw2 = get_contract_version(deps.storage)?;
    let old_version: Version = old_cw2.version.parse().map_err(|_| {
        StdError::generic_err("couldn't parse old contract version")
    })?;
    let new_version: Version = CONTRACT_VERSION.parse().map_err(|_| {
        StdError::generic_err("couldn't parse new contract version")
    })?;

    if old_cw2.contract != CONTRACT_NAME {
        Err(StdError::generic_err(format!(
            "mismatched contract migration name (from {} to {})",
            old_cw2.contract, CONTRACT_NAME
        ))
        .into())
    } else if old_version >= new_version {
        Err(StdError::generic_err(format!(
            "cannot migrate contract from newer to older (from {} to {})",
            old_cw2.version, CONTRACT_VERSION
        ))
        .into())
    } else {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::new())
    }
}

#[cfg(test)]
mod test {}
