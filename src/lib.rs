use std::{error::Error};
use std::fs;
//use std::thread;
use opentelemetry::sdk::export::trace::stdout;
use tracing::{debug, info, warn, span, Level, event, instrument, info_span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use lazy_static::lazy_static;
use prometheus::{ IntCounterVec, register_int_counter_vec };
//use prometheus_static_metric;

lazy_static! {
    pub static ref LETTER_COUNTER: IntCounterVec =
        register_int_counter_vec!("letter_counter_vec", "Letter Counter in line", &["line", "letter"])
        .expect("Can't create a metric");
}

// use metrics::{counter, histogram};
//
// pub fn process(query: &str) -> u64 {
//     let start = Instant::now();
//     let row_count = run_query(query);
//     let delta = start.elapsed();
//
//     histogram!("process.query_time", delta);
//     counter!("process.query_row_count", row_count);
//
//     row_count
// }


//#[tracing::instrument]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("not enough arguments"); }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let tracer = stdout::new_pipeline().install_simple();
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    event!(Level::INFO, "something happened");

    let span = span!(Level::INFO, "span_of_run_function");
    let _guard = span.enter();
    event!(Level::DEBUG, "something happened inside my_span");
    //let mut query_count_vector = Vec::new();
    // let mut loop_counter = 0;
    // loop {
    //     LETTER_COUNTER.with_label_values(&["testt","testt"]).inc();
    //     loop_counter += 1;
    //     if loop_counter >= 10 {break;}
    // }
    let contents = info_span!("txt.parse").in_scope(|| fs::read_to_string(config.filename))?;
    for line in search(&config.query, &contents) {
        //LETTER_COUNTER.with_label_values(&[&line, &config.query])
        //.inc_by(line.matches(&config.query).count().try_into().unwrap());
        //query_count_vector.push(line.matches(&config.query).count());
        span!(Level::TRACE, "liene", line, "{}", line.matches(&config.query).count());
        println!("{}, occurs {} times", line, line.matches(&config.query).count());
        //println!("LETTER_COUNTER[{}, {}] = {}",&line, &config.query,  LETTER_COUNTER.with_label_values(&[&line, &config.query]).get());
    }

    Ok(())
}

#[instrument]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    event!(Level::INFO, "inside search function!");
    let mut results = Vec::new();
    for line in contents.lines() {
        let count_in_line = line.matches(query).count();
        if count_in_line > 0 {
            results.push(line);
        }
    }
    if results.len() == 0 { println!("Word {} not found in text", query); }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
