use std::env;

use chrono::Utc;
use sqlx::{postgres::PgConnection, Connection};
use webhook_user_clerk::PartialUser;

#[tokio::test]
pub async fn pg() {
    let mut conn = PgConnection::connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .expect("To connect to SQL");

    conn.ping().await.expect("TO PING DB");

    let new_user = sqlx::query_as::<_,PartialUser>("INSERT INTO users (clerk_id,username,email, image_url, clerk_updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING id,clerk_id")
        .bind("userid_xxxx".to_owned())
        .bind("username-12345".to_owned())
        .bind("example@example.com".to_owned())
        .bind(Option::<String>::None)
        .bind(Utc::now().naive_utc())
        .fetch_one(&mut conn).await.expect("To insert user");

    dbg!(&new_user);

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let delete_user = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(new_user.id)
        .execute(&mut conn)
        .await
        .expect("To delete user")
        .rows_affected();

    dbg!(&delete_user);
}
