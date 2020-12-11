use super::*;

use crate::constants::{
    INIT_POOL_SUPPLY
};

#[test]
fn pool_initial_state_test() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());

    let pool_id = contract.new_pool(swap_fee());
    assert_eq!(contract.pool_is_finalized(pool_id), false);
    assert_eq!(u64::from(contract.get_pool_num_tokens(pool_id)), 0);
    assert_eq!(contract.get_pool_current_tokens(pool_id).len(), 0);
    assert_eq!(contract.get_pool_current_tokens(pool_id).len(), 0);
}

#[test]
fn bind_and_finalize_valid_pool() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());
    
    let pool_id = contract.new_pool(swap_fee());

    contract.bind_pool(
        pool_id,
        &token_a(),
        U128(to_token_denom(5)),
        U128(to_token_denom(1000))
    );

    contract.bind_pool(
        pool_id,
        &token_b(),
        U128(to_token_denom(5)),
        U128(to_token_denom(1000))
    );

    contract.finalize_pool(pool_id);

    let tokens = contract.get_pool_final_tokens(pool_id);
    assert_eq!(tokens.len(), 2);
}

#[test]
fn bind_rebind_finalize_valid_pool() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());
    
    let pool_id = contract.new_pool(swap_fee());

    // Bind token_a
    contract.bind_pool(
        pool_id,
        &token_a(),
        U128(to_token_denom(5)),
        U128(to_token_denom(1000))
    );

    // Bind token_b
    contract.bind_pool(
        pool_id,
        &token_b(),
        U128(to_token_denom(5)),
        U128(to_token_denom(1000))
    );

    // Rebind token_a
    contract.rebind_pool(
        pool_id,
        &token_b(),
        U128(to_token_denom(10)),
        U128(to_token_denom(300))
    );

    contract.finalize_pool(pool_id);

    let tokens = contract.get_pool_final_tokens(pool_id);
    assert_eq!(tokens.len(), 2);

    let balance_b = contract.get_pool_balance(pool_id, &token_b());

    assert_eq!(balance_b, U128(to_token_denom(300)));

    let owner_pool_tokens: u128 = contract.get_pool_token_balance(pool_id, &alice()).into();
    assert_eq!(owner_pool_tokens, INIT_POOL_SUPPLY);
}

#[test]
#[should_panic(expected = "ERR_NO_POOL")]
fn get_non_existing_pool_info_test() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let contract = PoolFactory::init(alice());
    assert_eq!(contract.pool_is_finalized(U64(1)), false);
}

#[test]
#[should_panic(expected = "ERR_NOT_FINALIZED")]
fn pool_final_tokens_fail_test() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());

    let pool_id = contract.new_pool(swap_fee());
    contract.get_pool_final_tokens(pool_id);
}

// Bind
// TODO: Test invalid token account id
// TODO: Test rebinding through bind
// TODO: Test finalized Pool binding
// TODO: Test binding after num_tokens >= MAX_BOUND_TOKENS

// Rebind
// TODO: Test rebind existing binding
// TODO: Test invalid token account id
// TODO: Test rebinding unbound token
// TODO: Test rebinding finalized pool
// TODO: Test denorm and balance bound assertions (min weight, max weight, min balance)

// Unbind
// TODO: Test Unbind
// TODO: Test Unbinding unbound token
// TODO: Test unbinding for finalized pool

// Finalize edge cases
// TODO: finalize pool from not controller
// TODO: finalize a finalized pool
// TODO: finalize a pool with < 2 tokens

