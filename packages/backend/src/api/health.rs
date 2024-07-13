use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[get("/health-check")]
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "ok".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_health_check_handler() {
        let app = test::init_service(App::new().service(health_check_handler)).await;

        let req = test::TestRequest::get().uri("/health-check").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let expected_json = serde_json::json!({
            "status": "ok",
        });

        assert_eq!(body, serde_json::to_string(&expected_json).unwrap());
    }
}
