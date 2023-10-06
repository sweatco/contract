use std::{collections::HashMap, env, fs};

use integration_utils::build::build_contract;
use near_units::parse_near;
use workspaces::{network::Sandbox, Account, Worker};

use crate::contract_name_interface::ContractName;

pub const EPOCH_BLOCKS_HEIGHT: u64 = 43_200;
pub const HOURS_PER_EPOCH: u64 = 12;
pub const ONE_HOUR_BLOCKS_HEIGHT: u64 = EPOCH_BLOCKS_HEIGHT / HOURS_PER_EPOCH;

pub struct Context {
    worker: Worker<Sandbox>,
    root_account: Account,
    pub accounts: HashMap<String, Account>,
    pub contract: ContractName,
}

impl Context {
    pub(crate) async fn new() -> anyhow::Result<Context> {
        println!("🏭 Initializing context");

        build_contract()?;

        let worker = workspaces::sandbox().await?;
        let root_account = worker.dev_create_account().await?;

        let contract = worker.dev_deploy(&Self::load_wasm("../res/contract_name.wasm")).await?;

        println!("@@ contract deployed to {}", contract.id());

        Ok(Context {
            worker,
            root_account,
            accounts: HashMap::new(),
            contract: ContractName { contract },
        })
    }

    pub(crate) async fn account(&mut self, name: &str) -> anyhow::Result<Account> {
        if !self.accounts.contains_key(name) {
            let account = self
                .root_account
                .create_subaccount(name)
                .initial_balance(parse_near!("3 N"))
                .transact()
                .await?
                .into_result()?;

            self.accounts.insert(name.to_string(), account);
        }

        Ok(self.accounts.get(name).unwrap().clone())
    }

    fn load_wasm(wasm_path: &str) -> Vec<u8> {
        let current_dir = env::current_dir().expect("Failed to get current dir");
        let wasm_filepath = fs::canonicalize(current_dir.join(wasm_path)).expect("Failed to get wasm file path");
        fs::read(wasm_filepath).expect("Failed to load wasm")
    }

    pub async fn fast_forward_hours(&self, hours: u64) -> anyhow::Result<()> {
        let blocks_to_advance = ONE_HOUR_BLOCKS_HEIGHT * hours;

        println!("⏳ Fast forward to {hours} hours ({blocks_to_advance} blocks)...");

        self.worker.fast_forward(blocks_to_advance).await?;

        Ok(())
    }
}
