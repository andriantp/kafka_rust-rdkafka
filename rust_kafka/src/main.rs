mod kafka;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use chrono::Utc;
use dotenvy;
use env_logger::Env;
use log::info;
use rand::Rng;
use std::{thread, time::Duration};

use kafka::config::KafkaConfig;
use kafka::producer_sync::KafkaProducerSync;
use kafka::producer_async::KafkaProducerAsync;
use kafka::consumer_sync::KafkaConsumerSync;
use kafka::consumer_async::KafkaConsumerAsync;

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    SyncProducer,
    AsyncProducer,
    SyncConsumer,
    AsyncConsumer,
}

#[derive(Parser)]
#[command(name = "rust_kafka", version, about = "Kafka demo in Rust SASL with (Sync & Async)", long_about = None)]
struct Cli {
    #[arg(value_enum)]
    mode: Mode,
}

fn main() -> Result<()> {
    println!("📂 Current dir: {:?}", std::env::current_dir());

    // Load .env
    if dotenvy::dotenv().is_err() {
        panic!("⚠️ .env file not found");
    }

    // Init logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();
    let config = KafkaConfig::new();

    match cli.mode {
        Mode::SyncProducer => run_sync_producer(config)?,
        Mode::AsyncProducer => run_async_producer(config)?,
        Mode::SyncConsumer => run_sync_consumer(config)?,
        Mode::AsyncConsumer => run_async_consumer(config)?,
    }

    Ok(())
}

fn run_sync_producer(config: KafkaConfig) -> Result<()> {
    let producer = KafkaProducerSync::new(&config)?;
    info!("🚀 Running SYNC producer...");

    loop {
        let msg = make_random_json();
        producer.send(&msg)?;
        thread::sleep(Duration::from_secs(3));
    }
}

fn run_sync_consumer(config: KafkaConfig) -> Result<()> {
    let consumer = KafkaConsumerSync::new(&config)?;
    info!("📥 Running SYNC consumer...");
    consumer.poll_messages()
}

fn run_async_producer(config: KafkaConfig) -> Result<()> {
    info!("🚀 Running ASYNC producer...");
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let producer = KafkaProducerAsync::new(&config)?;
        loop {
            let msg = make_random_json();
            producer.send(&msg).await?;
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    })
}

fn run_async_consumer(config: KafkaConfig) -> Result<()> {
    info!("📥 Running ASYNC consumer...");
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let consumer = KafkaConsumerAsync::new(&config)?;
        consumer.run().await?;
        Ok(())
    })
}

fn make_random_json() -> String {
    let temp = rand::thread_rng().gen_range(20.0..30.0);
    let humidity = rand::thread_rng().gen_range(50.0..80.0);
    let timestamp = Utc::now().to_rfc3339();

    format!(
        r#"{{"sensor":"A1","temp":{:.2},"humidity":{:.2},"time":"{}"}}"#,
        temp, humidity, timestamp
    )
}
