use futures_lite::StreamExt;
use lapin::{Consumer};

mod models;
mod amqp;

use crate::models::message::{GenericMessage, Message};
use crate::models::payload_trait::{Humidity, Temperature};
use crate::models::topic::Topic;
use crate::amqp::{init, read_message};
use crate::models::{new_humidity_message, new_temperature_message};

#[tokio::main]
async fn main() {
    println!("starting up");

    async_global_executor::block_on(async {
        let mut consumer: Consumer = init().await;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let payload_str: &str = read_message(&delivery).await;

                // deserialize to a GenericMessage (with turbofish operator "::<GenericMessage>")
                match serde_json::from_str::<GenericMessage>(payload_str) {
                    Ok(generic_msg) => {
                        println!("GenericMessage deserialized from JSON = {:?}", generic_msg);

                        if generic_msg.payload.get("temperature").is_some() {
                            let message: Message<Temperature> = new_temperature_message(generic_msg);
                            println!("message temperature {:?}", message);
                        } else if generic_msg.payload.get("humidity").is_some() {
                            let message: Message<Humidity> = new_humidity_message(generic_msg);
                            println!("message humidity {:?}", message);
                        } else {
                            eprintln!("Cannot recognize Message payload type");
                        }
                    }
                    Err(err) => {
                        eprintln!("Cannot convert payload as json Message. Error = {:?}", err);
                    }
                }
            }
        }
    })
}