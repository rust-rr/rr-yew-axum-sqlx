use anyhow::Result;
use serde_json::json;

// cargo watch -q -c -w examples/ -x "run --example quick_dev"
#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/Richard").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "test",
            "password": "welcome"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "ticketAAA"
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_delete("/api/tickets/0").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
