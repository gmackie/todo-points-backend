use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup")
                            .route(web::post().to(account_controller::signup))
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(account_controller::login))
                    )
                    .service(
                        web::resource("/logout")
                            .route(web::post().to(account_controller::logout))
                    )
            )
            .service(
                web::scope("/tasks")
                    .service(
                        web::resource("")
                            .route(web::get().to(task_controller::find_all))
                            .route(web::post().to(task_controller::insert))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(task_controller::find_by_id))
                            .route(web::put().to(task_controller::update))
                            .route(web::delete().to(task_controller::delete))
                    )
                    .service(
                        web::resource("/query/{query}")
                            .route(web::get().to(task_controller::query))   
                    )
            )
    );
}