use super::{guest::GuestUserDTO, user::UserDTO};

use agrobase::dtos::enums::{ChildrenEnum, ParentEnum};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTypeDTO {
    pub id: Option<Uuid>,

    pub name: String,
    pub description: String,

    pub is_subscription: bool,
    pub is_manager: bool,
    pub is_staff: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDTO {
    pub id: Option<Uuid>,

    pub name: String,
    pub is_active: bool,
    pub is_checked: bool,
    pub owner: ParentEnum<UserDTO, Uuid>,
    pub account_type: ParentEnum<AccountTypeDTO, Uuid>,
    pub guest_users: Option<ChildrenEnum<GuestUserDTO, Uuid>>,
    pub created: DateTime<Local>,
    pub updated: DateTime<Local>,
}

impl AccountDTO {
    pub fn build_owner_url(&self, base_url: String) -> Result<String, ()> {
        match self.owner.to_owned() {
            ParentEnum::Id(id) => Ok(format!("{:?}/{:?}", base_url, id)),
            ParentEnum::Record(record) => match record.id {
                None => Ok(base_url),
                Some(id) => Ok(format!("{}/{}", base_url, id.to_string())),
            },
        }
    }

    pub fn build_account_type_url(
        &self,
        base_url: String,
    ) -> Result<String, ()> {
        match self.account_type.to_owned() {
            ParentEnum::Id(id) => Ok(format!("{:?}/{:?}", base_url, id)),
            ParentEnum::Record(record) => match record.id {
                None => Ok(base_url),
                Some(id) => Ok(format!("{}/{}", base_url, id.to_string())),
            },
        }
    }

    pub fn build_guest_users_url(
        &self,
        base_url: String,
    ) -> Result<Vec<String>, ()> {
        match self.guest_users.to_owned() {
            None => Err(()),
            Some(records) => match records {
                ChildrenEnum::Ids(ids) => Ok(ids
                    .iter()
                    .map(|id| format!("{}/{}", base_url, id))
                    .collect()),
                ChildrenEnum::Records(records) => {
                    let urls = records
                        .iter()
                        .filter_map(|record| match record.id {
                            None => Some(base_url.to_owned()),
                            Some(_) => Some(format!(
                                "{}/{}",
                                base_url,
                                record.id.unwrap()
                            )),
                        })
                        .collect();

                    Ok(urls)
                }
            },
        }
    }
}

// ? --------------------------------------------------------------------------
// ? TESTS
// ? --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use chrono::Local;

    use super::*;
    use crate::domain::dtos::email::EmailDTO;

    #[test]
    fn test_if_account_works() {
        let base_url = "http://local.host/api/v1/accounts".to_string();

        let account_type = AccountTypeDTO {
            id: None,
            name: "".to_string(),
            description: "".to_string(),
            is_subscription: false,
            is_manager: false,
            is_staff: false,
        };

        let user = UserDTO {
            id: None,
            username: "username".to_string(),
            email: EmailDTO::from_string("username@email.domain".to_string())
                .unwrap(),
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
            created: Local::now(),
            updated: Local::now(),
        };

        let account = AccountDTO {
            id: None,
            name: String::from("Account Name"),
            is_active: true,
            is_checked: false,
            owner: ParentEnum::Record(user),
            account_type: ParentEnum::Record(account_type),
            guest_users: None,
            created: Local::now(),
            updated: Local::now(),
        };

        println!("{:?}", account.build_account_type_url(base_url.to_owned()));

        assert_eq!(
            account.build_account_type_url(base_url.to_owned()).is_ok(),
            true
        );

        assert_eq!(
            account.build_account_type_url(base_url.to_owned()).unwrap(),
            base_url.to_owned()
        );
    }
}
