mod api;
mod models;
mod repository;

use actix_web::{
    get,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    web::Data,
};
use crate::api::user_api::{create_user, get_user};
use crate::repository::mongodb_repo::MongoRepo;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use std::fs::File;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create("logs/app.log").unwrap()
            ),
            WriteLogger::new(
                LevelFilter::Debug,
                Config::default(),
                File::create("logs/debug.log").unwrap()
            ),
        ]
    ).unwrap();

    // error!("Bright red error");
    // debug!("This level is currently not enabled for any logger");

    info!("Connecting to MongoDB");
    let db = MongoRepo::init();
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(hello)
    })
        .bind(("localhost", 3000))?
        .run()
        .await
}
