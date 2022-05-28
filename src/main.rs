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
use crate::api::user_api::{create_user, get_user, user_routes};
use crate::repository::mongodb_repo::MongoRepo;
use crate::services::logger::Logger;

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
            .service(user_routes())
            .service(hello)
    })
        .bind(("localhost", 3000))?
        .run()
        .await
}
