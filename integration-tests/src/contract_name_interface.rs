use async_trait::async_trait;
use model::ContractNameInterfaceIntegration;
use workspaces::Contract;

pub struct ContractName {
    pub contract: Contract,
}

#[async_trait]
impl ContractNameInterfaceIntegration for ContractName {
    async fn init(&self) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn init_with_name(&self, name: String) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn return_none(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    async fn get_name(&self) -> anyhow::Result<String> {
        todo!()
    }
    // async fn init(&self) -> anyhow::Result<()> {
    //     println!("▶️ Init ft contract");
    //
    //     self.call("new")
    //         .args_json(json!({
    //             "postfix": ".u.sweat.testnet",
    //         }))
    //         .max_gas()
    //         .transact()
    //         .await?
    //         .into_result()?;
    //
    //     Ok(())
    // }
}
