use std::cmp::Ordering;
use near_sdk::{
    env,
    json_types::{
        U128
    },
    AccountId,
    borsh::{
        BorshDeserialize, BorshSerialize
    },
    collections::{
        UnorderedMap,
        Vector
    }
};

use crate::constants::{
    MAX_BOUND_TOKENS, 
    MIN_BOUND_TOKENS,
    MAX_FEE,
    MIN_FEE,
    MIN_WEIGHT,
    MAX_WEIGHT,
    EXIT_FEE,
    MIN_BALANCE,
    MAX_TOTAL_WEIGHT,
    INIT_POOL_SUPPLY
};

use crate::math;
use crate::token::FungibleTokenVault;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Record {
    pub bound: bool, // is this token record bound to the pool
    pub index: u64, // index of this record in list of records
    pub denorm: u128, // denormalized weight of this token 
    pub balance: u128, // pool balance of this token
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pool {
    id: u64,
    total_weight: u128,
    swap_fee: u128,
    finalized: bool,
    controller: AccountId,
    token: FungibleTokenVault,
    pub records: UnorderedMap<AccountId, Record>,
    pub tokens: Vector<AccountId>
}

impl Pool {
    /**
     * @notice Creates new `Pool` instance
     * @param swap_fee The 
     */
    pub fn new(
        sender: AccountId, 
        id: u64, 
        swap_fee: u128
    ) -> Self {
        assert!(swap_fee <= MAX_FEE, "ERR_MAX_FEE");
        assert!(swap_fee >= MIN_FEE, "ERR_MIN_FEE");

        Self {
            id,
            total_weight: 0,
            swap_fee,
            finalized: false,
            controller: sender,
            token: FungibleTokenVault::new(id),
            records: UnorderedMap::new(format!("records:{}", id).as_bytes().to_vec()),
            tokens: Vector::new(format!("tokens:{}", id).as_bytes().to_vec()),
        }
    }

    pub fn is_finalized(&self) -> bool { 
        self.finalized
    }

    pub fn is_bound(&self, token_account_id: &AccountId) -> bool {
        match self.records.get(token_account_id) {
            Some(record) => record.bound,
            None => false
        }
    }

    pub fn get_controller(&self) -> AccountId {
        self.controller.to_string()
    }

    pub fn get_num_tokens(&self) -> u64 {
        self.tokens.len()
    }

    pub fn get_current_tokens(&self) -> Vec<AccountId> {
        self.tokens.to_vec()
    }

    pub fn get_final_tokens(&self) -> Vec<AccountId> {
        assert!(self.finalized, "ERR_NOT_FINALIZED");
        self.tokens.to_vec()
    }

    pub fn get_balance(&self, token_account_id: &AccountId) -> u128 {
        self.records
            .get(token_account_id)
            .expect("ERR_NO_RECORD")
            .balance
    }

    pub fn get_pool_token_balance(&self, account_id: &AccountId) -> u128 {
        self.token.get_balance(account_id)
    }
 
    pub fn get_pool_token_total_supply(&self) -> u128 {
        self.token.total_supply()
    }

    pub fn get_swap_fee(&self) -> u128 {
        self.swap_fee
    }

    pub fn finalize(&mut self, sender: &AccountId) {
        assert!(!self.finalized, "ERR_IS_FINALIZED");
        assert!(self.get_num_tokens() >= MIN_BOUND_TOKENS, "ERR_MIN_TOKENS");
        assert_eq!(sender, &self.controller, "ERR_NO_CONTROLLER");

        self.finalized = true;
        self.token.mint(INIT_POOL_SUPPLY, sender);
    }

    pub fn bind(&mut self, 
        sender: &AccountId, 
        token_account_id: &AccountId, 
        denorm: u128, 
        balance: u128
    ) {
        assert_eq!(sender, &self.controller, "ERR_NO_CONTROLLER");
        assert!(env::is_valid_account_id(token_account_id.as_bytes()), "ERR_INVALID_ACCOUNT_ID");
        assert!(!self.is_bound(&token_account_id), "ERR_is_BOUND");
        assert!(!self.finalized, "ERR_IS_FINALIZED");
        assert!(self.get_num_tokens() < MAX_BOUND_TOKENS, "ERR_MAX_TOKENS");
        
        let new_record = Record {
            bound: true,
            index: self.get_num_tokens(),
            denorm: 0,
            balance: 0
        };
        
        self.records.insert(token_account_id, &new_record);
        self.tokens.push(token_account_id);
        self.rebind(sender, token_account_id, denorm, balance);
    }
    
    pub fn rebind(
        &mut self, 
        sender: &AccountId, 
        token_account_id: &AccountId, 
        denorm: u128, 
        balance: u128
    ) {
        assert_eq!(sender, &self.controller, "ERR_NO_CONTROLLER");
        assert!(env::is_valid_account_id(token_account_id.as_bytes()), "ERR_INVALID_ACCOUNT_ID");
        assert!(self.is_bound(token_account_id), "ERR_NOT_BOUND");
        assert!(!self.finalized, "ERR_IS_FINALIZED");
        
        assert!(denorm >= MIN_WEIGHT, "ERR_MIN_WEIGHT");
        assert!(denorm <= MAX_WEIGHT, "ERR_MAX_WEIGHT");
        assert!(balance >= MIN_BALANCE, "ERR_MIN_BALANCE");
        
        let mut record = self.records.get(token_account_id).expect("ERR_NO_RECORD");
        
        let old_weight = record.denorm;
        match denorm.cmp(&old_weight) {
            Ordering::Greater => {
                self.total_weight += denorm - old_weight;
                assert!(self.total_weight <= MAX_TOTAL_WEIGHT, "ERR_MAX_TOTAL_WEIGHT");
            },
            Ordering::Less => {
                self.total_weight -= old_weight - denorm;
            }, 
            Ordering::Equal => ()
        };

        record.denorm = denorm;

        // let old_balance = record.balance;
        record.balance = balance;

        self.records.insert(token_account_id, &record);
        
        // match balance.cmp(&old_balance) {
        //     Ordering::Greater => {
        //         // Transfer from user to this contract 
        //         // token(token_account_id).transfer_from(env::predecessor_account(), old_balance - balance)
        //     },
        //     Ordering::Less => {
        //         // let token_balance_withdraw = old_balance - balance;
        //         // let token_exit_fee = token_balance_withdraw * EXIT_FEE; // TODO: do we need this?

        //         // Transfer from contract to this user
        //         // token(token_account_id).transfer(env::predecessor_account(), token_balance_withdraw - token_exit_fee)
        //     },
        //     Ordering::Equal => ()
        // }
        
    }

    pub fn unbind(
        &mut self, 
        sender: &AccountId, 
        token_account_id: &AccountId
    ) {
        assert_eq!(sender, &self.controller, "ERR_NO_CONTROLLER");
        assert!(self.is_bound(token_account_id), "ERR_NOT_BOUND");
        assert!(!self.finalized, "ERR_IS_FINALIZED");

        let record = self.records.get(token_account_id).expect("ERR_NO_RECORD");

        // let token_balance = record.balance;
        // let token_exit_fee = record.balance * EXIT_FEE;

        self.total_weight -= record.denorm;
        
        let index = record.index;
        self.tokens.swap_remove(index);
        self.records.remove(token_account_id);

        // token(token_account_id).transfer(env::predecessor_account(), token_balance - token_exit_fee)
    }

    // TODO: Gulp function requires async balance checks, will only work when pools are sharded

    pub fn get_spot_price(
        &self, 
        token_in: &AccountId, 
        token_out: &AccountId
    ) -> u128 {
        assert!(self.is_bound(token_in), "ERR_NOT_BOUND");
        assert!(self.is_bound(token_out), "ERR_NOT_BOUND");
        let record_in = self.records.get(token_in).expect("ERR_NO_RECORD");
        let record_out = self.records.get(token_out).expect("ERR_NO_RECORD");

        math::calc_spot_price(record_in.balance, record_in.denorm, record_out.balance, record_out.denorm, self.swap_fee)
    }

    pub fn get_spot_price_sans_fee(
        &self, 
        token_in: &AccountId, 
        token_out: &AccountId
    ) -> u128 {
        assert!(self.is_bound(token_in), "ERR_NOT_BOUND");
        assert!(self.is_bound(token_out), "ERR_NOT_BOUND");
        let record_in = self.records.get(token_in).expect("ERR_NO_RECORD");
        let record_out = self.records.get(token_out).expect("ERR_NO_RECORD");

        math::calc_spot_price(record_in.balance, record_in.denorm, record_out.balance, record_out.denorm, 0)
    }

    pub fn join_pool(
        &mut self,
        sender: &AccountId,
        pool_amount_out: u128,
        max_amounts_in: Vec<U128>
    ) {
        assert!(self.finalized, "ERR_NOT_FINALIZED");
        assert_eq!(max_amounts_in.len() as u64, self.get_num_tokens(), "ERR_AMOUNTS_LEN");

        let pool_total = self.token.total_supply();
        let ratio = math::div_u128(pool_amount_out, pool_total);
        assert_ne!(ratio, 0, "ERR_MATH_APPROX");

        for (i, token) in self.tokens.iter().enumerate() {
            let mut record = self.records
                .get(&token)
                .expect("ERR_NO_RECORD");

            let balance = record.balance;
            
            let token_amount_in = math::mul_u128(ratio, balance);

            assert_ne!(token_amount_in, 0, "ERR_MATH_APPROX");
            assert!(token_amount_in <= u128::from(max_amounts_in[i]), "ERR_LIMIT_IN");

            record.balance += token_amount_in;
            
            self.records.insert(&token, &record);

            // TODO: Transfer tokens in from user
        }

        self.token.mint(pool_amount_out, sender);
    }

    pub fn exit_pool(
        &mut self,
        sender: &AccountId,
        pool_amount_in: u128,
        min_amounts_out: Vec<U128>
    ) {
        assert!(self.finalized, "ERR_NOT_FINALIZED");
        assert_eq!(min_amounts_out.len() as u64, self.get_num_tokens(), "ERR_AMOUNTS_LEN");

        let pool_total = self.token.total_supply();
        let exit_fee = math::mul_u128(pool_amount_in, EXIT_FEE);
        let pool_amount_in_min_exit_fee = pool_amount_in - exit_fee;
        let ratio = math::div_u128(pool_amount_in_min_exit_fee, pool_total);
        assert_ne!(ratio, 0, "ERR_MATH_APPROX");

        // Make sure this method is called by on_vault_received I think? 
        // Burn `pool_amount_in` pool_tokens 
        // TODO: Temp burn fn
        self.token.faux_burn(pool_amount_in);
        for (i, token) in self.tokens.iter().enumerate() {
            let mut record = self.records
                .get(&token)
                .expect("ERR_NO_RECORD");
            let balance = record.balance;
            let token_amount_out = math::mul_u128(ratio, balance);
            
            assert_ne!(token_amount_out, 0, "ERR_MATH_APPROX");
            assert!(token_amount_out >= u128::from(min_amounts_out[i]), "ERR_LIMIT_OUT");

            record.balance -= token_amount_out;

            self.records.insert(&token, &record);
            // Transfer token to user
        }

    }
}
