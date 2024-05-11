#[macro_use]
extern crate rocket;

use artspace_core::env::EnvContainer;
use rocket::http::{Method, Status};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use ws_backend::{env::WSBackendEnvVars, ClerkFairing, WSBackendState};

#[launch]
async fn launch() -> _ {
    let env = ws_backend::env::WSBackendEnv::load_env();

    let origins = env
        .get_env_var(WSBackendEnvVars::CORSOrigins)
        .split(",")
        .map(str::to_owned)
        .collect::<Vec<String>>();

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

    let state = WSBackendState::with_jwks_set().await;

    rocket::build()
        .manage::<WSBackendState>(state)
        .attach(ClerkFairing {})
        .attach(cors)
        .mount("/ws", routes![ping_get, unauthorized_get])
}

#[get("/ping")]
fn ping_get() -> &'static str {
    "Pong"
}

#[get("/unauthorized")]
fn unauthorized_get() -> Status {
    Status::Unauthorized
}
