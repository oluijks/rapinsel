use crate::model::faq::{CreateFaqPayload, FaqModel};
use crate::util::response::{FaqResponse, FaqsResponse};
use crate::AppState;
use actix_web::body::BoxBody;
use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

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

#[post("/faqs")]
pub async fn create_faq_handler(
    state: Data<AppState>,
    body: Json<CreateFaqPayload>,
) -> HttpResponse<BoxBody> {
    let is_payload_valid = body.validate();

    match is_payload_valid {
        Ok(_) => {
            let query_result: Result<FaqModel, sqlx::Error> = sqlx::query_as!(
                FaqModel,
                r#"INSERT INTO faqs (question, answer) VALUES ($1, $2) RETURNING *"#,
                body.question.to_string(),
                body.answer.to_string(),
            )
            .fetch_one(&state.db)
            .await;

            match query_result {
                Ok(faq) => {
                    let faq_response = FaqResponse {
                        status: "created".to_string(),
                        data: faq,
                    };
                    HttpResponse::Created().json(faq_response)
                }
                Err(err) => HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "fail", "message": err.to_string()})),
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[put("/faqs/{id}")]
pub async fn update_faq_handler(
    state: Data<AppState>,
    path: Path<uuid::Uuid>,
    body: Json<CreateFaqPayload>,
) -> HttpResponse<BoxBody> {
    let is_payload_valid = body.validate();

    match is_payload_valid {
        Ok(_) => {
            let fid: Uuid = path.into_inner();
            let query_result: Result<FaqModel, sqlx::Error> =
                sqlx::query_as!(FaqModel, "SELECT * FROM faqs WHERE id = $1", fid)
                    .fetch_one(&state.db)
                    .await;

            match query_result {
                Ok(mut faq) => {
                    if !body.question.is_empty() {
                        faq.question = body.question.clone();
                    }
                    if !body.answer.is_empty() {
                        faq.answer = body.answer.clone();
                    }

                    let updated_faq_result: Result<FaqModel, sqlx::Error> = sqlx::query_as!(
                        FaqModel,
                        r#"UPDATE faqs SET question = $1, answer = $2 WHERE id = $3 RETURNING *"#,
                        faq.question.to_string(),
                        faq.answer.to_string(),
                        fid
                    )
                    .fetch_one(&state.db)
                    .await;

                    match updated_faq_result {
                        Ok(faq) => {
                            let faq_response = FaqResponse {
                                status: "updated".to_string(),
                                data: faq,
                            };
                            HttpResponse::Created().json(faq_response)
                        }
                        Err(err) => HttpResponse::InternalServerError().json(
                            serde_json::json!({"status": "fail", "message": err.to_string()}),
                        ),
                    }
                }
                Err(_error) => {
                    let message = format!("faq with id: {} not found", fid);
                    HttpResponse::NotFound()
                        .json(serde_json::json!({"status": "fail", "message": message}))
                }
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
