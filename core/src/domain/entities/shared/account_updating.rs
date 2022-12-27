use crate::domain::dtos::account::AccountDTO;

use async_trait::async_trait;
use clean_base::{
    entities::default_response::UpdatingResponseKind,
    utils::errors::MappedErrors,
};
use shaku::Interface;

#[async_trait]
pub trait AccountUpdating: Interface + Send + Sync {
    async fn update(
        &self,
        account: AccountDTO,
    ) -> Result<UpdatingResponseKind<AccountDTO>, MappedErrors>;
}