use crate::model::faq::{CreateFaqPayload, FaqModel};
use crate::util::response::{FaqResponse, FaqsResponse};
use crate::AppState;
use actix_web::Responder;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;
use validator::Validate;

#[get("/faqs")]
pub async fn fetch_faqs_handler(state: Data<AppState>) -> impl Responder {
    let query_result: Result<Vec<FaqModel>, sqlx::Error> =
        sqlx::query_as!(FaqModel, "SELECT * FROM faqs")
            .fetch_all(&state.db)
            .await;

    match query_result {
        Ok(faqs) => {
            if faqs.is_empty() {
                return HttpResponse::NotFound()
                    .json(serde_json::json!({"status": "ok", "message": "no faqs found"}));
            }
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
pub async fn fetch_faq_handler(state: Data<AppState>, path: Path<uuid::Uuid>) -> impl Responder {
    let fid: Uuid = path.into_inner();
    let query_result: Result<Option<FaqModel>, sqlx::Error> =
        sqlx::query_as!(FaqModel, "SELECT * FROM faqs WHERE id = $1", fid)
            .fetch_optional(&state.db)
            .await;

    match query_result {
        Ok(Some(faq)) => {
            let faq_response = FaqResponse {
                status: "ok".to_string(),
                data: faq,
            };
            HttpResponse::Ok().json(faq_response)
        }
        Ok(None) => {
            // @see https://docs.rs/sqlx-core/0.7.4/sqlx_core/error/enum.Error.html
            // no rows returned by a query that expected to return at least one row
            let message = format!("faq with id: {} not found", fid);
            HttpResponse::NotFound().json(serde_json::json!({"status": "fail", "message": message}))
        }
        Err(err) => HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": err.to_string()})),
    }
}

#[post("/faqs")]
pub async fn create_faq_handler(
    state: Data<AppState>,
    body: Json<CreateFaqPayload>,
) -> impl Responder {
    let is_payload_valid = body.validate();

    match is_payload_valid {
        Ok(_) => {
            let query_result: Result<FaqModel, sqlx::Error> = sqlx::query_as!(
                FaqModel,
                "INSERT INTO faqs (question, answer) VALUES ($1, $2) RETURNING *",
                body.question.clone(),
                body.answer.clone(),
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
) -> impl Responder {
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
                    faq.question.clone_from(&body.question);
                    faq.answer.clone_from(&body.answer);

                    let updated_faq_result: Result<FaqModel, sqlx::Error> = sqlx::query_as!(
                        FaqModel,
                        "UPDATE faqs SET question = $1, answer = $2 WHERE id = $3 RETURNING *",
                        body.question.to_string(),
                        body.answer.to_string(),
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
                            HttpResponse::Ok().json(faq_response)
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

#[delete("/faqs/{id}")]
pub async fn delete_faq_handler(state: Data<AppState>, path: Path<uuid::Uuid>) -> impl Responder {
    let fid: Uuid = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM faqs WHERE id = $1", fid)
        .execute(&state.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("faq with id: {} not found", fid);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }
    return HttpResponse::NoContent().finish();
}
