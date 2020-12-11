use super::*;
use crate::math;

#[test]
fn test_finalized_even_pool_pricing_sans_fee() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());
    
    let pool_id = contract.new_pool(swap_fee());

    contract.bind_pool(
        pool_id,
        &token_a(),
        U128(to_token_denom(10)),
        U128(to_token_denom(1000))
    );

    contract.bind_pool(
        pool_id,
        &token_b(),
        U128(to_token_denom(10)),
        U128(to_token_denom(1000))
    );

    let price_token_a = contract.get_pool_spot_price_sans_fee(pool_id, &token_b(), &token_a());
    let price_token_b = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_b());

    let expected_spot_price = U128(to_token_denom(1));
    assert_eq!(price_token_a, expected_spot_price);
    assert_eq!(price_token_b, expected_spot_price);
}

#[test]
fn test_finalized_uneven_pool_pricing_sans_fee() {
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

    let price_a: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_b(), &token_a()).into();
    let price_b: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_b()).into();

    let expected_price_a = to_token_denom(2) / 100;
    let expected_price_b = to_token_denom(50);

    assert_eq!(expected_price_a, price_a);
    assert_eq!(expected_price_b, price_b);
}

#[test]
fn test_finalized_uneven_pool_pricing() {
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

    let price_a: u128 = contract.get_pool_spot_price(pool_id, &token_b(), &token_a()).into();
    let price_b: u128 = contract.get_pool_spot_price(pool_id, &token_a(), &token_b()).into();

    let swap_fee_u128: u128 = swap_fee().into();
    let scale = math::div_u128(to_token_denom(1), to_token_denom(1) - swap_fee_u128);

    let expected_price_a = math::div_u128(to_token_denom(2) / 100, scale);
    let expected_price_b = math::div_u128(to_token_denom(50), scale);

    assert_eq!(expected_price_a, price_a);
    assert_eq!(expected_price_b, price_b);
}