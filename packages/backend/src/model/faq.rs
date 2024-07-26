use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FaqModel {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

// @see https://github.com/Keats/validator
// @see https://dev.to/chaudharypraveen98/form-validation-in-rust-404l
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct FaqPayload {
    #[validate(length(
        min = 3,
        max = 255,
        message = "question must be between 3 and 255 characters"
    ))]
    pub question: String,
    #[validate(length(min = 3, message = "answer must be at least 3 characters"))]
    pub answer: String,
}
