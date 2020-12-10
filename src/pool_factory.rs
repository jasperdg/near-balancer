use near_sdk::{
    near_bindgen,
    json_types::{
        U128, 
        U64
    },
    AccountId, 
    env,
    collections::{
        UnorderedMap
    },
    borsh::{
        BorshDeserialize,
        BorshSerialize
    }
};

use crate::pool::Pool;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PoolFactory {
    owner: AccountId, // The owner of the contract
    nonce: u64, // Incrementing number that's used to define a pool's id
    pools: UnorderedMap<u64, Pool> // Maps pool ids to pool
}

/** 
 * @notice implement `Default` for `PoolFactory` - allowing for a default state to be set
 * @panics if the contract hasn't been initialized yet, there is no default state
 */
impl Default for PoolFactory {
    fn default() -> Self {
        panic!("ERR_CONTRACT_NOT_INITIATED")
    }
}

#[near_bindgen]
impl PoolFactory {

    /**
     * @notice Initialize the contract by setting the owner
     * @param owner The `account_id` that's going to have owner privileges
     */
    #[init]
    pub fn init(owner: AccountId) -> Self {
        assert!(!env::state_exists(), "ERR_CONTRACT_IS_INITIALIZED");
        assert!(env::is_valid_account_id(owner.as_bytes()), "ERR_INVALID_ACCOUNT_ID");
        
        Self {
            owner: owner,
            nonce: 0,
            pools: UnorderedMap::new(b"pools".to_vec())
        }
    }

    /**
     * @return the `account_id` of the current owner
     */
    pub fn get_owner(&self) -> &AccountId {
        &self.owner
    }

    /**
     * @return returns the `account_id` of the current owner
     */
    pub fn get_nonce(&self) -> U64 {
        self.nonce.into()
    }

    /*** POOL_GETTERS ***/
    
    pub fn pool_exists(&self, pool_id: U64) -> bool {
        match self.pools.get(&pool_id.into()) {
            Some(pool) => true,
            None => false
        }
    }

    pub fn pool_is_finalized(&self, pool_id: U64) -> bool { 
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.is_finalized()
    }

    pub fn pool_token_is_bound(&self, token_account_id: &AccountId, pool_id: U64) -> bool {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.records
            .get(token_account_id)
            .expect("ERR_NO_RECORD")
            .bound
    }

    pub fn get_pool_num_tokens(&self, pool_id: U64) -> U64 {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.tokens.len().into()
    }

    pub fn get_pool_current_tokens(&self, pool_id: U64) -> Vec<AccountId> {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.tokens.to_vec()
    }

    pub fn get_pool_final_tokens(&self, pool_id: U64) -> Vec<AccountId> {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        assert!(pool.is_finalized(), "ERR_NOT_FINALIZED");
        pool.tokens.to_vec()
    }

    pub fn get_pool_balance(
        &self, 
        pool_id: U64,
        token_account_id: &AccountId
    ) -> U128 {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.records
            .get(token_account_id)
            .expect("ERR_NO_RECORD")
            .balance
            .into()
    }

    pub fn pool_get_swap_fee(&self, pool_id: U64) -> U128 {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.get_swap_fee().into()
    }

    /**
     * @notice allows the previous owner to set a new owner
     * @param new_owner the `account_id` of the new owner
     * @panics if the signer of this tx is not the previous owner
     * @panics if `new_owner` is not a valid account id
     */
    pub fn set_owner(&mut self, new_owner: AccountId) {
        assert_eq!(env::predecessor_account_id(), self.owner, "ERR_NOT_OWNER");
        assert!(env::is_valid_account_id(new_owner.as_bytes()), "ERR_INVALID_ACCOUNT_ID");
        
        self.owner = new_owner;
    }

    /**
     * @notice creates new token pool
     * @param
     * @return the new pool's id 
     */ 
    pub fn new_pool(&mut self, swap_fee: U128) -> U64 {
        self.nonce += 1;
        let new_pool = Pool::new(env::predecessor_account_id(), self.nonce, u128::from(swap_fee));
        self.pools.insert(&self.nonce, &new_pool);
        self.nonce.into()
    }


    /*** POOL SETTERS ***/

    pub fn finalize_pool(&mut self, pool_id: U64) {
        let mut pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.finalize(&env::predecessor_account_id());
        self.pools.insert(&pool_id.into(), &pool);
    }

    pub fn bind_pool(
        &mut self, 
        pool_id: U64,
        token_account_id: &AccountId,
        denorm: U128,
        balance: U128
    ) {
        let mut pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.bind(
            &env::predecessor_account_id(),
            token_account_id,
            u128::from(denorm),
            u128::from(balance)
        );
        self.pools.insert(&pool_id.into(), &pool);
    }

    pub fn rebind_pool(
        &mut self, 
        pool_id: U64,
        token_account_id: &AccountId,
        denorm: U128,
        balance: U128
    ) {
        let mut pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.rebind(
            &env::predecessor_account_id(),
            token_account_id,
            u128::from(denorm),
            u128::from(balance)
        );
        self.pools.insert(&pool_id.into(), &pool);
    }

    pub fn unbind_pool(
        &mut self, 
        pool_id: U64,
        token_account_id: &AccountId,
    ) {
        let mut pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.unbind(
            &env::predecessor_account_id(),
            token_account_id
        );
        self.pools.insert(&pool_id.into(), &pool);
    }

    pub fn get_pool_spot_price(
        &self,
        pool_id: U64,
        token_in: &AccountId,
        token_out: &AccountId,
    ) -> U128 {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.get_spot_price(token_in, token_out).into()
    }

    pub fn get_pool_spot_price_sans_fee(
        &self,
        pool_id: U64,
        token_in: &AccountId,
        token_out: &AccountId,
    ) -> U128 {
        let pool = self.pools.get(&pool_id.into()).expect("ERR_NO_POOL");
        pool.get_spot_price_sans_fee(token_in, token_out).into()
    }
}