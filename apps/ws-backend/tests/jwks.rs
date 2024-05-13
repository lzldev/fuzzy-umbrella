use std::env;

use jsonwebtoken::jwk::JwkSet;
use serde_json::json;
use ws_backend::jwt::decode_jwt;

#[tokio::test]
async fn refactor_jwks_test() -> Result<(), anyhow::Error> {
    let jwks_kid = env::var("JWKS_KID").expect("JWKS_KID ENV VAR");
    let jwks_n = env::var("JWKS_N").expect("JWKS_N ENV VAR");
    let jwks_e = env::var("JWKS_E").expect("JWKS_E ENV VAR");
    let test_token = env::var("TEST_TOKEN").expect("TEST_TOKEN ENV VAR");

    let set = serde_json::from_value::<JwkSet>(json!(
    {
      "keys": [
        {
          "use": "sig",
          "kty": "RSA",
          "kid":jwks_kid,
          "alg": "RS256",
          "n":jwks_n,
          "e":jwks_e
        }
      ]
    }))
    .expect("JWKSet To be parsed");

    let token = decode_jwt(&test_token, &set)?;

    Ok(())
}
