use anyhow::Result;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use crate::kafka::config::KafkaConfig;
use log::info;

pub struct KafkaProducerSync {
    producer: BaseProducer,
    topic: String,
}

impl KafkaProducerSync {
    pub fn new(config: &KafkaConfig) -> Result<Self> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", &config.broker)
            .set("security.protocol", "SASL_PLAINTEXT")
            .set("sasl.mechanism", "PLAIN")
            .set("sasl.username", &config.username)
            .set("sasl.password", &config.password)
            .create::<BaseProducer>()?;

        Ok(Self {
            producer,
            topic: config.topic.clone(),
        })
    }

    pub fn send(&self, value: &str) -> Result<()> {
        match self.producer.send(
            BaseRecord::to(&self.topic)
                .payload(value)
                .key("key"),
        ) {
            Ok(_) => {
                info!("📤 Sent (sync): {}", value);
            }
            Err((err, _record)) => {
                return Err(anyhow::anyhow!("Kafka sync send error: {}", err));
            }
        }

        // Flush sync producer
        let _ = self.producer.flush(std::time::Duration::from_secs(1));

        Ok(())
    }
}
