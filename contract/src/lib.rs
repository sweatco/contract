use model::ContractNameInterface;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub name: String,
}

#[near_bindgen]
impl Contract {
    // pub fn init() -> Self {
    //     Self {
    //         name: "Alice".to_string(),
    //     }
    // }
}

#[near_bindgen]
impl ContractNameInterface for Contract {
    #[init]
    #[must_use]
    fn init() -> Self {
        todo!()
    }

    #[init]
    #[must_use]
    fn init_with_name(name: String) -> Self {
        todo!()
    }

    fn return_none(&mut self) {
        todo!()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
