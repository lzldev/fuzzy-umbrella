use libsql::{Builder, Connection, Database};

use crate::env::get_turso_options;

pub struct DB {
    pub db: Database,
    pub connection: Connection,
}

impl DB {
    pub async fn connect() -> Connection {
        let turso_options = get_turso_options();

        let db = Builder::new_remote(turso_options.connection_url, turso_options.auth_token)
            .build()
            .await
            .expect("Failed to connect to Turso");

        db.connect().unwrap()
    }
}
