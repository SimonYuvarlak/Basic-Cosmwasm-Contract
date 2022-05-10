use std::ops::Add;
use std::ptr::null;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64, Addr, StdError, Order};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{OwnerResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ScoreResponse};
use crate::state::{State, STATE, SCORE_SEQ, Score, SCORE_LIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:{{project-name}}";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //Create the initial state and set the owner
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    SCORE_SEQ.save(deps.storage, &Uint64::zero());
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
        ExecuteMsg::AddScore {address , score} => add_score(deps, info, address, score),
    }
}

pub fn add_score(deps: DepsMut, info: MessageInfo, address: Addr, score: Uint64) -> Result<Response, ContractError> {
    //Get the owner of the contract
    let owner_state = STATE.load(deps.storage)?;
    //Check if the funciton caller is the owner
    if owner_state.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    let owner_addr = owner_state.owner.clone().to_string();
    //Validate the owner address
    let _owner = deps.api.addr_validate(&owner_addr)?;

    let id = SCORE_SEQ.update::<_, StdError>(deps.storage, |id| Ok(id.add(Uint64::new(1))))?;

    let new_score = Score {
        address: address.clone(),
        score: score.clone(),
    };

    SCORE_LIST.save(deps.storage, id.u64(), &new_score)?;

    Ok(Response::new()
        .add_attribute("address", address)
        .add_attribute("score", score)
        .add_attribute("id", id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetScore {address} => to_binary(&query_score(deps, address)?)
    }
}

fn query_score(deps: Deps, address: Addr) -> StdResult<ScoreResponse> {
    let scores: StdResult<Vec<_>> = SCORE_LIST
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let all_scores: Vec<Score> = scores?.into_iter().map(|list| list.1).collect();
    let mut resulting_score = Score {
        address: address.clone(),
        score: Uint64::new(0),
    };
    for score in all_scores {
        if score.address == address {
            resulting_score = score;
        }
    }

    Ok(ScoreResponse { address: resulting_score.address, score: resulting_score.score })
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    // Ok(Response::new()
    //     .add_attribute("method", "query owner")
    //     .add_attribute("owner", state.owner))
    Ok(OwnerResponse { owner: state.owner })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn total_flow() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let info = mock_info("creator", &coins(1000, "earth"));
        let msg = InstantiateMsg { owner: info.sender.clone() };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(info.sender, res.attributes[1].value);

        //it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(info.sender, value.owner);

        //now lets add a score to an address
        let msg = ExecuteMsg::AddScore {
            address: info.clone().sender,
            score: Uint64::new(10),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!("10", res.attributes[1].value);

        //let's query that address
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetScore {address: info.sender.clone()}).unwrap();
        let value: ScoreResponse = from_binary(&res).unwrap();
        assert_eq!(Uint64::new(10), value.score);
    }
}
