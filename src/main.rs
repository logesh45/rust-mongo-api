mod api;
mod models;
mod repository;
mod services;

use actix_web::{
    get,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    web::Data,
};
use crate::api::user_api::user_routes;
use crate::repository::mongodb_repo::MongoRepo;
use crate::services::logger::Logger;
use crate::models::error_model::ErrorResponse;

#[macro_use]
extern crate log;
extern crate simplelog;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::init();
    // error!("Bright red error");
    // debug!("This level is currently not enabled for any logger");

    info!("Connecting to MongoDB");
    let db = MongoRepo::init();
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(
                configure_query_config(
                    actix_web_validator::JsonConfig::default()
                )
            )
            .service(user_routes())
            .service(hello)
    })
        .bind(("localhost", 3000))?
        .run()
        .await
}

/// register error handler to automate validation error user-friendly message
pub fn configure_query_config(cfg: actix_web_validator::JsonConfig) -> actix_web_validator::JsonConfig {
    cfg.error_handler(|err, _| {

        // info!("Errors {:?}", err);

        let error_response = ErrorResponse {
            success: false,
            message: format_error(&err),
        };


        actix_web::error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(error_response),
        )
            .into()
    })
}


/// handle different errors based on type
///
/// see also:  [`format_validations`]
pub fn format_error(error: &actix_web_validator::Error) -> String {
    match error {
        actix_web_validator::Error::Validate(validations) => format_validations(validations),
        _ => String::new(),
    }
}

/// format query validation errors
///
/// see also:  [`format_field_errors`]
pub fn format_validations(validations: &validator::ValidationErrors) -> String {
    let errors = validations.clone().into_errors();
    let mut messages = vec![];

    info!("Errors {:?}", errors);

    for (key, value) in errors {
        info!("key {:?}:, {:?}", key, value);
        let optional = match value {
            validator::ValidationErrorsKind::Struct(details) => Some(format_validations(&*details)),
            validator::ValidationErrorsKind::Field(details) => Some(format_field_errors(key, details)),
            _ => None
        };
        if optional.is_some() {
            messages.push(optional.unwrap());
        }
    }
    messages.join("\n")
}

/// format query validation errors
///
/// turn a [`Vec<validator::ValidationError>`] into a formatted string
pub fn format_field_errors(key: &'static str, details: Vec<validator::ValidationError>) -> String {
    let mut message = String::new();

    info!("details type {:?}:, {:?}", key, details);

    for detail in details {
        info!("detail type {:?}:, {:?}", key, detail);

        if detail.code == "invalid" {
            message = format!(
                "'{}' is invalid for field {} (possible values : {})",
                detail
                    .params
                    .get("provided")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                key,
                detail
                    .params
                    .get("allowed")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string()
            );
        } else {
            message = detail.message.unwrap().into()
        }
    }
    message
}
