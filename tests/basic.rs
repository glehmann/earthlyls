mod common;

use crate::common::*;

#[tokio::test]
async fn should_initialize() -> anyhow::Result<()> {
    TestContext::new().await?.initialize().await?;
    // panic!("Don’t panic!");
    Ok(())
}
