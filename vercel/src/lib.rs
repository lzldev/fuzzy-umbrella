use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookMessage {
    #[serde(rename = "type")]
    _type: String,
    object: String,
    data: CreateUserEventData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserEventData {
    id: String,
    object: String,
    birthday: String,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    external_id: Option<String>,
    primary_email_address_id: Option<String>,
    primary_phone_number_id: Option<String>,
    primary_web3_wallet_id: Option<String>,
    image_url: Option<String>,
    gender: String,
    email_addresses: Vec<ClerkEmail>,
    password_enabled: bool,
    two_factor_enabled: bool,
    created_at: f64,
    updated_at: f64,
    last_sign_in_at: f64,
    external_accounts: Vec<ClerkExternalAccount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClerkExternalAccount {
    id: String,
    object: String,
    provider: String,
    identification_id: String,
    provider_user_id: String,
    approved_scopes: String,
    email_address: String,
    first_name: String,
    last_name: String,
    image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClerkEmail {
    id: String,
    object: String,
    email_address: String,
    verification: ClerkEmailVerfication,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClerkEmailVerfication {
    status: String,
    strategy: String,
}
