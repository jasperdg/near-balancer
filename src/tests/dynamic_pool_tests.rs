use super::*;
use crate::constants::{
    INIT_POOL_SUPPLY
};

fn create_even_pool() -> (PoolFactory, U64) {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());
    
    let pool_id = contract.new_pool(swap_fee());

    // Token a is stable coin worth $1
    contract.bind_pool(
        pool_id,
        &token_a(),
        U128(to_token_denom(20)),
        U128(to_token_denom(100))
    );

    // Token b is governance token worth $200
    contract.bind_pool(
        pool_id,
        &token_b(),
        U128(to_token_denom(10)),
        U128(to_token_denom(1))
    );
    contract.finalize_pool(pool_id);

    (contract, pool_id)
}

#[test]
fn test_pool_join() {
    let  (mut contract, pool_id) = create_even_pool();

    let price_a: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_b(), &token_a()).into();
    let price_b: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_b()).into();

    let expected_price_a = to_token_denom(2) / 100;
    let expected_price_b = to_token_denom(50);

    assert_eq!(expected_price_a, price_a);
    assert_eq!(expected_price_b, price_b);

    testing_env!(get_context(bob(), 0));

    let pool_amount_out = to_token_denom(100);
    let max_amounts_in = vec![U128(to_token_denom(100)), U128(to_token_denom(1))];
    contract.join_pool(pool_id, U128(pool_amount_out), max_amounts_in);

    let expected_total_supply = to_token_denom(200);
    let total_supply: u128 = contract.get_pool_token_total_supply(pool_id).into();
    assert_eq!(total_supply, expected_total_supply);

    /* Test Pool Token balances */
    let owner_pool_tokens: u128 = contract.get_pool_token_balance(pool_id, &alice()).into();
    let joined_pool_tokens: u128 = contract.get_pool_token_balance(pool_id, &bob()).into();
    
    assert_eq!(owner_pool_tokens, INIT_POOL_SUPPLY);
    assert_eq!(joined_pool_tokens, INIT_POOL_SUPPLY);

    /* Test pooled tokens balances */
    let pool_dai_balance: u128 = contract.get_pool_balance(pool_id, &token_a()).into();
    let pool_mkr_balance: u128 = contract.get_pool_balance(pool_id, &token_b()).into();

    let expected_pool_dai_balance = to_token_denom(200);
    let expected_pool_mkr_balance = to_token_denom(2);

    assert_eq!(pool_dai_balance, expected_pool_dai_balance);
    assert_eq!(pool_mkr_balance, expected_pool_mkr_balance);
}

#[test]
fn test_pool_exit() {
    let  (mut contract, pool_id) = create_even_pool();

    let price_a: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_b(), &token_a()).into();
    let price_b: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_b()).into();

    let expected_price_a = to_token_denom(2) / 100;
    let expected_price_b = to_token_denom(50);

    assert_eq!(expected_price_a, price_a);
    assert_eq!(expected_price_b, price_b);

    testing_env!(get_context(bob(), 0));

    let pool_amount_out = to_token_denom(100);
    let max_amounts_in = vec![U128(to_token_denom(100)), U128(to_token_denom(1))];
    contract.join_pool(pool_id, U128(pool_amount_out), max_amounts_in);

    let owner_pool_tokens: u128 = contract.get_pool_token_balance(pool_id, &alice()).into();
    let joined_pool_tokens: u128 = contract.get_pool_token_balance(pool_id, &bob()).into();
    
    assert_eq!(owner_pool_tokens, INIT_POOL_SUPPLY);
    assert_eq!(joined_pool_tokens, INIT_POOL_SUPPLY);

    let pool_tokens_in = U128(to_token_denom(100));
    let min_amounts_out = vec![U128(to_token_denom(100)), U128(to_token_denom(1))];

    /* Test exit */
    contract.exit_pool(pool_id, pool_tokens_in, min_amounts_out);

    let owner_pool_tokens_after_exit: u128 = contract.get_pool_token_balance(pool_id, &alice()).into();
    // TODO: check if pool tokens are burned correctly
    // let joined_pool_tokens_after_exit: u128 = contract.get_pool_token_balance(pool_id, &bob()).into();
    
    assert_eq!(owner_pool_tokens_after_exit, INIT_POOL_SUPPLY);
    // assert_eq!(joined_pool_tokens_after_exit, 0); // First need to implement burn / internal_transfers

    /* Test pooled tokens balances */
    let pool_dai_balance: u128 = contract.get_pool_balance(pool_id, &token_a()).into();
    let pool_mkr_balance: u128 = contract.get_pool_balance(pool_id, &token_b()).into();

    let expected_pool_dai_balance = to_token_denom(100);
    let expected_pool_mkr_balance = to_token_denom(1);

    assert_eq!(pool_dai_balance, expected_pool_dai_balance);
    assert_eq!(pool_mkr_balance, expected_pool_mkr_balance);
}