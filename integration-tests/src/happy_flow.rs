use crate::common::{prepare_contract, Prepared};

#[tokio::test]
async fn happy_flow() -> anyhow::Result<()> {
    println!("👷🏽 Run happy flow test");

    let Prepared {
        context: _,
        manager: _,
        alice: _,
        fee_account: _,
    } = prepare_contract().await?;

    Ok(())
}
