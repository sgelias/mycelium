use crate::{
    prisma::{
        account as account_model, account_type as account_type_model,
        user as user_model,
    },
    repositories::connector::get_client,
};

use async_trait::async_trait;
use chrono::Local;
use clean_base::{
    dtos::enums::ParentEnum,
    entities::default_response::{CreateResponseKind, GetOrCreateResponseKind},
    utils::errors::{creation_err, MappedErrors},
};
use myc_core::domain::{
    dtos::{
        account::{Account, AccountType},
        email::Email,
        user::User,
    },
    entities::AccountRegistration,
};
use shaku::Component;
use std::process::id as process_id;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = AccountRegistration)]
pub struct AccountRegistrationSqlDbRepository {}

#[async_trait]
impl AccountRegistration for AccountRegistrationSqlDbRepository {
    async fn get_or_create(
        &self,
        account: Account,
    ) -> Result<GetOrCreateResponseKind<Account>, MappedErrors> {
        // ? -------------------------------------------------------------------
        // ? Try to build the prisma client
        // ? -------------------------------------------------------------------

        let tmp_client = get_client().await;

        let client = match tmp_client.get(&process_id()) {
            None => {
                return Err(creation_err(
                    String::from(
                        "Prisma Client error. Could not fetch client.",
                    ),
                    Some(false),
                    None,
                ))
            }
            Some(res) => res,
        };

        // ? -------------------------------------------------------------------
        // ? Build the initial query (get part of the get-or-create)
        // ? -------------------------------------------------------------------

        let response = client
            .account()
            .find_first(vec![
                account_model::name::equals(account.name.to_owned()),
                account_model::owner::is(vec![user_model::email::equals(
                    match account.owner.to_owned() {
                        ParentEnum::Id(_) => {
                            return Err(creation_err(
                                String::from("Could not create account. User e-mail invalid."),
                                Some(true),
                                None,
                            ))
                        }
                        ParentEnum::Record(record) => {
                            record.email.get_email().to_owned()
                        }
                    },
                )]),
            ])
            .include(account_model::include!({ owner account_type }))
            .exec()
            .await;

        match response.unwrap() {
            Some(record) => {
                return Ok(GetOrCreateResponseKind::NotCreated(
                    Account {
                        id: Some(Uuid::parse_str(&record.id).unwrap()),
                        name: record.name,
                        is_active: record.is_active,
                        is_checked: record.is_checked,
                        owner: ParentEnum::Record(User {
                            id: Some(
                                Uuid::parse_str(&record.owner.id).unwrap(),
                            ),
                            username: record.owner.username,
                            email: match Email::from_string(record.owner.email)
                            {
                                Err(err) => return Err(err),
                                Ok(res) => res,
                            },
                            first_name: Some(record.owner.first_name),
                            last_name: Some(record.owner.last_name),
                            is_active: record.owner.is_active,
                            created: record.owner.created.into(),
                            updated: match record.owner.updated {
                                None => None,
                                Some(date) => Some(date.with_timezone(&Local)),
                            },
                        }),
                        account_type: ParentEnum::Record(AccountType {
                            id: Some(
                                Uuid::parse_str(&record.account_type.id)
                                    .unwrap(),
                            ),
                            name: record.account_type.name,
                            description: record.account_type.description,
                            is_subscription: record
                                .account_type
                                .is_subscription,
                            is_manager: record.account_type.is_manager,
                            is_staff: record.account_type.is_staff,
                        }),
                        guest_users: None,
                        created: record.created.into(),
                        updated: match record.updated {
                            None => None,
                            Some(date) => Some(date.with_timezone(&Local)),
                        },
                    },
                    "Account already exists".to_string(),
                ));
            }
            None => (),
        };

        // ? -------------------------------------------------------------------
        // ? Build create part of the get-or-create
        // ? -------------------------------------------------------------------

        let response = client
            .account()
            .create(
                account.name,
                user_model::id::equals(match account.owner {
                    ParentEnum::Id(_) => return Err(creation_err(
                        String::from(
                            "Could not create account. Invalid owner.",
                        ),
                        Some(true),
                        None,
                    )),
                    ParentEnum::Record(record) => match record.id {
                        None => return Err(creation_err(
                            String::from(
                                "Could not create account. User e-mail invalid.",
                            ),
                            Some(true),
                            None,
                        )),
                        Some(res) => res.to_string(),
                    }
                }),
                account_type_model::id::equals(match account.account_type {
                    ParentEnum::Id(_) => return Err(creation_err(
                        String::from(
                            "Could not create account. Invalid account type.",
                        ),
                        Some(true),
                        None,
                    )),
                    ParentEnum::Record(record) => match record.id {
                        None => return Err(creation_err(
                            String::from(
                                "Could not create account. Invalid account type.",
                            ),
                            Some(true),
                            None,
                        )),
                        Some(res) => res.to_string(),
                    }
                }),
                vec![],
            )
            .include(account_model::include!({ owner account_type }))
            .exec()
            .await;

        match response {
            Ok(record) => Ok(GetOrCreateResponseKind::Created(Account {
                id: Some(Uuid::parse_str(&record.id).unwrap()),
                name: record.name,
                is_active: record.is_active,
                is_checked: record.is_checked,
                owner: ParentEnum::Record(User {
                    id: Some(Uuid::parse_str(&record.owner.id).unwrap()),
                    username: record.owner.username,
                    email: match Email::from_string(record.owner.email) {
                        Err(err) => return Err(err),
                        Ok(res) => res,
                    },
                    first_name: Some(record.owner.first_name),
                    last_name: Some(record.owner.last_name),
                    is_active: record.owner.is_active,
                    created: record.owner.created.into(),
                    updated: match record.owner.updated {
                        None => None,
                        Some(date) => Some(date.with_timezone(&Local)),
                    },
                }),
                account_type: ParentEnum::Record(AccountType {
                    id: Some(Uuid::parse_str(&record.account_type.id).unwrap()),
                    name: record.account_type.name,
                    description: record.account_type.description,
                    is_subscription: record.account_type.is_subscription,
                    is_manager: record.account_type.is_manager,
                    is_staff: record.account_type.is_staff,
                }),
                guest_users: None,
                created: record.created.into(),
                updated: match record.updated {
                    None => None,
                    Some(date) => Some(date.with_timezone(&Local)),
                },
            })),
            Err(err) => {
                return Err(creation_err(
                    format!(
                        "Unexpected error detected on update record: {}",
                        err
                    ),
                    Some(false),
                    None,
                ));
            }
        }
    }

    // ? -----------------------------------------------------------------------
    // ! NOT IMPLEMENTED METHODS
    // ? -----------------------------------------------------------------------

    async fn create(
        &self,
        _: Account,
    ) -> Result<CreateResponseKind<Account>, MappedErrors> {
        panic!("Not implemented method `create`.")
    }
}
