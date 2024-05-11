use std::borrow::Cow;

use anyhow::anyhow;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{uri::Origin, Status},
    request::{self, FromRequest},
    Data, Request,
};

pub mod env;

pub struct WSBackendState {}
impl WSBackendState {
    pub async fn with_jwks_set() -> Self {
        Self {}
    }
}

pub struct ClerkUser;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClerkUser {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let state = req
            .rocket()
            .state::<WSBackendState>()
            .expect("To get rocket state");

        let session_cookie = match req.cookies().get("__session") {
            Some(r) => r,
            None => {
                return request::Outcome::Error((
                    Status::Unauthorized,
                    anyhow!(Status::Unauthorized),
                ))
            }
        };

        /* .. */
        request::Outcome::Success(Self {})
    }
}

pub struct ClerkFairing;

#[rocket::async_trait]
impl Fairing for ClerkFairing {
    fn info(&self) -> Info {
        Info {
            name: "Clerk Fairing",
            kind: rocket::fairing::Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let uri = request.uri().path();
        dbg!(&uri);

        if uri == "/ws/unauthorized" {
            return;
        }

        let session_cookie = match request.cookies().get("__session") {
            Some(c) => c,
            None => {
                dbg!("Unauuth");
                request.set_uri(Origin::parse("/ws/unauthorized").unwrap());
                return ();
            }
        };

        let state = request
            .rocket()
            .state::<WSBackendState>()
            .expect("To Get rocket state");

        dbg!(&session_cookie);
    }
}
