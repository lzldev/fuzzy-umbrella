use artspace_core::env::EnvContainer;
use rocket::http::{CookieJar, Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use ws_backend::env::WSBackendEnvVars;

#[macro_use]
extern crate rocket;

#[launch]
fn launch() -> _ {
    let env = ws_backend::env::WSBackendEnv::load_env();

    let origins = env.get_env_var(WSBackendEnvVars::CORSOrigins);
    let origins = origins
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
    .expect("To build cors options.");

    rocket::build().mount("/", routes![ping_get]).attach(cors)
}

#[get("/ping")]
fn ping_get(cookies: &CookieJar<'_>) -> &'static str {
    dbg!(&cookies.get("__session"));
    "Pong"
}
