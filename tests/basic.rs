mod common;

use std::path::Path;

use crate::common::*;

#[tokio::test]
async fn should_initialize() -> anyhow::Result<()> {
    TestContext::new().await?.initialize(Path::new("/tmp")).await?;
    Ok(())
}
