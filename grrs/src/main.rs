use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer, StreamConsumer};
use rdkafka::Message;

const broker = "localhost:9092";
const topic = "my_topic";
const group_id = "my_group_id";

const consumer: BaseConsumer = ClientConfig::new()
    .set("bootstrap.servers", broker)
    .set("group.id", group_id)
    .create()
    .expect("Consumer creation error");

consumer.subscribe(&[topic]).expect("Can't subscribe to topic");

loop {
    match consumer.poll(Duration::from_secs(1)) {
        Some(Ok(message)) => {
            let payload = message.payload().unwrap();
            let command = std::str::from_utf8(payload).unwrap();
            println!("Received command: {}", command);
        },
        Some(Err(e)) => {
            eprintln!("Error while receiving message: {:?}", e);
        },
        None => {
        }
    }
}
