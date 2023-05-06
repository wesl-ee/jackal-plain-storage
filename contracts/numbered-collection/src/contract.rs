#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use jackal_pub_storage::numbered_collection::response::ItemResponse;

use crate::error::ContractError;
use crate::state::{COLLECTION, CONFIG};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::response::ConfigResponse;
use jackal_bindings::JackalMsg;

use semver::Version;

const CONTRACT_NAME: &str = "jackal-pub-storage:numbered-collection";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<JackalMsg>, ContractError> {
    CONFIG.save(deps.storage, &msg.config)?;

    let res =
        Response::new().add_message(CosmosMsg::Custom(JackalMsg::BuyStorage {
            bytes: msg.bytes,
            duration: msg.duration,
            payment_denom: "ujkl".to_string(),
            // The sender's wallet pays for this storage
            creator: info.sender.to_string(),
            // Storage is owned by the contract so as to not mess with a
            // wallet's existing owned storage. User owns this storage by proxy
            // by being the owner of this contract
            for_address: env.contract.address.to_string(),
        }));

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<JackalMsg>, ContractError> {
    match msg {
        ExecuteMsg::RenewCollection {} => {
            Ok(Response::new()) // TODO
        }
        ExecuteMsg::Store { cid_map } => execute_store(deps, info, cid_map),
    }
}

pub fn execute_store(
    deps: DepsMut,
    info: MessageInfo,
    cid_map: Vec<(u32, String)>,
) -> Result<Response<JackalMsg>, ContractError> {
    let mut resp = Response::new();
    for e in &cid_map {
        // cid_map is a map of token ID → CID
        let token_id = e.0;
        let cid = e.1.to_string();

        COLLECTION.save(deps.storage, token_id, &cid)?;
        resp = resp.add_message(CosmosMsg::Custom(JackalMsg::SignContract {
            creator: info.sender.to_string(),
            cid,
            pay_once: false,
        }))
    }

    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Item { token_id } => to_binary(&query_item(deps, token_id)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    CONFIG.load(deps.storage)
}

fn query_item(deps: Deps, token_id: Vec<u32>) -> StdResult<ItemResponse> {
    token_id
        .into_iter()
        .map(|token_id| {
            let cid = COLLECTION.load(deps.storage, token_id)?;
            Ok(crate::response::QueryItem {
                token_id,
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
