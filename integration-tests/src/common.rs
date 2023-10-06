use std::{
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
};

use ed25519_dalek::{SigningKey, VerifyingKey};
use model::ContractNameInterfaceIntegration;
use rand::rngs::OsRng;
use serde_json::Value;
use workspaces::Account;

use crate::context::Context;

pub trait ValueGetters {
    fn get_u128(&self, key: &str) -> u128;
    fn get_interest(&self) -> u128;
    fn get_jar_id(&self) -> String;
}

impl ValueGetters for Value {
    fn get_u128(&self, key: &str) -> u128 {
        self.as_object()
            .unwrap()
            .get(key)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
            .parse::<u128>()
            .unwrap()
    }

    fn get_interest(&self) -> u128 {
        self.as_object().unwrap().get("amount").unwrap().get_u128("total")
    }

    fn get_jar_id(&self) -> String {
        self.as_object()
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}

static CONTRACT_READY: AtomicBool = AtomicBool::new(false);

/// Compile contract in release mode and prepare it for integration tests usage
pub fn build_contract() -> anyhow::Result<()> {
    if CONTRACT_READY.load(Ordering::Relaxed) {
        return Ok(());
    }

    Command::new("make")
        .arg("build")
        .current_dir("..")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    CONTRACT_READY.store(true, Ordering::Relaxed);

    Ok(())
}

pub struct Prepared {
    pub context: Context,
    pub manager: Account,
    pub alice: Account,
    pub fee_account: Account,
}

pub async fn prepare_contract() -> anyhow::Result<Prepared> {
    let mut context = Context::new().await?;

    let manager = context.account("manager").await?;
    let alice = context.account("alice").await?;
    let fee_account = context.account("fee").await?;

    context.contract.init().await?;

    Ok(Prepared {
        context,
        manager,
        alice,
        fee_account,
    })
}

#[must_use]
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key: VerifyingKey = VerifyingKey::from(&signing_key);

    (signing_key, verifying_key)
}
