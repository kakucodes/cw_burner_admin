use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::AMOUNT_BURNED;
use cosmwasm_std::{coin, entry_point, to_json_binary, Env};
use cosmwasm_std::{Binary, Deps, DepsMut, MessageInfo, Response};
use cw2::set_contract_version;
use cw_utils::{may_pay, nonpayable};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgBurn;

const CONTRACT_NAME: &str = "crates.io:cw-burner-admin";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BurnBalance { denom } => {
            may_pay(&info, &denom)?;

            let contract_balance = deps
                .querier
                .query_balance(env.contract.address.clone(), denom.clone())?;

            let mut resp = Response::default();

            // if the contract has a balance, burn it
            if !contract_balance.amount.is_zero() {
                resp = resp.clone().add_message(MsgBurn {
                    sender: env.contract.address.to_string(),
                    burn_from_address: env.contract.address.to_string(),
                    amount: Some(contract_balance.clone().into()),
                });

                // update the amount burned tally
                AMOUNT_BURNED.update(deps.storage, denom.clone(), |old| {
                    Ok::<u128, ContractError>(old.unwrap_or_default() + contract_balance.amount.u128())
                })?;
            };

            Ok(resp.add_attributes(vec![
                ("action", "burn_balance"),
                ("burn", &contract_balance.to_string()),
            ]))
        },
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let result = match msg {
        QueryMsg::AmountBurned { denom } => {
            let burned = AMOUNT_BURNED.may_load(deps.storage, denom.clone())?.unwrap_or_default();
            to_json_binary(&coin(burned, denom))
        },
    }?;

    Ok(result)
}
