use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable, Channel,
    BasicProperties, Connection, ConnectionProperties,
};

const QUEUE_NAME: &str = "queue_test";

pub async fn init() -> Channel {
    let uri = "amqp://localhost:5672";
    let options = ConnectionProperties::default()
        // Use tokio executor and reactor.
        // At the moment the reactor is only available for unix.
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio);

    let connection = Connection::connect(uri, options).await.unwrap();
    let channel = connection.create_channel().await.unwrap();

    let _queue = channel
        .queue_declare(
            QUEUE_NAME,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await.unwrap();

    channel
}

pub async fn publish_message(channel: &Channel, msg_byte: Vec<u8>) {
    // send to RabbitMQ
    channel.basic_publish(
        "",
        QUEUE_NAME,
        BasicPublishOptions::default(),
        msg_byte.as_slice(),
        BasicProperties::default(),
    ).await.unwrap().await.unwrap();
}