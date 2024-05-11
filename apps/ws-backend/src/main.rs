#[macro_use]
extern crate rocket;

use artspace_core::env::EnvContainer;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use ws_backend::{
    auth::{ClerkFairing, WSBackendJWKS},
    env::WSBackendEnvVars,
    ClerkUser, WSBackendState,
};

#[launch]
async fn launch() -> _ {
    let env = ws_backend::env::WSBackendEnv::load_env();

    let origins = env
        .get_env_var(WSBackendEnvVars::CORSOrigins)
        .split(",")
        .map(str::to_owned)
        .collect::<Vec<String>>();

    let jwks_state =
        WSBackendJWKS::from_uri(env.get_env_var(WSBackendEnvVars::ClerkJWKSUrl).as_str())
            .await
            .expect("To create a jwks state.");

    dbg!(&origins);

    let allowed_origins = AllowedOrigins::some_exact(origins.as_slice());

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("to build cors options.");

    let state = WSBackendState {};

    rocket::build()
        .manage::<WSBackendJWKS>(jwks_state)
        .manage::<WSBackendState>(state)
        .attach(ClerkFairing {})
        .attach(cors)
        .mount(
            "/ws",
            routes![ws_backend::auth::unauthorized_get, ping_get, ping_clerk_get],
        )
}

#[get("/ping")]
fn ping_get() -> &'static str {
    "Pong"
}

#[get("/ping/clerk")]
fn ping_clerk_get(clerk: ClerkUser<'_>) -> &'static str {
    dbg!(&clerk);
    "Pong"
}
