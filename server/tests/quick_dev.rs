use anyhow::Result;

// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/Richard").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
