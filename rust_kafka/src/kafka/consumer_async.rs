use anyhow::Result;
use log::info;
use rdkafka::consumer::{Consumer, StreamConsumer, CommitMode};
use rdkafka::{ClientConfig, Message};
use futures::StreamExt;

use crate::kafka::config::KafkaConfig;

pub struct KafkaConsumerAsync {
    consumer: StreamConsumer,
}

impl KafkaConsumerAsync {
    pub fn new(config: &KafkaConfig) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            // --- connection ---
            .set("bootstrap.servers", &config.broker)

            // --- SASL/PLAIN ---
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "PLAIN")
            .set("sasl.username", &config.username)
            .set("sasl.password", &config.password)

            // --- consumer group ---
            .set("group.id", &config.group_id)

            // --- earliest offset ---
            .set("auto.offset.reset", "earliest")

            // --- manual commit (biar sama seperti versi sync) ---
            .set("enable.auto.commit", "false")

            .create()?;

        consumer.subscribe(&[&config.topic])?;

        info!(
            "📥 Async Consumer connected → broker: {}, topic: {}",
            config.broker, config.topic
        );

        Ok(Self { consumer })
    }

    /// 🔥 Async stream consumer
    pub async fn run(&self) -> Result<()> {
        info!("📥 Async Consumer started, waiting for messages...");

        let mut stream = self.consumer.stream();

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => {
                    // payload
                    if let Some(Ok(text)) = message.payload_view::<str>() {
                        println!("✅ received: {}", text);
                    }

                    // manual commit
                    self.consumer.commit_message(&message, CommitMode::Async)?;
                }

                Err(e) => {
                    println!("❌ stream error: {:?}", e);
                }
            }
        }

        Ok(())
    }
}
