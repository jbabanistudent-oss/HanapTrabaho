#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env, Symbol};

// ALISIN ANG <'static>: Sa SDK v25, 'token::Client' na lang ito nang direkta
fn setup_test_token(env: &Env, admin: &Address) -> token::Client {
    let token_address = env.register_stellar_asset_contract(admin.clone());
    token::Client::new(env, &token_address)
}

#[test]
fn test_1_happy_path_escrow_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TrabahoVaultContract);
    let client = TrabahoVaultContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let employer = Address::generate(&env);
    let freelancer = Address::generate(&env);
    
    let usdc = setup_test_token(&env, &token_admin);
    
    // UPDATE: Ginamit ang 'StellarAssetContractClient' na siyang bago sa v25
    token::StellarAssetContractClient::new(&env, &usdc.address).mint(&employer, &500);

    let doc_hash = Symbol::new(&env, "task_deliverable_v1");

    client.initialize(&employer, &freelancer, &usdc.address, &150, &doc_hash);
    client.fund_vault();
    assert_eq!(usdc.balance(&contract_id), 150);

    client.release_payment();
    assert_eq!(usdc.balance(&freelancer), 150);
    assert_eq!(usdc.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "Vault escrow agreement already initialized")]
fn test_2_edge_case_duplicate_initialization_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrabahoVaultContract);
    let client = TrabahoVaultContractClient::new(&env, &contract_id);

    let employer = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let token_addr = Address::generate(&env);
    let doc_hash = Symbol::new(&env, "hash");

    client.initialize(&employer, &freelancer, &token_addr, &100, &doc_hash);
    client.initialize(&employer, &freelancer, &token_addr, &100, &doc_hash);
}

#[test]
fn test_3_state_verification_reflects_funding() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TrabahoVaultContract);
    let client = TrabahoVaultContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let employer = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let usdc = setup_test_token(&env, &token_admin);
    
    // UPDATE: Ginamit ang 'StellarAssetContractClient' para sa v25 compliance
    token::StellarAssetContractClient::new(&env, &usdc.address).mint(&employer, &200);

    let doc_hash = Symbol::new(&env, "proof");

    client.initialize(&employer, &freelancer, &usdc.address, &200, &doc_hash);
    client.fund_vault();

    let state = client.get_vault_details();
    assert_eq!(state.is_funded, true);
    assert_eq!(state.is_released, false);
}

#[test]
#[should_panic(expected = "Cannot release funds from an unfunded vault")]
fn test_4_edge_case_release_unfunded_vault_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TrabahoVaultContract);
    let client = TrabahoVaultContractClient::new(&env, &contract_id);

    let employer = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let token_addr = Address::generate(&env);
    let doc_hash = Symbol::new(&env, "proof");

    client.initialize(&employer, &freelancer, &token_addr, &100, &doc_hash);
    client.release_payment();
}

#[test]
fn test_5_state_verification_on_disbursal() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, TrabahoVaultContract);
    let client = TrabahoVaultContractClient::new(&env, &contract_id);

    let token_admin = Address::generate(&env);
    let employer = Address::generate(&env);
    let freelancer = Address::generate(&env);
    let usdc = setup_test_token(&env, &token_admin);
    
    // UPDATE: Ginamit ang 'StellarAssetContractClient' para sa v25 compliance
    token::StellarAssetContractClient::new(&env, &usdc.address).mint(&employer, &100);

    let doc_hash = Symbol::new(&env, "final");

    client.initialize(&employer, &freelancer, &usdc.address, &100, &doc_hash);
    client.fund_vault();
    client.release_payment();

    let state = client.get_vault_details();
    assert_eq!(state.is_released, true);
}