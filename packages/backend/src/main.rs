#![deny(clippy::all)]

mod api;
mod config;
mod model;
mod scopes;
mod util;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::scope, web::Data, App, HttpServer};
use api::{health::health_check_handler, info::api_info_handler};
use config::Config;
use dotenv::dotenv;
use scopes::faqs::scoped_faqs_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

const API_SCOPE: &str = "/api/";

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let config: Config = Config::init();

    let scoped_api_version: String = API_SCOPE.to_owned() + &config.version;
    let pool: Pool<Postgres> = match PgPoolOptions::new()
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

    println!(
        "{}",
        format_args!("[api] server is running on {}:{}", config.host, config.port)
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
            ])
            .allowed_origin("https://rapinsel.io")
            .allowed_origin("http://localhost:5173")
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(
                scope(&scoped_api_version.to_owned())
                    .configure(scoped_faqs_config)
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
