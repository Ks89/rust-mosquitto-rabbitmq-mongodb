pub mod message;
pub mod topic;
pub mod payload_trait;

use crate::{GenericMessage, Message};
use crate::models::payload_trait::{Humidity, Temperature};

// this fn is very bad -> find a generic way to process all T in a single fn
pub fn new_temperature_message(val: GenericMessage) -> Message<Temperature> {
    let temperature: Option<f64> = val.payload
        .get("temperature")
        .and_then(|value| value.as_f64());

    let payload = Temperature {
        temperature: temperature.unwrap()
    };
    let message: Message<Temperature> = Message::new(
        val.uuid,
        val.profile_token,
        val.topic,
        payload,
    );
    message
}

// this fn is very bad -> find a generic way to process all T in a single fn
pub fn new_humidity_message(val: GenericMessage) -> Message<Humidity> {
    let humidity: Option<f64> = val.payload
        .get("humidity")
        .and_then(|value| value.as_f64());

    let payload = Humidity {
        humidity: humidity.unwrap()
    };
    let message: Message<Humidity> = Message::new(
        val.uuid,
        val.profile_token,
        val.topic,
        payload,
    );
    message
}