use crate::model::faq::FaqModel;
use crate::util::response::{FaqResponse, FaqsResponse};
use crate::AppState;
use actix_web::body::BoxBody;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use uuid::Uuid;

#[get("/faqs")]
pub async fn fetch_faqs_handler(state: Data<AppState>) -> HttpResponse<BoxBody> {
    let query_result: Result<Vec<FaqModel>, sqlx::Error> =
        sqlx::query_as!(FaqModel, "SELECT * FROM faqs")
            .fetch_all(&state.db)
            .await;

    match query_result {
        Ok(faqs) => {
            let faqs_response = FaqsResponse {
                status: "success".to_string(),
                data: faqs,
            };
            HttpResponse::Ok().json(faqs_response)
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/faqs/{id}")]
pub async fn fetch_faq_handler(
    state: Data<AppState>,
    path: Path<uuid::Uuid>,
) -> HttpResponse<BoxBody> {
    let fid: Uuid = path.into_inner();
    let query_result: Result<FaqModel, sqlx::Error> =
        sqlx::query_as!(FaqModel, "SELECT * FROM faqs WHERE id = $1", fid)
            .fetch_one(&state.db)
            .await;

    match query_result {
        Ok(faq) => {
            let faq_response = FaqResponse {
                status: "ok".to_string(),
                data: faq,
            };
            HttpResponse::Ok().json(faq_response)
        }
        Err(_error) => {
            // @see https://docs.rs/sqlx-core/0.7.4/sqlx_core/error/enum.Error.html
            // no rows returned by a query that expected to return at least one row
            let message = format!("faq with id: {} not found", fid);
            HttpResponse::NotFound().json(serde_json::json!({"status": "fail", "message": message}))
        }
    }
}
