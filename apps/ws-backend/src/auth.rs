use jsonwebtoken::{jwk::JwkSet, TokenData};
use rocket::{
    fairing::{Fairing, Info},
    http::{uri::Origin, Status},
    Data, Request,
};

use crate::jwt::{decode_jwt, Claims};

/**
 * Work around to return errors from fairing.
 */
#[rocket::get("/unauthorized")]
pub fn unauthorized_get() -> Status {
    Status::Unauthorized
}

#[derive(Debug)]
pub struct WSBackendJWKS {
    pub jwks_set: JwkSet,
}

impl WSBackendJWKS {
    pub async fn from_uri(uri: &str) -> Result<Self, anyhow::Error> {
        let req = reqwest::get(uri).await?;

        let jwks_set = req.json::<JwkSet>().await?;

        Ok(Self { jwks_set })
    }
}

pub struct ClerkFairing;

#[rocket::async_trait]
#[allow(unused_variables, dead_code)]
impl Fairing for ClerkFairing {
    fn info(&self) -> Info {
        Info {
            name: "Clerk Fairing",
            kind: rocket::fairing::Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let uri = request.uri().path();

        if uri == "/ws/unauthorized" {
            return;
        }

        let session_cookie = match request.cookies().get("__session") {
            Some(c) => c,
            None => {
                request.set_uri(Origin::parse("/ws/unauthorized").unwrap());
                return ();
            }
        };

        let state = request
            .rocket()
            .state::<WSBackendJWKS>()
            .expect("To Get rocket state");

        let token = match decode_jwt(session_cookie.value(), &state.jwks_set) {
            Ok(token) => token,
            Err(err) => {
                eprintln!("Auth Error {err:#?}");
                request.set_uri(Origin::parse("/ws/unauthorized").unwrap());
                return ();
            }
        };

        request
            .local_cache_async(async { Ok::<TokenData<Claims>, anyhow::Error>(token) })
            .await;
    }
}
