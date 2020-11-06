mod cli;
mod core;

use std::path::PathBuf;

fn main() {
    let matches = cli::build_cli().get_matches();

    let config = match matches.value_of("config") {
        Some(path) => core::config::Config::from_file(path),
        None => core::config::Config::new(),
    };


    let output_dir = match matches.value_of("dir") {
        Some(path) => PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Can't find output directory: {}", path)),
        None => std::env::current_dir().expect("Failed get current directory.")
    };

    println!("conf: {:?}", config);
    println!("output_dir: {:?}", output_dir);

    let inputs: Vec<_> = matches.values_of("FILE").unwrap().collect();
    println!("Using input files: {:?}", inputs);
}
