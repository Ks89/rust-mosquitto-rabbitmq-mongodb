mod models;

use futures_lite::StreamExt;

use lapin::{
    types,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use serde_json::Value;
use crate::models::message::{GenericMessage, Message};
use crate::models::payload_trait::{Humidity, Temperature};
use crate::models::topic::Topic;

const QUEUE_NAME: &str = "queue_test";

#[tokio::main]
async fn main() {
    println!("starting up");

    let uri = "amqp://localhost:5672";
    let options = ConnectionProperties::default()
        // Use tokio executor and reactor.
        // At the moment the reactor is only available for unix.
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    async_global_executor::block_on(async {
        let connection = Connection::connect(uri, options).await.unwrap();
        println!("CONNECTED");

        let channel = connection.create_channel().await.unwrap();
        println!("conn status state: {:?}", connection.status().state());

        let _queue = channel
            .queue_declare(
                QUEUE_NAME,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        println!("Declared queue {:?}", _queue);

        let mut consumer = channel
            .basic_consume(
                QUEUE_NAME,
                "tag_foo",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        println!("conn status state: {:?}", connection.status().state());

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");

                let payload_str = match std::str::from_utf8(&delivery.data) {
                    Ok(res) => {
                        println!("payload_str: {}", res);
                        res
                    }
                    Err(err) => {
                        eprintln!("cannot read payload as utf8. Error = {}", err);
                        ""
                    }
                };

                // deserialize to a Notification (with turbofish operator "::<Notification>")
                match serde_json::from_str::<GenericMessage>(payload_str) {
                    Ok(val) => {
                        println!("Notification deserialized from JSON = {:?}", val);

                        if val.payload.get("temperature").is_some() {
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
                            println!("message temperature {:?}", message);
                        } else if val.payload.get("humidity").is_some() {
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
                            println!("message humidity {:?}", message);
                        }
                    }
                    Err(err) => {
                        eprintln!("Cannot convert payload as json Notification. Error = {}", err);
                    }
                }
            }
        }
    })
}