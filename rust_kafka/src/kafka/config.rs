use std::env;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub broker: String,
    pub topic: String,
    pub group_id: String,

    // 🔐 SASL/PLAIN credentials
    pub username: String,
    pub password: String,
}

impl KafkaConfig {
    pub fn new() -> Self {
        let cfg = Self {
            broker: env::var("KAFKA_BROKER").unwrap_or_else(|_| "localhost:9094".to_string()),
            topic: env::var("KAFKA_TOPIC").unwrap_or_else(|_| "kafka-rust".to_string()),
            group_id: env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "group-1".to_string()),

            // 🔐 SASL/PLAIN creds (fallback agar tidak panic saat dev)
            username: env::var("KAFKA_USER").unwrap_or_else(|_| "client".to_string()),
            password: env::var("KAFKA_PASSWORD").unwrap_or_else(|_| "client-pass".to_string()),
        };

        log::info!("🧩 Loaded Kafka config: {:?}", cfg);
        cfg
    }
}
