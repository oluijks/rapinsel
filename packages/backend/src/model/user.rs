use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub encrypted_password: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "deletedAt")]
    pub deleted_at: Option<DateTime<Utc>>,
}

// @see https://github.com/Keats/validator
// @see https://dev.to/chaudharypraveen98/form-validation-in-rust-404l
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserPayload {
    #[validate(length(
        min = 3,
        max = 255,
        message = "name must be between 3 and 255 characters"
    ))]
    pub name: String,
    #[validate(email(message = "email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters"))]
    pub password: String,
}
