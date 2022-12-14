use crate::domain::dtos::user::User;

use async_trait::async_trait;
use clean_base::{
    entities::default_response::UpdatingResponseKind,
    utils::errors::MappedErrors,
};
use shaku::Interface;

#[async_trait]
pub trait UserUpdating: Interface + Send + Sync {
    async fn update(
        &self,
        user: User,
    ) -> Result<UpdatingResponseKind<User>, MappedErrors>;
}
