use crate::domain::dtos::account::Account;

use async_trait::async_trait;
use clean_base::{
    entities::default_response::{CreateResponseKind, GetOrCreateResponseKind},
    utils::errors::MappedErrors,
};
use shaku::Interface;

#[async_trait]
pub trait AccountRegistration: Interface + Send + Sync {
    async fn get_or_create(
        &self,
        account: Account,
    ) -> Result<GetOrCreateResponseKind<Account>, MappedErrors>;

    async fn create(
        &self,
        user: Account,
    ) -> Result<CreateResponseKind<Account>, MappedErrors>;
}
