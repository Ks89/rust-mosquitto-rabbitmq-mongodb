use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::topic::{Topic};
use crate::models::payload_trait::{PayloadTrait};

// input message from RabbitMQ
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericMessage {
    pub uuid: String,
    pub profile_token: String,
    pub topic: Topic,
    // payload is variable, because it can be PayloadTrait (Temperature, Humidity...)
    // so I need to parse something that cannot be expressed with a fixed struct
    pub payload: Value,
}

// Message processed using GenericMessage as input
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
}