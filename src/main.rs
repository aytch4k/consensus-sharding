use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use prometheus::{Encoder, TextEncoder, IntCounter, IntGauge, Opts, Registry};
use serde::Deserialize;
use std::fs;
use std::sync::Arc;
use std::sync::Mutex;
use std::env;

// Configuration for the shard node
#[derive(Deserialize)]
struct ShardConfig {
    shard_id: u32,
    port: u16,
    validators: u32,
    max_transactions_per_block: u32,
    block_time_milliseconds: u32,
    state_storage: String,
}

// Function to load configuration from a file
fn load_config(file_path: &str) -> ShardConfig {
    let config_data = fs::read_to_string(file_path).expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("Unable to parse config file")
}

// Prometheus Metrics
struct Metrics {
    processed_transactions: IntCounter,
    failed_transactions: IntCounter,
    processing_latency: IntGauge,
}

impl Metrics {
    fn new() -> Self {
        let processed_opts = Opts::new("processed_transactions_total", "Total processed transactions");
        let failed_opts = Opts::new("failed_transactions_total", "Total failed transactions");
        let latency_opts = Opts::new("shard_processing_latency_milliseconds", "Shard processing latency in milliseconds");

        let processed_transactions = IntCounter::with_opts(processed_opts).unwrap();
        let failed_transactions = IntCounter::with_opts(failed_opts).unwrap();
        let processing_latency = IntGauge::with_opts(latency_opts).unwrap();

        Metrics {
            processed_transactions,
            failed_transactions,
            processing_latency,
        }
    }
}

// Shared Metrics Registry
fn setup_registry(metrics: &Metrics) -> Registry {
    let registry = Registry::new();
    registry.register(Box::new(metrics.processed_transactions.clone())).unwrap();
    registry.register(Box::new(metrics.failed_transactions.clone())).unwrap();
    registry.register(Box::new(metrics.processing_latency.clone())).unwrap();
    registry
}

// Process Transaction Handler
async fn process_transaction(data: web::Json<serde_json::Value>, metrics: web::Data<Arc<Mutex<Metrics>>>) -> impl Responder {
    let start_time = std::time::Instant::now();
    println!("Received transaction: {:?}", data);

    let mut metrics = metrics.lock().unwrap();
    if let Some(transaction_id) = data["transaction_id"].as_str() {
        metrics.processed_transactions.inc();
        metrics.processing_latency.set(start_time.elapsed().as_millis() as i64);
        HttpResponse::Ok().json({
            "status": "processed",
            "transaction_id": transaction_id
        })
    } else {
        metrics.failed_transactions.inc();
        HttpResponse::BadRequest().json({
            "status": "failed",
            "reason": "Invalid transaction data"
        })
    }
}

// Metrics Endpoint
async fn metrics_endpoint(metrics: web::Data<Arc<Mutex<Metrics>>>) -> impl Responder {
    let registry = setup_registry(&metrics.lock().unwrap());
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&registry.gather(), &mut buffer).unwrap();
    HttpResponse::Ok().body(String::from_utf8(buffer).unwrap())
}

// Main Function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config_path = env::var("SHARD_CONFIG").expect("SHARD_CONFIG environment variable not set");
    let config: ShardConfig = load_config(&config_path);

    println!("Starting Shard Node: {}", config.shard_id);
    println!("Listening on port: {}", config.port);
    println!("Validators: {}", config.validators);
    println!("Max Transactions per Block: {}", config.max_transactions_per_block);
    println!("Block Time: {} ms", config.block_time_milliseconds);
    println!("State Storage: {}", config.state_storage);

    let metrics = web::Data::new(Arc::new(Mutex::new(Metrics::new())));

    HttpServer::new(move || {
        App::new()
            .app_data(metrics.clone())
            .route("/process", web::post().to(process_transaction))
            .route("/metrics", web::get().to(metrics_endpoint))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
