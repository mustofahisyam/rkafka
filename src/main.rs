use std::time::Duration;
use std::env;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main() {
    let broker = env::var("BROKER_SERVER").expect("Error: VAR_NAME not found");

    
}
