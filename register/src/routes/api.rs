use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::{Status};

use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc};
use mongodb::Database;

use crate::errors::api_error::{ApiResponse, ApiError};
use crate::models::sensor::RegisterInput;
use crate::db::sensor;

/// register a new sensor device
#[post("/register", data = "<input>")]
pub async fn post_register(db: &State<Database>, input: Json<RegisterInput>) -> ApiResponse {
    // can set with a single error like this.
    match sensor::insert_register(&db, input).await {
        Ok(_register_doc_id) => {
            ApiResponse {
                json: json!({ "id": _register_doc_id }),
                status: Status::Ok,
            }
        }
        Err(_error) => {
            println!("{:?}", _error);
            ApiResponse {
                json: serde_json::to_value(ApiError { code: 0, message: "Invalid input".to_string() }).unwrap(),
                status: Status::BadRequest,
            }
        }
    }
}