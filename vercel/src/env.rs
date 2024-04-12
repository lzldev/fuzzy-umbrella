pub fn validate_env() {
    let env_vars = vec!["TURSO_CONNECTION_URL", "TURSO_AUTH_TOKEN", "WEBHOOK_SECRET"];

    for var in env_vars {
        if std::env::var(var).is_err() {
            panic!("{} must be set", var);
        }
    }
}

pub struct TursoOptions {
    pub connection_url: String,
    pub auth_token: String,
}

pub fn get_turso_options() -> TursoOptions {
    TursoOptions {
        connection_url: std::env::var("TURSO_CONNECTION_URL")
            .expect("TURSO_CONNECTION_URL must be set"),
        auth_token: std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set"),
    }
}
