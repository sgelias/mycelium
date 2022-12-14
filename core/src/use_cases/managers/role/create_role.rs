use crate::domain::{
    dtos::{profile::Profile, role::Role},
    entities::RoleRegistration,
};

use clean_base::{
    entities::default_response::GetOrCreateResponseKind,
    utils::errors::{use_case_err, MappedErrors},
};

/// Create a single role.
///
/// This function should be executed before the Guest Roles creation. Role
/// examples should be: ResultsExpert, CustomerExpert, Staff.
pub async fn create_role(
    profile: Profile,
    name: String,
    description: String,
    role_registration_repo: Box<&dyn RoleRegistration>,
) -> Result<GetOrCreateResponseKind<Role>, MappedErrors> {
    // ? ----------------------------------------------------------------------
    // ? Check if the current account has sufficient privileges to create role
    // ? ----------------------------------------------------------------------

    if !profile.is_manager {
        return Err(use_case_err(
            "The current user has no sufficient privileges to register new 
            role."
                .to_string(),
            Some(true),
            None,
        ));
    }

    // ? ----------------------------------------------------------------------
    // ? Persist Role
    // ? ----------------------------------------------------------------------

    role_registration_repo
        .get_or_create(Role {
            id: None,
            name,
            description,
        })
        .await
}
