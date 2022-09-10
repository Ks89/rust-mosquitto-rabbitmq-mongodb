use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Temperature, Topic};
use crate::models::payload_trait::{Humidity, PayloadTrait};
use crate::types::Float;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericMessage {
    pub uuid: String,
    pub profile_token: String,
    pub topic: Topic,
    pub payload: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message<T> where T: PayloadTrait + Sized + Serialize {
    pub uuid: String,
    pub profile_token: String,
    pub topic: Topic,
    pub payload: T,
}

impl<T> Message<T> where T: PayloadTrait + Sized + Serialize {
    pub fn new(uuid: String, profile_token: String, topic: Topic, payload: T) -> Message<T> {
        Self {
            uuid,
            profile_token,
            topic,
            payload,
        }
    }

    pub fn new_as_json(uuid: String, profile_token: String, topic: Topic, payload: T) -> String {
        // println!("Notification deserialized from JSON = {:?}", &val);
        let message = Self::new(
            uuid,
            profile_token,
            topic,
            payload,
        );
        // println!("message {:?}", &queue_message);
        serde_json::to_string(&message).unwrap()
    }
}