# 📘 Rust Kafka (rust-rdkafka) + SASL/PLAIN + KRaft Mode

Implementation of Kafka producers & consumers in Rust using rust-rdkafka, with SASL/PLAIN authentication and Kafka KRaft mode (no Zookeeper).
This project supports 4 operation modes:
- Sync Producer
- Sync Consumer
- Async Producer
- Async Consumer

All components run end-to-end using Docker Compose and AKHQ.

## 🏗️ Project Structure

```bash
.
├── docker                            # 🐳 Environment for running local Kafka
│   ├── docker-compose.yml            # ⚙️ Kafka service definition (with SASL/PLAIN)
│   ├── kafka                         # 📂 Optional folder for extra Kafka configs (server.properties, etc)
│   └── secrets                       # 🔐 JAAS config for SASL/PLAIN
│       └── kafka_server_jaas.conf    # 🪪 JAAS configuration for broker authentication
│
├── Makefile                          # 🧰 Helpful shortcuts for Docker
│
└── rust_kafka                        # 🦀 Main Rust-based Kafka client project
    ├── .env
    ├── Cargo.toml
    └── src
        ├── kafka
        │   ├── config.rs          # Load environment variables (broker, topic, credentials)
        │   ├── producer_sync.rs   # BaseProducer (SYNC)
        │   ├── producer_async.rs  # FutureProducer (ASYNC)
        │   ├── consumer_sync.rs   # BaseConsumer (SYNC)
        │   ├── consumer_async.rs  # StreamConsumer (ASYNC)
        │   └── mod.rs
        └── main.rs               # CLI commands (sync-producer, async-consumer, etc)

```

## 🔐 Environment Variables (.env)

```bash
KAFKA_BROKER=localhost:9094
KAFKA_TOPIC=rustkafka-sync
KAFKA_GROUP_ID=group-1
KAFKA_USER=client
KAFKA_PASSWORD=client-secret
```

## 🔒 SASL JAAS Config

File: ./secrets/kafka_server_jaas.conf
```bash
KafkaServer {
  org.apache.kafka.common.security.plain.PlainLoginModule required
  user_client="client-secret";
};
```
Meaning :
```bash
User = client,
Password = client-secret.
```

## 🐳 Docker Compose (Kafka KRaft + SASL/PLAIN + AKHQ)

Features:
- KRaft mode (no Zookeeper)
- PLAINTEXT listener (9092) for AKHQ
- SASL_PLAINTEXT listener (9094) for Rust clients
- AKHQ for observing topics & messages

Deploy
```bash
make up
```

Access:
- [akhq](http://localhost:8080)



## 📦 Build & Run
- Install dependencies
```bash
sudo apt install -y build-essential pkg-config libssl-dev libsasl2-dev cmake
```
- Build
```bash
cargo build
```

🚀 Tes
```bash
cargo run -- sync-producer
cargo run -- sync-consumer

cargo run -- async-producer
cargo run -- async-consumer
```


## 🔍 Difference: SYNC vs ASYNC

### Sync
- Blocking
- Simple
- Suitable for low throughput workloads

### Async
- Non-blocking
- High throughput
- Supports delivery reports
- Ideal for real-time streaming

## 🔗 Referensi

Full article:
[Adding SASL/PLAIN Authentication to Rust Clients with rust-rdkafka](https://andriantriputra.medium.com/be-rust-kafka-adding-sasl-plain-authentication-to-rust-clients-with-rust-rdkafka-d3c6f4cc7591)

---

## Author

Andrian Tri Putra
- [Medium](https://andriantriputra.medium.com/)
GitHub
- [andriantp](https://github.com/andriantp)
- [AndrianTriPutra](https://github.com/AndrianTriPutra)

---

## License
Licensed under the Apache License 2.0
