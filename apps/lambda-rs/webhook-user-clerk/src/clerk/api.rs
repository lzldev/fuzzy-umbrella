use artspace_shared::ClerkPrivateMetadata;
use clerk_rs::models::UpdateUserMetadataRequest;
use serde_json::json;

use crate::{PartialUser, WebhookClerkContext};

pub async fn update_user_metadata(
    partial_user: PartialUser,
    context: &WebhookClerkContext,
) -> Result<
    clerk_rs::models::User,
    clerk_rs::apis::Error<clerk_rs::apis::users_api::UpdateUserMetadataError>,
> {
    let mut update_user_metadata_request = UpdateUserMetadataRequest::new();

    update_user_metadata_request.public_metadata = Some(json!({"nothing":String::from("0")}));

    update_user_metadata_request.private_metadata = Some(
        serde_json::to_value(ClerkPrivateMetadata {
            user_id: partial_user.id,
        })
        .unwrap(),
    );

    clerk_rs::apis::users_api::User::update_user_metadata(
        &context.clerk_client,
        partial_user.clerk_id.as_str(),
        Some(update_user_metadata_request),
    )
    .await
}
