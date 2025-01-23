use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    /// Burns all the "owned token" in the contract's balance.
    #[cw_orch(payable)]
    BurnBalance { denom: String },
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(Coin)]
    AmountBurned { denom: String },
}
