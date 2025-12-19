use serde::{Deserialize, Serialize};

use crate::services::requests::SERVER_ADDRESS;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub token: String,
    pub name: String,
}

impl UserInfo {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

impl From<LoginInfo> for LoginInfoWrapper {
    fn from(user: LoginInfo) -> Self {
        Self { user }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum ConfirmationType {
    Registration,
    PasswordReset,
}

#[derive(Serialize, Debug)]
pub struct SendConfirmationLink {
    pub email: String,
    pub confirmation_type: ConfirmationType,
    pub server_address: String,
}

impl SendConfirmationLink {
    pub fn new(email: String, confirmation_type: ConfirmationType) -> Self {
        Self {
            email,
            confirmation_type,
            server_address: SERVER_ADDRESS.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SendConfirmationLinkWrapper {
    pub data: SendConfirmationLink,
}

impl From<SendConfirmationLink> for SendConfirmationLinkWrapper {
    fn from(data: SendConfirmationLink) -> Self {
        Self { data }
    }
}
