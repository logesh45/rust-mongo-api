use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use validator::{Validate};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub location: String,
    pub title: String,
}

#[derive(Validate, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, message = "name should be at least 3 characters"))]
    pub name: String,
    pub location: String,
    #[validate(length(min = 3, message = "title should be at least 2 characters"))]
    pub title: String,
}
