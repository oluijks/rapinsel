use crate::model::faq::FaqModel;
use serde::Serialize;

#[derive(Serialize)]
pub struct GeneralResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct FaqResponse {
    pub status: String,
    pub faq: FaqModel,
}

#[derive(Serialize)]
pub struct FaqsResponse {
    pub status: String,
    pub faqs: Vec<FaqModel>,
}
