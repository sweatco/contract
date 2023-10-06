use model::ContractNameInterfaceIntegration;

#[tokio::test]
async fn happy_flow() -> anyhow::Result<()> {
    use crate::common::{prepare_contract, Prepared};

    println!("👷🏽 Run happy flow test");

    let Prepared {
        mut context,
        manager: _,
        alice: _,
        fee_account: _,
    } = prepare_contract().await?;

    assert_eq!(context.contract.receive_name().await?, "Default name");

    context.contract.set_name("New name".to_string()).await?;

    assert_eq!(context.contract.receive_name().await?, "New name");

    Ok(())
}
