use libsql::Builder as TursoBuilder;

#[tokio::test]
async fn connect_to_libsql() {
    let database = TursoBuilder::new_remote(
        std::env::var("TURSO_URL").unwrap(),
        std::env::var("TURSO_TOKEN").unwrap(),
    )
    .build()
    .await
    .expect("Couldn't open connection to turso database.");

    let db = database
        .connect()
        .expect("Couldn't open connection to turso.");

    let mut select = db
        .query("SELECT * FROM users LIMIT 1;", libsql::params![])
        .await
        .unwrap();

    while let Some(row) = select.next().await.unwrap() {
        eprintln!("{row:#?}");
    }
}
