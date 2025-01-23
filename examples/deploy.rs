use cw_burner_admin::{msg::InstantiateMsg, BurnerAdminContract};
use cw_orch::{
    anyhow::{self, Ok},
    daemon::{networks, TxSender},
    prelude::*,
};

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let network = networks::OSMO_5;
    let chain = DaemonBuilder::new(network).build()?;

    let contract = BurnerAdminContract::new(chain.clone());
    let sender = chain.sender().address();

    contract.upload_if_needed()?;

    if contract.address().is_err() {
        contract.instantiate(&InstantiateMsg {}, Some(&sender), &[])?;
    }

    Ok(())
}
