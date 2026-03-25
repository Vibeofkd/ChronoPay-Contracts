#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Address, Env, String};
use soroban_sdk::testutils::Address as _;

fn setup() -> Env {
    let env = Env::default();
    env.mock_all_auths(); // Required for address.require_auth()
    env
}

#[test]
fn test_hello() {
    let env = setup();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let words = client.hello(&String::from_str(&env, "Dev"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "ChronoPay"),
            String::from_str(&env, "Dev"),
        ]
    );
}

#[test]
fn test_create_and_query_slot_with_increment() {
    let env = setup();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);

    let professional = Address::generate(&env);
    
    // Test auto-increment (Merged from main logic)
    let slot_id_1 = client.create_time_slot(&professional, &1000u64, &2000u64);
    let slot_id_2 = client.create_time_slot(&professional, &3000u64, &4000u64);
    
    assert_eq!(slot_id_1, 1);
    assert_eq!(slot_id_2, 2);

    // Query existing slot (From feature/sc-038 logic)
    let slot = client.get_time_slot(&slot_id_1).expect("slot should exist");
    assert_eq!(slot.professional, professional);
    assert_eq!(slot.start_time, 1000u64);
    assert_eq!(slot.end_time, 2000u64);
    assert!(slot.token.is_none());

    // Query non-existent slot
    let non_existent = client.get_time_slot(&999u32);
    assert!(non_existent.is_none());
}

#[test]
#[should_panic(expected = "end_time must be after start_time")]
fn test_create_time_slot_rejects_invalid_times() {
    let env = setup();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);
    
    let professional = Address::generate(&env);
    // Should panic because start == end
    let _ = client.create_time_slot(&professional, &10u64, &10u64);
}

#[test]
fn test_token_stubs() {
    let env = setup();
    let contract_id = env.register(ChronoPayContract, ());
    let client = ChronoPayContractClient::new(&env, &contract_id);
    
    let professional = Address::generate(&env);
    let slot_id = client.create_time_slot(&professional, &1000u64, &2000u64);
    
    // Testing the stubs from the main branch
    let token = client.mint_time_token(&slot_id);
    assert_eq!(token, soroban_sdk::Symbol::new(&env, "TIME_TOKEN"));

    let redeemed = client.redeem_time_token(&token);
    assert!(redeemed);
}