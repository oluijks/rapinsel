use crate::model::user::{UserModel, UserPayload};
use crate::util::response::{UserResponse, UsersResponse};
use crate::AppState;
use actix_web::Responder;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use validator::Validate;

#[get("/users")]
pub async fn fetch_users_handler(state: Data<AppState>) -> impl Responder {
    let query_result: Result<Vec<UserModel>, sqlx::Error> =
        sqlx::query_as!(UserModel, "SELECT * FROM users")
            .fetch_all(&state.db)
            .await;

    match query_result {
        Ok(users) => {
            if users.is_empty() {
                return HttpResponse::NotFound()
                    .json(serde_json::json!({"status": "ok", "message": "no users found"}));
            }
            let users_response = UsersResponse {
                status: "success".to_string(),
                data: users,
            };
            HttpResponse::Ok().json(users_response)
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn fetch_user_handler(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let fid: i32 = path.into_inner();
    let query_result: Result<Option<UserModel>, sqlx::Error> =
        sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", fid)
            .fetch_optional(&state.db)
            .await;

    match query_result {
        Ok(Some(user)) => {
            let user_response = UserResponse {
                status: "ok".to_string(),
                data: user,
            };
            HttpResponse::Ok().json(user_response)
        }
        Ok(None) => {
            // @see https://docs.rs/sqlx-core/0.7.4/sqlx_core/error/enum.Error.html
            // no rows returned by a query that expected to return at least one row
            let message = format!("user with id: {} not found", fid);
            HttpResponse::NotFound().json(serde_json::json!({"status": "fail", "message": message}))
        }
        Err(err) => HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": err.to_string()})),
    }
}

#[post("/users")]
pub async fn create_user_handler(state: Data<AppState>, body: Json<UserPayload>) -> impl Responder {
    let is_payload_valid = body.validate();

    match is_payload_valid {
        Ok(_) => {
            let query_result: Result<UserModel, sqlx::Error> = sqlx::query_as!(
                UserModel,
                "INSERT INTO users (name, email, encrypted_password) VALUES ($1, $2, $3) RETURNING *",
                body.name.clone(),
                body.email.clone(),
                body.encrypted_password.clone(),
            )
            .fetch_one(&state.db)
            .await;

            match query_result {
                Ok(user) => {
                    let user_response = UserResponse {
                        status: "created".to_string(),
                        data: user,
                    };
                    HttpResponse::Created().json(user_response)
                }
                Err(err) => HttpResponse::InternalServerError()
                    .json(serde_json::json!({"status": "fail", "message": err.to_string()})),
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[put("/users/{id}")]
pub async fn update_user_handler(
    state: Data<AppState>,
    path: Path<i32>,
    body: Json<UserPayload>,
) -> impl Responder {
    let is_payload_valid = body.validate();

    match is_payload_valid {
        Ok(_) => {
            let uid: i32 = path.into_inner();
            let query_result: Result<UserModel, sqlx::Error> =
                sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = $1", uid)
                    .fetch_one(&state.db)
                    .await;

            match query_result {
                Ok(mut user) => {
                    user.name.clone_from(&body.question);
                    user.email.clone_from(&body.answer);

                    let updated_user_result: Result<UserModel, sqlx::Error> = sqlx::query_as!(
                        UserModel,
                        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *",
                        body.name.to_string(),
                        body.email.to_string(),
                        uid
                    )
                    .fetch_one(&state.db)
                    .await;

                    match updated_user_result {
                        Ok(user) => {
                            let user_response = UserResponse {
                                status: "updated".to_string(),
                                data: user,
                            };
                            HttpResponse::Ok().json(user_response)
                        }
                        Err(err) => HttpResponse::InternalServerError().json(
                            serde_json::json!({"status": "fail", "message": err.to_string()}),
                        ),
                    }
                }
                Err(_error) => {
                    let message = format!("user with id: {} not found", uid);
                    HttpResponse::NotFound()
                        .json(serde_json::json!({"status": "fail", "message": message}))
                }
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_handler(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let uid: i32 = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM users WHERE uuid = $1", uid)
        .execute(&state.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("user with id: {} not found", fid);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }
    return HttpResponse::NoContent().finish();
}
