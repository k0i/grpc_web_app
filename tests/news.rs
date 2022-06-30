use std::time::Duration;

use kb::{
    configuration::{self, get_configuration},
    pb::{self, ListNewsRequest},
};
use sqlx::{Connection, PgConnection};

#[tokio::test]
async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let server = kb::run()?;
    let _ = tokio::spawn(async move {
        server.serve(addr).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    let connection_string = get_configuration()
        .expect("failed to load config")
        .database
        .connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let req = ListNewsRequest {
        date_from: "test",
        date_to: "test",
    };
    let mut client =
        pb::news_service_client::NewsServiceClient::connect("http://127.0.0.1:50051").await?;
    let res = client.list_news(req).await?;
    let list_res = res.get_ref();
    assert_eq!(list_res.news.len(), 1);
    Ok(())
}
