use cw_orch::{interface, prelude::*};

use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

pub const CONTRACT_ID: &str = "burner_admin";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty, id = CONTRACT_ID)]
pub struct BurnerAdminContract;

impl<Chain> Uploadable for BurnerAdminContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw_burner_admin")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
    }
}
