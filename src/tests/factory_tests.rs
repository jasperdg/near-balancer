use super::*;

#[test]
fn test_ownership_transfering() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());

    let owner = contract.get_owner();
    assert_eq!(owner, &alice());

    contract.set_owner(bob());

    let new_owner = contract.get_owner();
    assert_eq!(new_owner, &bob());
}

#[test]
#[should_panic(expected = "ERR_NOT_OWNER")]
fn test_forbidden_ownership_transfering() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(bob());

    let owner = contract.get_owner();
    assert_eq!(owner, &bob());

    contract.set_owner(bob());
}

#[test]
fn test_id_increment() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());

    let init_nonce = u64::from(contract.get_nonce());
    contract.new_pool(swap_fee());
    let new_nonce = u64::from(contract.get_nonce());

    assert_eq!(init_nonce + 1, new_nonce);
}

#[test]
fn test_pool_creation() {
    let context = get_context(alice(), 0);
    testing_env!(context);
    let mut contract = PoolFactory::init(alice());

    let pool_id = contract.new_pool(swap_fee());
    assert_eq!(u64::from(pool_id), 1);

    assert!(contract.pool_exists(pool_id));
}