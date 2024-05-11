use libsql::de;
use webhook_user_clerk::{clerk::update_user_metadata, PartialUser, WebhookClerkContext};

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

#[tokio::test]
async fn turso_migrate_metadata() {
    let context = WebhookClerkContext::new().await;
    let conn = context.database.connect().unwrap();

    let mut query = conn
        .query("SELECT id,clerk_id FROM users", libsql::params![])
        .await
        .unwrap();

    let mut n = 0;
    while let Ok(row) = query.next().await {
        let row = match row {
            Some(row) => row,
            None => break,
        };

        let user = de::from_row::<PartialUser>(&row).unwrap();
        let _ = update_user_metadata(user, &context).await.unwrap();

        eprintln!("Updated {n} users.");
        n = n + 1;
    }
}
