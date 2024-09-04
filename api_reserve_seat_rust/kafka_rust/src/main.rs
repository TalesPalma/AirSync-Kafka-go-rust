use std::time::Duration;

use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};

#[tokio::main]
async fn main() {
    produce_ksf().await;
    consumer_ksf().await;
}

async fn produce_ksf() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    let topic = "meu-topico";
    let payload = "Mensagem kafka";

    producer
        .send(
            FutureRecord::to(topic).payload(payload).key("Minha-key"),
            Duration::from_secs(3),
        )
        .await
        .expect("Failed to send message");

    println!("Message send {} sucessfully ", payload)
}

async fn consumer_ksf() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "meu-grupo")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(&["meu-topico"])
        .expect("Failed to subscribe to topic");

    loop {
        let message = consumer.recv().await.expect("Failed consumer msg");

        match message.payload_view::<str>() {
            Some(Ok(payload)) => println!("Messagem recebida: {}", payload),
            Some(Err(e)) => println!("Messagem vazia com erro : {:?}", e),
            None => println!("Messagem vazia"),
        }
    }
}
