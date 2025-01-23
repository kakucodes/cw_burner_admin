use cosmwasm_std::{coin, coins};
use cw_burner_admin::{msg::InstantiateMsg, BurnerAdminContract, BurnerAdminExecuteMsgFns, BurnerAdminQueryMsgFns};

use cw_orch::prelude::*;
use cw_orch_osmosis_test_tube::OsmosisTestTube;
use osmosis_test_tube::osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::tokenfactory::v1beta1::{MsgChangeAdmin, MsgCreateDenom, MsgMint},
};
use osmosis_test_tube::Account;

pub const SUBDENOM: &str = "ukaku";

#[test]
pub fn single_coin_test() -> cw_orch::anyhow::Result<()> {
    let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    let burner_contract = BurnerAdminContract::new(chain.clone());

    burner_contract.upload()?;
    burner_contract.instantiate(&InstantiateMsg {}, None, &[])?;

    let sender = burner_contract.environment().sender.clone();
    let sender_addr = sender.address();

    // We create a new denom
    chain.commit_any(
        vec![MsgCreateDenom {
            sender: sender_addr.clone(),
            subdenom: SUBDENOM.to_string(),
        }
        .to_any()
        .into()],
        None,
    )?;
    let denom = format!("factory/{}/{}", sender_addr, SUBDENOM);

    // We mint some tokens
    chain.commit_any(
        vec![MsgMint {
            sender: sender_addr.clone(),
            amount: Some(Coin {
                amount: 100_000.to_string(),
                denom: denom.clone(),
            }),
            mint_to_address: sender_addr.clone(),
        }
        .to_any()
        .into()],
        None,
    )?;

    // We verify the supply is 100_000ukaku
    assert_eq!(chain.bank_querier().supply_of(&denom)?, coin(100_000, denom.clone()));

    // We send it to the contract
    chain.bank_send(
        burner_contract.address()?.to_string(),
        vec![coin(50_000, denom.clone())],
    )?;

    // We verify everything has worked correctly
    assert_eq!(
        chain
            .bank_querier()
            .balance(&burner_contract.address()?, Some(denom.clone()))?
            .first()
            .cloned(),
        Some(coin(50_000, denom.clone())),
        "The contract has the correct balance after being sent tokens"
    );

    // We give the burner contract admin rights over our denom
    chain.commit_any(
        vec![MsgChangeAdmin {
            sender: sender_addr.clone(),
            denom: denom.clone(),
            new_admin: burner_contract.address()?.to_string(),
        }
        .to_any()
        .into()],
        None,
    )?;

    burner_contract.burn_balance(denom.clone(), &coins(2_500, denom.clone()))?;

    // We verify the contract has no balance since it burned all the denom tokens
    assert_eq!(
        chain
            .bank_querier()
            .balance(&burner_contract.address()?, Some(denom.clone()))?
            .first()
            .cloned(),
        Some(coin(0, denom.clone()))
    );
    // We verify the supply has been decreased and the tokens were fully burned
    assert_eq!(chain.bank_querier().supply_of(&denom)?, coin(47_500, denom.clone()));

    burner_contract.burn_balance(denom.clone(), &coins(22_500, denom.clone()))?;

    // Verify that the total supply has been further decreased
    assert_eq!(chain.bank_querier().supply_of(&denom)?, coin(25_000, denom.clone()));

    // try to mint more tokens from the old admin
    let failed_mint = chain.commit_any(
        vec![MsgMint {
            sender: sender_addr.clone(),
            amount: Some(Coin {
                amount: 100_000.to_string(),
                denom: denom.clone(),
            }),
            mint_to_address: sender_addr.clone(),
        }
        .to_any()
        .into()],
        None,
    );
    assert!(
        failed_mint.is_err(),
        "Ensure that the old admin cannot mint more tokens",
    );
    assert_eq!(
        chain.bank_querier().supply_of(&denom)?,
        coin(25_000, denom.clone()),
        "Total supply should be unchanged"
    );

    let burned_denom = burner_contract.amount_burned(denom.clone())?;
    assert_eq!(burned_denom, coin(75_000, denom), "The amount burned should be correct");

    Ok(())
}

#[test]
pub fn multi_coin_test() -> cw_orch::anyhow::Result<()> {
    let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    let burner_contract = BurnerAdminContract::new(chain.clone());

    burner_contract.upload()?;
    burner_contract.instantiate(&InstantiateMsg {}, None, &[])?;

    let sender = burner_contract.environment().sender.clone();
    let sender_addr = sender.address();

    // Create first denom
    let subdenom1 = "ukaku";
    chain.commit_any(
        vec![MsgCreateDenom {
            sender: sender_addr.clone(),
            subdenom: subdenom1.to_string(),
        }
        .to_any()
        .into()],
        None,
    )?;
    let denom1 = format!("factory/{}/{}", sender_addr, subdenom1);

    // Create second denom
    let subdenom2 = "umochi";
    chain.commit_any(
        vec![MsgCreateDenom {
            sender: sender_addr.clone(),
            subdenom: subdenom2.to_string(),
        }
        .to_any()
        .into()],
        None,
    )?;
    let denom2 = format!("factory/{}/{}", sender_addr, subdenom2);

    // Mint tokens for both denoms
    chain.commit_any(
        vec![
            MsgMint {
                sender: sender_addr.clone(),
                amount: Some(Coin {
                    amount: 100_000.to_string(),
                    denom: denom1.clone(),
                }),
                mint_to_address: sender_addr.clone(),
            }
            .to_any()
            .into(),
            MsgMint {
                sender: sender_addr.clone(),
                amount: Some(Coin {
                    amount: 200_000.to_string(),
                    denom: denom2.clone(),
                }),
                mint_to_address: sender_addr.clone(),
            }
            .to_any()
            .into(),
        ],
        None,
    )?;

    // Verify initial supplies
    assert_eq!(chain.bank_querier().supply_of(&denom1)?, coin(100_000, denom1.clone()));
    assert_eq!(chain.bank_querier().supply_of(&denom2)?, coin(200_000, denom2.clone()));

    // Send tokens to contract
    chain.bank_send(
        burner_contract.address()?.to_string(),
        vec![coin(50_000, denom1.clone()), coin(100_000, denom2.clone())],
    )?;

    // Transfer admin rights for both denoms
    chain.commit_any(
        vec![
            MsgChangeAdmin {
                sender: sender_addr.clone(),
                denom: denom1.clone(),
                new_admin: burner_contract.address()?.to_string(),
            }
            .to_any()
            .into(),
            MsgChangeAdmin {
                sender: sender_addr.clone(),
                denom: denom2.clone(),
                new_admin: burner_contract.address()?.to_string(),
            }
            .to_any()
            .into(),
        ],
        None,
    )?;

    // Burn partial amounts of both denoms
    burner_contract.burn_balance(denom1.clone(), &coins(25_000, denom1.clone()))?;
    burner_contract.burn_balance(denom2.clone(), &coins(50_000, denom2.clone()))?;

    // Verify burned amounts
    let burned_denom1 = burner_contract.amount_burned(denom1.clone())?;
    assert_eq!(
        burned_denom1,
        coin(75_000, denom1.clone()),
        "The amount burned for denom1 should be correct"
    );

    let burned_denom2 = burner_contract.amount_burned(denom2.clone())?;
    assert_eq!(
        burned_denom2,
        coin(150_000, denom2.clone()),
        "The amount burned for denom2 should be correct"
    );

    // Verify reduced supplies
    assert_eq!(chain.bank_querier().supply_of(&denom1)?, coin(25_000, denom1.clone()));
    assert_eq!(chain.bank_querier().supply_of(&denom2)?, coin(50_000, denom2.clone()));

    // Verify contract balances
    assert_eq!(
        chain
            .bank_querier()
            .balance(&burner_contract.address()?, Some(denom1.clone()))?
            .first()
            .cloned(),
        Some(coin(0, denom1.clone())),
        "Contract should have remaining balance of first denom"
    );
    assert_eq!(
        chain
            .bank_querier()
            .balance(&burner_contract.address()?, Some(denom2.clone()))?
            .first()
            .cloned(),
        Some(coin(0, denom2.clone())),
        "Contract should have remaining balance of second denom"
    );

    // Verify final burned amounts
    let final_burned_denom1 = burner_contract.amount_burned(denom1.clone())?;
    assert_eq!(
        final_burned_denom1,
        coin(75_000, denom1.clone()),
        "The final amount burned for denom1 should be correct"
    );

    let final_burned_denom2 = burner_contract.amount_burned(denom2.clone())?;
    assert_eq!(
        final_burned_denom2,
        coin(150_000, denom2.clone()),
        "The final amount burned for denom2 should be correct"
    );

    Ok(())
}

#[test]
pub fn authorization_test() -> cw_orch::anyhow::Result<()> {
    pretty_env_logger::init();
    let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    let burner_contract = BurnerAdminContract::new(chain.clone());

    // Upload and instantiate the contract
    burner_contract.upload()?;
    burner_contract.instantiate(&InstantiateMsg {}, None, &[])?;

    let sender = burner_contract.environment().sender.clone();
    let sender_addr = sender.address();

    // Create a secondary account for additional testing
    let other_account = chain.clone().init_account(coins(1_000_000_000, "uosmo"))?;
    let other_addr = other_account.address();

    // Create a new denom
    let subdenom = "ubtc";
    chain.commit_any(
        vec![MsgCreateDenom {
            sender: sender_addr.clone(),
            subdenom: subdenom.to_string(),
        }
        .to_any()
        .into()],
        None,
    )?;
    let denom = format!("factory/{}/{}", sender_addr, subdenom);

    // Initial minting by token creator should succeed
    chain.commit_any(
        vec![MsgMint {
            sender: sender_addr.clone(),
            amount: Some(Coin {
                amount: 100_000.to_string(),
                denom: denom.clone(),
            }),
            mint_to_address: sender_addr.clone(),
        }
        .to_any()
        .into()],
        None,
    )?;

    // Verify initial supply after creator mint
    assert_eq!(chain.bank_querier().supply_of(&denom)?, coin(100_000, denom.clone()));

    // Send some tokens to both the contract and the other account
    chain.bank_send(
        burner_contract.address()?.to_string(),
        vec![coin(30_000, denom.clone())],
    )?;
    chain.bank_send(other_addr.clone(), vec![coin(30_000, denom.clone())])?;

    // Attempt to burn from burner contract before admin rights transfer should fail
    let burn_result = burner_contract.burn_balance(denom.clone(), &coins(10_000, denom.clone()));
    assert!(burn_result.is_err(), "Burn should fail before admin rights transfer");

    // Attempt unauthorized mint should fail
    let unauthorized_mint = chain.commit_any(
        vec![MsgMint {
            sender: other_addr.clone(),
            amount: Some(Coin {
                amount: 50_000.to_string(),
                denom: denom.clone(),
            }),
            mint_to_address: other_addr.clone(),
        }
        .to_any()
        .into()],
        None,
    );
    assert!(
        unauthorized_mint.is_err(),
        "Unauthorized account should not be able to mint"
    );

    // Transfer admin rights to the burner contract
    chain.commit_any(
        vec![MsgChangeAdmin {
            sender: sender_addr.clone(),
            denom: denom.clone(),
            new_admin: burner_contract.address()?.to_string(),
        }
        .to_any()
        .into()],
        None,
    )?;

    // Original creator attempt to mint after admin transfer should fail
    let creator_mint_after_transfer = chain.commit_any(
        vec![MsgMint {
            sender: sender_addr.clone(),
            amount: Some(Coin {
                amount: 50_000.to_string(),
                denom: denom.clone(),
            }),
            mint_to_address: sender_addr.clone(),
        }
        .to_any()
        .into()],
        None,
    );
    assert!(
        creator_mint_after_transfer.is_err(),
        "Original creator should not be able to mint after admin transfer"
    );

    // Burner contract should now allow burns from any account
    // First, let's have the other account burn through the contract
    burner_contract
        .call_as(&other_account)
        .burn_balance(denom.clone(), &coins(10_000, denom.clone()))?;

    // Verify supply after burn from other account
    assert_eq!(
        chain.bank_querier().supply_of(&denom)?,
        coin(60_000, denom.clone()),
        "Supply should be reduced after burn from other account"
    );

    // Original creator can also burn through the contract
    burner_contract
        .call_as(&sender)
        .burn_balance(denom.clone(), &coins(10_000, denom.clone()))?;

    // Verify supply after burn from creator
    assert_eq!(
        chain.bank_querier().supply_of(&denom)?,
        coin(50_000, denom.clone()),
        "Supply should be reduced after burn from creator"
    );

    // Attempt to migrate the contract should fail
    let migrate_result = burner_contract.call_as(&sender).migrate(&cosmwasm_std::Empty {}, 1u64);
    assert!(migrate_result.is_err(), "Contract migration should not be possible");

    // Final verification - attempt direct mint should still fail for non-admin
    let final_mint_attempt = chain.commit_any(
        vec![MsgMint {
            sender: other_addr.clone(),
            amount: Some(Coin {
                amount: 25_000.to_string(),
                denom: denom.clone(),
            }),
            mint_to_address: other_addr.clone(),
        }
        .to_any()
        .into()],
        None,
    );
    assert!(
        final_mint_attempt.is_err(),
        "Direct minting should still not be possible for non-admin accounts"
    );

    // Verify final supply unchanged by failed operations
    assert_eq!(
        chain.bank_querier().supply_of(&denom)?,
        coin(50_000, denom.clone()),
        "Supply should be unchanged after failed operations"
    );

    Ok(())
}
