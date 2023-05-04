#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError,
};
use cw2::{get_contract_version, set_contract_version};

use crate::error::ContractError;
use crate::state::CONFIG;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use crate::response::ConfigResponse;

use semver::Version;

const CONTRACT_NAME: &str = "levana.finance:market";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(deps.storage, &msg.config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {}
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::BlockInfo {} => to_binary(&env.block),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    CONFIG.load(deps.storage)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let old_cw2 = get_contract_version(deps.storage)?;
    let old_version: Version = old_cw2
        .version
        .parse()
        .map_err(|_| StdError::generic_err("couldn't parse old contract version"))?;
    let new_version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|_| StdError::generic_err("couldn't parse new contract version"))?;

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
