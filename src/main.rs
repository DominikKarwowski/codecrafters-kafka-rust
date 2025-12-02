#![allow(unused_imports)]
use codecrafters_kafka::{Config, run};

fn main() {
    println!("Logs from your program will appear here!");
    
    let config = Config::default();

    run(config);
}

