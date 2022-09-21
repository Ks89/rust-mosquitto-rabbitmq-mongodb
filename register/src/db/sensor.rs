use crate::models::sensor::Sensor;
use crate::models::sensor::SensorDocument;
use crate::models::sensor::RegisterInput;

use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::FindOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;
use rocket::serde::json::Json;

pub async fn insert_register(
    db: &Database,
    input: Json<RegisterInput>,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("sensors");

    let date_now: DateTime = DateTime::now();

    let insert_one_result = collection
        .insert_one(
            doc! {
                "mac": input.mac.clone(),
                "uuid": input.uuid.clone(),
                "name": input.name.clone(),
                "manufacturer": input.manufacturer.clone(),
                "model": input.model.clone(),
                "profileOwnerId": input.profileOwnerId.clone(),
                "apiToken": input.apiToken.clone(),
                "value": 0,
                "createdAt": date_now,
                "modifiedAt": date_now,
            },
            None,
        ).await?;

    Ok(insert_one_result.inserted_id.as_object_id().unwrap().to_hex())
}