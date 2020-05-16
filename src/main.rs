#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use std::{env, io};

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;

mod tasks;
use self::tasks::api as task_api;
mod users;
use self::users::api as user_api;
mod errors;
mod db;
mod api;
mod schema;
mod utils;

static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "escape-room-backend=debug,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");
    let domain: String =
        std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());


    let app = move || {
        debug!("Constructing the App");

        let session_store = CookieSession::signed(SESSION_SIGNING_KEY).secure(false);

        let error_handlers = ErrorHandlers::new()
            .handler(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                api::internal_server_error,
            )
            .handler(http::StatusCode::BAD_REQUEST, api::bad_request)
            .handler(http::StatusCode::NOT_FOUND, api::not_found);

        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false),
            ))
            .data(web::JsonConfig::default().limit(4096))
            .wrap(session_store)
            .wrap(error_handlers)
            .service(
                web::scope("/api")
                    .service(web::resource("/").route(web::get().to(task_api::index)))
                    .service(web::resource("/users").route(web::get().to(user_api::index)))
                    .service(web::resource("/users").route(web::post().to(user_api::create)))
                    .service(web::resource("/users/{id}").route(web::post().to(user_api::update)))
                    .service(web::resource("/todos").route(web::get().to(task_api::index)))
                    .service(web::resource("/todos").route(web::post().to(task_api::create)))
                    .service(web::resource("/todos/{id}").route(web::post().to(task_api::update)))
            )
            .service(fs::Files::new("/static", "static/"))
    };

    debug!("Starting server");
    HttpServer::new(app).bind("localhost:8080")?.run().await
}
