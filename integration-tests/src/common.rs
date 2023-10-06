use model::ContractNameInterfaceIntegration;
use workspaces::Account;

use crate::context::Context;

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
