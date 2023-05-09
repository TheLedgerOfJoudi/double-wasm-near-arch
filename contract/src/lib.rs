use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TokenOwners {
    owners_to_tokens: UnorderedMap<String, String>,
}

impl Default for TokenOwners {
    fn default() -> Self {
        Self {
            owners_to_tokens: UnorderedMap::new(b"a"),
        }
    }
}

#[near_bindgen]
impl TokenOwners {
    pub fn set_info(&mut self, token_id: &String, owner_id: &String) {
        self.owners_to_tokens.insert(&owner_id, &token_id);
    }

    pub fn get_token(&self, owner_id: &String) -> String {
        match self.owners_to_tokens.get(&owner_id) {
            Some(token) => token,
            None => "No token".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    #[test]
    fn run() {
        let context: VMContext = VMContextBuilder::new().context;
        testing_env!(context);
        let mut contract: TokenOwners = TokenOwners::default();
        let token_id: String = "your_token".to_string();
        let owner_id: String = "you.testnet".to_string();
        contract.set_info(&token_id, &owner_id);
        let token_of_owner: String = contract.get_token(&owner_id);
        assert_eq!(token_of_owner, token_id);
    }
}
