use anyhow::{anyhow, Context};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub iss: String,
    pub azp: String,
    pub sid: String,
    pub sub: String,
}

pub fn decode_jwt(token: &str, key_set: &JwkSet) -> Result<TokenData<Claims>, anyhow::Error> {
    let header = decode_header(token).context("JWT Parsing Invalid Header")?;

    match &header.alg {
        jsonwebtoken::Algorithm::RS256
        | jsonwebtoken::Algorithm::RS384
        | jsonwebtoken::Algorithm::RS512 => (),
        _ => return Err(anyhow!("Token Algorithm not supported")),
    };

    if header.kid.is_none() {
        return Err(anyhow!("Token header has no KID"));
    }

    let kid = header.kid;

    let key = match key_set
        .keys
        .iter()
        .find(|key| key.common.key_id.is_some() && key.common.key_id == kid)
    {
        Some(jwk) => jwk,
        None => return Err(anyhow!("Key id not found in current set.")),
    };

    let key_params = match &key.algorithm {
        jsonwebtoken::jwk::AlgorithmParameters::RSA(rsa) => rsa,
        _ => return Err(anyhow!("JWK isn't using RSA")), // This should also check the token's algo.
    };

    return decode::<Claims>(
        token,
        &DecodingKey::from_rsa_components(&key_params.n, &key_params.e).unwrap(),
        &Validation::new(jsonwebtoken::Algorithm::RS256),
    )
    .context("Token Decode Failed");
}
