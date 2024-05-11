use libsql::de;
use webhook_user_clerk::{PartialUser, WebhookClerkContext};

#[tokio::test]
async fn turso_test() {
    let context = WebhookClerkContext::new().await;
    let conn = context.database.connect().unwrap();

    let mut query = conn
        .query("SELECT id FROM users LIMIT 1", libsql::params![])
        .await
        .unwrap();

    let row = query.next().await.unwrap().unwrap();

    eprintln!("Query : {:#?}", row);

    let dde = de::from_row::<PartialUser>(&row).unwrap();

    eprintln!("DDE : {:#?}", dde);
}
