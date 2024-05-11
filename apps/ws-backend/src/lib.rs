pub mod auth;
pub mod env;
pub mod jwt;

use anyhow::anyhow;
use jsonwebtoken::TokenData;
use jwt::Claims;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};

pub struct WSBackendState {}
impl WSBackendState {}

#[derive(Debug)]
pub struct ClerkUser<'r> {
    pub token: &'r TokenData<Claims>,
}

#[allow(unused_variables, dead_code)]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClerkUser<'r> {
    type Error = anyhow::Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = request
            .local_cache_async(async {
                dbg!("Cache Miss");

                let session_cookie = match request.cookies().get("__session") {
                    Some(r) => r,
                    None => {
                        return Err::<TokenData<Claims>, anyhow::Error>(anyhow!(
                            "Session not found"
                        ));
                    }
                };

                panic!("Tried restoring Token cache without running ClerkFairing")
            })
            .await;

        if let Err(err) = token {
            return request::Outcome::Error((Status::Unauthorized, anyhow!(Status::Unauthorized)));
        }

        let token = token.as_ref().unwrap();

        request::Outcome::Success(Self { token })
    }
}
