use std::env;
use std::process;

use tracing::info;

use grafana_cmd_tracing::Config;

use metrics::{Key, Label, SharedString};
use tracing_subscriber::prelude::*;
use tracing_opentelemetry;
use std::borrow::Cow;

use opentelemetry::{KeyValue, sdk::Resource};
use opentelemetry::{ global::shutdown_tracer_provider};

use opentelemetry_otlp::{self, WithExportConfig};

fn main() {

    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://0.0.0.0:4317");
    // Then pass it into pipeline builder
    let tracer_config = opentelemetry::sdk::trace::config()
    .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name", "server service",
                    )]));
    let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(otlp_exporter)
            .with_trace_config(tracer_config) 
            .install_batch(opentelemetry::runtime::AsyncStd)
            .expect("failed to install");

    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(opentelemetry)
        .try_init();
    {
        println!("Key: {} bytes", std::mem::size_of::<Key>());
        println!("Label: {} bytes", std::mem::size_of::<Label>());
        println!("Cow<'static, [Label]>: {} bytes", std::mem::size_of::<Cow<'static, [Label]>>());
        println!("Vec<SharedString>: {} bytes", std::mem::size_of::<Vec<SharedString>>());
        println!(
        "[Option<SharedString>; 2]: {} bytes",
        std::mem::size_of::<[Option<SharedString>; 2]>()
        );

        let args: Vec<String> = env::args().collect();

        let config = Config::new(&args).unwrap_or_else(|err| {
            println!("Problem during arguments parsing: {}", err);
            process::exit(1);
        });

        info!(config.query, "Search ");
        println!("In file {}", config.filename);

        if let Err(e) = grafana_cmd_tracing::run(config) {
            println!("Error in application: {}", e);
            process::exit(1);
        }
    }
    shutdown_tracer_provider();
}
