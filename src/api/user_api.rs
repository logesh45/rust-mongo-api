use crate::{models::user_model::{User, CreateUserRequest}, repository::mongodb_repo::MongoRepo};

use actix_web::{
    get, post,
    web::{scope, Data, Path},
    HttpResponse, Scope,
};


#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, request_body: actix_web_validator::Json<CreateUserRequest>) -> HttpResponse {


            let data = User {
                id: None,
                name: request_body.name.to_owned(),
                location: request_body.location.to_owned(),
                title: request_body.title.to_owned(),
            };

            // HttpResponse::Ok().json(data)

            let user_detail = db.create_user(data);
            match user_detail {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }

}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn user_routes() -> Scope {
    scope("/v1")
        .service(create_user)
        .service(get_user)
}
