use crate::models::sensor::Sensor;
use crate::models::sensor::SensorDocument;

use mongodb::bson::{doc, DateTime};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};
use serde::Serialize;
use crate::models::message::Message;
use crate::models::payload_trait::{Humidity, Light, PayloadTrait, Temperature};

pub async fn update_message<T: PayloadTrait + Sized + Serialize>(
    db: &Database,
    message: Message<T>,
) -> mongodb::error::Result<Option<Sensor>> {
    let collection = db.collection::<SensorDocument>("sensors");

    let find_one_and_update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let sensor_doc = update_sensor_by_message(
        &collection,
        find_one_and_update_options,
        message.uuid,
        message.profile_token,
        message.payload.get_value(),
    ).await;

    if sensor_doc.is_none() {
        println!("none!!!!!");
        return Ok(None);
    }

    Ok(Some(document_to_json(&sensor_doc.unwrap())))
}

async fn update_sensor_by_message(
    collection: &Collection<SensorDocument>,
    find_one_and_update_options: FindOneAndUpdateOptions,
    uuid: String,
    profile_token: String,
    value: f64,
) -> Option<SensorDocument> {
    println!("uuid {}", uuid);
    println!("profile_token {}", profile_token);
    println!("value {}", value);

    let sensor_doc = collection
        .find_one_and_update(
            doc! { "uuid": uuid, "profileOwnerId": profile_token },
            doc! { "$set": {
                "value": value,
                "modifiedAt": DateTime::now()}},
            find_one_and_update_options,
        )
        .await.unwrap();
    sensor_doc
}


fn document_to_json(sensor_doc: &SensorDocument) -> Sensor {
    Sensor {
        _id: sensor_doc._id.to_string(),
        uuid: sensor_doc.uuid.to_string(),
        mac: sensor_doc.mac.to_string(),
        name: sensor_doc.name.to_string(),
        manufacturer: sensor_doc.manufacturer.to_string(),
        model: sensor_doc.model.to_string(),
        profileOwnerId: sensor_doc.profileOwnerId.to_string(),
        apiToken: sensor_doc.apiToken.to_string(),
        createdAt: sensor_doc.createdAt.to_string(),
        modifiedAt: sensor_doc.modifiedAt.to_string(),
        value: sensor_doc.value,
    }
}