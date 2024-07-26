use crate::model::faq::FaqModel;
use crate::model::user::UserModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct GeneralResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct FaqResponse {
    pub status: String,
    pub data: FaqModel,
}

#[derive(Serialize)]
pub struct FaqsResponse {
    pub status: String,
    pub data: Vec<FaqModel>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserModel,
}

#[derive(Serialize)]
pub struct UsersResponse {
    pub status: String,
    pub data: Vec<UserModel>,
}
