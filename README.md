
# Producer Consumer (Rust<>Kafka)

## Producer
How to run producer:
```
$cargo run --bin producer
```
You will be asked to enter some of the information below
Name | Description 
--- | --- 
Topic | Select the kafka topic where do you want to send it
Message | Information to be sent or delivered to the consumer

## Consumer
How to run consumer:
```
$cargo run --bin consumer
```
You will be asked to enter some of the information below
Env Name | Description 
--- | --- 
Topic | Select the kafka topic where do you want to receive from it
group | Select the group who want to consume the messages from the topic


## Environment Set
You configure the following config in the Environment variable:

Env Name | Description 
--- | --- 
BROKER_SERVER | Configure the kafka server 

The configuration is set in `.cargo/config.toml` under `[env]` section