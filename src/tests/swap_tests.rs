use super::*;

fn create_even_pool() -> (PoolFactory, U64) {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());
    
    let pool_id = contract.new_pool(swap_fee());

    // Token a is stable coin worth $1
    contract.bind_pool(
        pool_id,
        &token_a(),
        U128(to_token_denom(10)),
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
fn test_swap_exact_amt_in_even_pool() {
    let  (mut contract, pool_id) = create_even_pool();
    let tokens_in = to_token_denom(25);

    let (tokens_out, spot_price_after) = contract.swap_exact_amount_in(
        pool_id,
        &token_a(),
        U128(tokens_in),
        &token_b(),
        U128(0),
        U128(to_token_denom(1000))
    );

    // TODO: Add math to calculate expected result (verified in js for now)
    let expected_tokens_out = 199519711827096258; 
    let expected_spot_price_after = 156626128385155466808;

    assert_eq!(U128(expected_tokens_out), tokens_out);
    assert_eq!(expected_spot_price_after, u128::from(spot_price_after));
}

fn create_uneven_pool() -> (PoolFactory, U64) {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());
    
    let pool_id = contract.new_pool(U128(0));

    // Token a is stable coin worth $1
    contract.bind_pool(
        pool_id,
        &token_a(),
        U128(to_token_denom(20)),
        U128(to_token_denom(200))
    );

    // Token b is a token worth $0.5
    contract.bind_pool(
        pool_id,
        &token_b(),
        U128(to_token_denom(10)),
        U128(to_token_denom(50))
    );
    
    // Token c is a token worth 0.5
    contract.bind_pool(
        pool_id,
        &token_c(),
        U128(to_token_denom(10)),
        U128(to_token_denom(50))
    );
    contract.finalize_pool(pool_id);

    (contract, pool_id)
}

#[test]
fn test_swap_exact_amt_in_uneven_pool() {
    let  (mut contract, pool_id) = create_uneven_pool();
    let tokens_in = to_token_denom(10);

    /*** verify base state pricing ***/
    let expected_a_to_b_spot_price = to_token_denom(5) / 10;
    let expected_a_to_c_spot_price = to_token_denom(5) / 10;

    let a_to_b_spot_price: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_b()).into();
    let a_to_c_spot_price: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_c()).into();

    assert_eq!(expected_a_to_b_spot_price, a_to_b_spot_price);
    assert_eq!(expected_a_to_c_spot_price, a_to_c_spot_price);


    let (tokens_out, spot_price_after) = contract.swap_exact_amount_in(
        pool_id,
        &token_a(),
        U128(tokens_in),
        &token_b(),
        U128(0),
        U128(to_token_denom(1000))
    );


    /*** verify post swap pricing ***/
    let expected_a_to_b_spot_price = 805255000000000000;
    let expected_a_to_c_spot_price = to_token_denom(55) / 100;

    let a_to_b_spot_price: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_b()).into();
    let a_to_c_spot_price: u128 = contract.get_pool_spot_price_sans_fee(pool_id, &token_a(), &token_c()).into();

    assert_eq!(expected_a_to_b_spot_price, a_to_b_spot_price);
    assert_eq!(expected_a_to_c_spot_price, a_to_c_spot_price);


}