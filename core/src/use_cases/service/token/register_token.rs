use crate::domain::{dtos::token::Token, entities::TokenRegistration};

use clean_base::{
    entities::default_response::CreateResponseKind, utils::errors::MappedErrors,
};

use super::generate_token_expiration_time;

/// Register a new token.
///
/// This function should be useful to send a new token to the data repository.
pub async fn register_token(
    own_service: String,
    token_registration_repo: Box<&dyn TokenRegistration>,
) -> Result<CreateResponseKind<Token>, MappedErrors> {
    token_registration_repo
        .create(
            Token::new_undated_token(own_service).await,
            generate_token_expiration_time().await,
        )
        .await
}
