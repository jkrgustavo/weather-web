use anyhow::Result;

#[tokio::test]
async fn index_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let response = hc.do_get("/");
    response.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn quick_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let response = hc.do_get("/weather/seattle");
    response.await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn weather_test() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    let response = hc.do_get("/coords/austin");
    response.await?.print().await?;

    Ok(())
}
