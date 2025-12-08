use std::time::Duration;
use anyhow::Result;
use log::info;

use rdkafka::consumer::{BaseConsumer, Consumer, CommitMode};
use rdkafka::ClientConfig;
use rdkafka::message::Message;

use crate::kafka::config::KafkaConfig;

pub struct KafkaConsumerSync {
    consumer: BaseConsumer,
}

impl KafkaConsumerSync {
    pub fn new(config: &KafkaConfig) -> Result<Self> {
        // 🚀 Build BaseConsumer (sync)
        let consumer: BaseConsumer = ClientConfig::new()
            // --- connection ---
            .set("bootstrap.servers", &config.broker)

            // --- SASL/PLAIN ---
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "PLAIN")
            .set("sasl.username", &config.username)
            .set("sasl.password", &config.password)

            // --- consumer group ---
            .set("group.id", &config.group_id)

            // --- message offset (Earliest) ---
            .set("auto.offset.reset", "earliest")

            // --- disable auto-commit (manual commit) ---
            .set("enable.auto.commit", "false")

            .create()?;

        // subscribe topic
        consumer.subscribe(&[&config.topic])?;

        info!(
            "📥 Consumer connected → broker: {}, topic: {}",
            config.broker, config.topic
        );

        Ok(Self { consumer })
    }

    pub fn poll_messages(&self) -> Result<()> {
        info!("📥 Consumer started, waiting for messages...");

        loop {
            // poll mirip dengan library lama
            if let Some(result) = self.consumer.poll(Duration::from_millis(500)) {
                match result {
                    Ok(message) => {
                        // payload
                        if let Some(payload) = message.payload_view::<str>() {
                            match payload {
                                Ok(text) => println!("✅ received: {}", text),
                                Err(e) => println!("⚠️ payload error: {:?}", e),
                            }
                        }

                        // manual commit offset
                        self.consumer.commit_message(&message, CommitMode::Sync)?;
                    }

                    Err(e) => {
                        println!("❌ poll error: {:?}", e);
                    }
                }
            }
        }
    }
}
