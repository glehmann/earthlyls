mod common;

use crate::common::*;

#[tokio::test]
async fn should_initialize() {
    TestContext::new().await.initialize().await;
    // panic!("Don’t panic!");
}
