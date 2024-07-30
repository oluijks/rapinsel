#![deny(clippy::all)]

mod api;
mod config;
mod model;
mod scope;
mod util;

use actix_web::{http::header, web::scope, App, HttpServer};
use api::{health::health_check_handler, info::api_info_handler};

#[derive(Debug, Clone)]
pub struct AppState {
    db: sqlx::Pool<sqlx::Postgres>,
}

const API_SCOPE: &str = "/api/";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // dotenv::dotenv().ok();
    dotenvy::dotenv().ok();
    env_logger::init();

    let config: config::Config = config::Config::init();

    let scoped_api_version: String = API_SCOPE.to_owned() + &config.version;

    // region:  database connection pool
    let pool: sqlx::Pool<sqlx::Postgres> = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.pg_conn_str)
        .await
    {
        Ok(pool) => {
            println!("[pdb] pool created");
            pool
        }
        Err(err) => {
            println!("[pdb] failed to create pool: {:?}", err);
            std::process::exit(1);
        }
    };
    // endregion:  database connection pool

    println!(
        "{}",
        format_args!("[api] listening on {}:{}", config.host, config.port)
    );

    HttpServer::new(move || {
        // region:  CORS
        let cors = actix_cors::Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
            ])
            .allowed_origin("https://rapinsel.io")
            .allowed_origin("http://localhost:4173")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://192.168.1.170:5173")
            .supports_credentials();
        // endregion:  CORS

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(actix_web::web::Data::new(AppState { db: pool.clone() }))
            .service(
                scope(&scoped_api_version.to_owned())
                    .configure(scope::faqs::scoped_faqs_config)
                    .service(api_info_handler)
                    .service(health_check_handler),
            )
    })
    .bind(format!("{}:{}", config.host, config.port))
    .expect("address should be free and valid")
    .workers(num_cpus::get())
    .run()
    .await?;

    Ok(())
}
