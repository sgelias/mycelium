use crate::domain::dtos::token::Token;

use async_trait::async_trait;
use chrono::{DateTime, Local};
use clean_base::{
    entities::default_response::CreateResponseKind, utils::errors::MappedErrors,
};
use shaku::Interface;

#[async_trait]
pub trait TokenRegistration: Interface + Send + Sync {
    async fn create(
        &self,
        token: Token,
        expires: DateTime<Local>,
    ) -> Result<CreateResponseKind<Token>, MappedErrors>;
}
