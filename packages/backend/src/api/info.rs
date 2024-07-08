use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
}

const API_NAME: &str = "rapinsel";
const API_VERSION: &str = "v1";

#[get("/api-info")]
pub async fn api_info_handler() -> impl Responder {
    HttpResponse::Ok().json(Info {
        name: API_NAME.to_string(),
        version: API_VERSION.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_api_info_handler() {
        let app = test::init_service(App::new().service(api_info_handler)).await;

        let req = test::TestRequest::get().uri("/api-info").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let expected_json = serde_json::json!({
          "name": API_NAME.to_string(),
          "version": API_VERSION.to_string(),
        });

        assert_eq!(body, serde_json::to_string(&expected_json).unwrap());
    }
}
