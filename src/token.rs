use near_sdk::{
	AccountId,
	env,
	near_bindgen,
	collections::{
		LookupMap,
		LookupSet,
		Vector
	},
	borsh::{
		self,
		BorshDeserialize, 
		BorshSerialize
	}
};

type TokenId = u64;
/**
 * Balance map Map<account_id, Vec<token_ids>>
 * transfers
 * transfer_from 
 * set_approval
 * get_tokens
 * get_token
 * get_owner
 * mint
 */

/** 
 * @title Non-funbile-token
 */
#[derive(BorshDeserialize, BorshSerialize)]
struct GoogleImageToken {
	url: String,
	mint_time: u64,
	owner: AccountId
}

/**
 * @notice The state struct for the NFT implementation 
 */
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct NFT {
	nonce: u64,
	allowances: LookupMap<AccountId, LookupMap<AccountId, LookupSet<TokenId>>>,
	balances: LookupMap<AccountId, Vector<TokenId>>,
	tokens: LookupMap<TokenId, GoogleImageToken>,
}

impl Default for NFT {
    fn default() -> Self {
        panic!("Contract should be initialized before usage")
    }
}

#[near_bindgen]
impl NFT {
		
	#[init]
	pub fn init() -> Self {
		Self {
			nonce: 0,
			allowances: LookupMap::new(b"allowances".to_vec()),
			balances: LookupMap::new(b"balances".to_vec()),
			tokens: LookupMap::new(b"tokens".to_vec()),
		}
	}

	pub fn mint(&mut self, owner: AccountId, url: String) -> u64 {
		let token_id = self.nonce;

		let token: GoogleImageToken = GoogleImageToken {
			url,
			mint_time: env::block_timestamp() / 100000,
			owner: owner.to_string(),
		};

		self.tokens.insert(&self.nonce, &token);
		
		let mut current_tokens = self.balances.get(&owner).unwrap_or(Vector::new(format!("balance_{}", owner).as_bytes().to_vec()));
		
		current_tokens.push(&self.nonce);
		self.balances.insert(&owner, &current_tokens);
		self.nonce += 1;

		return token_id;
	}
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
	use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{VMContext, testing_env};
	use near_runtime_standalone::{RuntimeStandalone};
	use near_primitives::transaction::{ExecutionStatus, ExecutionOutcome};

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1000 * 10u128.pow(24),
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

	fn alice() -> AccountId {
		return "alice".to_string();
	}
	
	fn bob() -> AccountId {
		return "bob".to_string();
	}

	fn image_address() -> String {
		return "https://i.kym-cdn.com/photos/images/newsfeed/001/499/826/2f0.png".to_string();
	}

	#[test]
	fn test_init() {
		let mut context = get_context(alice());
		testing_env!(context);
		let contract = NFT::init();
	}

	#[test]
	fn test_mint() {
		let mut context = get_context(alice());
		testing_env!(context);
		let mut contract = NFT::init();

		let token = contract.mint(alice(), image_address());
		assert_eq!(token, 0);
	}
}