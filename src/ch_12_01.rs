use ds_learn_rust::{run, Config};
use std::{env, process};

pub fn ch_12_01() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|x| {
        println!("Failed to parse config: {x}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Error: {e}");
        process::exit(1);
    };
}
