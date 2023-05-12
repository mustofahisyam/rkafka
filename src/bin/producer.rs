use std::io;
use std::time::Duration;
use std::env;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};


fn main() {
    let broker = env::var("BROKER_SERVER").expect("Error: VAR_NAME not found");   

    println!("Please input your topic:");
    
    let mut topic = String::new();
    
    io::stdin()
        .read_line(&mut topic)
        .expect("Failed to read topic input");

    let topic: String = match topic.trim().parse() {
        Ok(t) => t,
        Err(_) => panic!("Invalid topic input"),
    };
    
    println!("Please input your data:");

    let mut data = String::new();
    
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read data input");

    let data: String = match data.trim().parse() {
        Ok(d) => d,
        Err(_) => panic!("Invalid data input"),
    };

    let data = &data.as_bytes();
    

    if let Err(e) = produce_message(data, &topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {}", e);
    }
}


fn produce_message<'a, 'b>(
    data: &'a [u8],
    topic: &'b str,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {}", brokers, topic);

    let mut producer = Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;

    producer.send(&Record {
        topic,
        partition: -1,
        key: (),
        value: data,
    })?;

    producer.send(&Record::from_value(topic, data))?;

    Ok(())
}