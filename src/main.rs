use std::env;
use std::process;

use tracing::info;
use tracing_subscriber;

use grafana_cmd_tracing::Config;

use metrics::{Key, Label, SharedString};
use std::borrow::Cow;

fn main() {

    println!("Key: {} bytes", std::mem::size_of::<Key>());
    println!("Label: {} bytes", std::mem::size_of::<Label>());
    println!("Cow<'static, [Label]>: {} bytes", std::mem::size_of::<Cow<'static, [Label]>>());
    println!("Vec<SharedString>: {} bytes", std::mem::size_of::<Vec<SharedString>>());
    println!(
       "[Option<SharedString>; 2]: {} bytes",
       std::mem::size_of::<[Option<SharedString>; 2]>()
    );

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

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
