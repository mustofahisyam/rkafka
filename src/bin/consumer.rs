use std::io;
use std::env;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;


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
    
    println!("Please input your grup:");

    let mut group = String::new();
    
    io::stdin()
        .read_line(&mut group)
        .expect("Failed to read grup input");

    let group: String = match group.trim().parse() {
        Ok(d) => d,
        Err(_) => panic!("Invalid grup input"),
    };


    let topic = topic.to_owned();
    let group = group.to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}


fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!(
                    "{}:{}@{}: {:?}",
                    ms.topic(),
                    ms.partition(),
                    m.offset,
                    m.value
                );
            }
            let _ = con.consume_messageset(ms);
        }
        con.commit_consumed()?;
    }
}