use std::{str::FromStr, thread::sleep, time::Duration};

use anyhow::Result;
use opentelemetry_appender_tracing::layer::{self, OpenTelemetryTracingBridge};
use opentelemetry_otlp::{LogExporter, Protocol, WithExportConfig};
use opentelemetry_sdk::{
    logs::{Logger, LoggerProvider},
    runtime::Tokio,
};
use tracing::{error, info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, Layer, prelude::*};

#[tokio::main]
async fn main() {
    let otlp_layer = build_otlp_log_bridge_layer();
    let _ = initialize_logger(otlp_layer, true);

    loop {
        info!(target: "stdout", message = "Hello, world!");
        error!(target: "otlp", message = "Hello, world! OTLP");
        sleep(Duration::from_secs(5));
    }
}

fn initialize_logger(
    otlp_layer: OpenTelemetryTracingBridge<LoggerProvider, Logger>,
    enable_otlp: bool,
) -> Result<()> {
    let registry = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_current_span(true)
                .with_target(true),
        )
        .with(LevelFilter::from_str("info")?);

    if enable_otlp {
        let filter_otel = EnvFilter::new("error")
            .add_directive("stdout=off".parse().unwrap())
            .add_directive("hyper=off".parse().unwrap())
            .add_directive("opentelemetry=off".parse().unwrap())
            .add_directive("tonic=off".parse().unwrap())
            .add_directive("h2=off".parse().unwrap())
            .add_directive("reqwest=off".parse().unwrap());
        registry.with(otlp_layer.with_filter(filter_otel)).init();
    } else {
        registry.init();
    }

    return Ok(());
}

fn build_otlp_log_bridge_layer() -> OpenTelemetryTracingBridge<LoggerProvider, Logger> {
    let log_exporter = LogExporter::builder()
        .with_tonic()
        .with_endpoint("http://127.0.0.1:4317")
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let log_provider = LoggerProvider::builder()
        .with_batch_exporter(log_exporter, Tokio)
        .build();

    return layer::OpenTelemetryTracingBridge::new(&log_provider);
}
