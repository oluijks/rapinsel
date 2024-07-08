use crate::model::faq::FaqModel;
use crate::AppState;
use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[get("/faqs")]
pub async fn fetch_faqs_handler(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(FaqModel, "SELECT * FROM faqs")
        .fetch_all(&state.db)
        .await
    {
        Ok(faqs) => {
            let faqs_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "faqs": faqs
                })
            });
            return HttpResponse::Ok().json(faqs_response);
        }
        Err(_) => {
            let message = format!("no faqs found");
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "ok", "message": message}));
        }
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
            let faq_response = serde_json::json!({
                "status": "ok",
                "data": serde_json::json!({
                "faq": faq
            })});
            return HttpResponse::Ok().json(faq_response);
        }
        Ok(None) => {
            let message = format!("faq with id: {} not found", fid);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "ok", "message": message}));
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
