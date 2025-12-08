use anyhow::Result;
use log::info;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
// use rdkafka::error::KafkaError;
// use rdkafka::producer::DeliveryFuture;

use crate::kafka::config::KafkaConfig;
use std::time::Duration;

pub struct KafkaProducerAsync {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducerAsync {
    pub fn new(config: &KafkaConfig) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            // --- connection ---
            .set("bootstrap.servers", &config.broker)

            // --- SASL/PLAIN ---
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "PLAIN")
            .set("sasl.username", &config.username)
            .set("sasl.password", &config.password)

            // --- Optional tuning ---
            .set("message.timeout.ms", "5000")
            .create()?;

        info!("🧩 Async Producer connected to {}", &config.broker);

        Ok(Self {
            producer,
            topic: config.topic.clone(),
        })
    }

    /// 🔥 Async send: mengembalikan DeliveryReport lengkap
    pub async fn send(&self, message: &str) -> Result<()> {
        let record = FutureRecord::to(&self.topic)
            .payload(message)
            .key("");

        // FutureProducer → .send() returns a Future
        let delivery_status = self
            .producer
            .send(record, Duration::from_secs(1))
            .await;

        match delivery_status {
            Ok((partition, offset)) => {
                info!(
                    "📦 delivered: '{}' → partition {}, offset {}",
                    message, partition, offset
                );
                Ok(())
            }
            Err((err, _msg)) => {
                Err(anyhow::anyhow!("❌ delivery failed: {}", err))
            }
        }
    }
}
