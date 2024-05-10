use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookMessage {
    #[serde(rename = "type")]
    pub _type: String,
    pub object: String,
    pub data: CreateUserEventData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserEventData {
    pub id: String,
    pub object: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub external_id: Option<String>,
    pub primary_email_address_id: Option<String>,
    pub primary_phone_number_id: Option<String>,
    pub primary_web3_wallet_id: Option<String>,
    pub image_url: Option<String>,
    pub email_addresses: Vec<ClerkEmail>,
    pub password_enabled: bool,
    pub two_factor_enabled: bool,
    pub created_at: f64,
    pub updated_at: f64,
    pub last_sign_in_at: Option<f64>,
    pub external_accounts: Vec<ClerkExternalAccount>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkExternalAccount {
    pub id: String,
    pub object: String,
    pub provider: String,
    pub identification_id: String,
    pub provider_user_id: String,
    pub approved_scopes: String,
    pub email_address: String,
    pub first_name: String,
    pub last_name: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkEmail {
    pub id: String,
    pub object: String,
    pub email_address: String,
    pub verification: ClerkEmailVerfication,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClerkEmailVerfication {
    pub status: String,
    pub strategy: String,
}
