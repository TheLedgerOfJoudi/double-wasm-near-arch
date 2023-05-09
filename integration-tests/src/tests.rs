use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::{Account, Contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount("alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    test_absent_token(&alice, &contract).await?;
    test_set_token(&alice, &contract).await?;
    Ok(())
}

async fn test_absent_token(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    let token: String = user
        .call(contract.id(), "get_token")
        .args_json(json!({"owner_id":"no_one"}))
        .transact()
        .await?
        .json()?;

    assert_eq!(token, "No token".to_string());
    println!("      Passed ✅ no token returned");
    Ok(())
}

async fn test_set_token(user: &Account, contract: &Contract) -> anyhow::Result<()> {
    user.call(contract.id(), "set_info")
        .args_json(json!({"token_id":"token1", "owner_id":"owner1"}))
        .transact()
        .await?
        .into_result()?;

    let token: String = user
        .call(contract.id(), "get_token")
        .args_json(json!({"owner_id":"owner1"}))
        .transact()
        .await?
        .json()?;

    assert_eq!(token, "token1".to_string());
    println!("      Passed ✅ sets token");
    Ok(())
}
