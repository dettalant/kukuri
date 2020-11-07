mod cli;
mod core;
mod import;

use std::path::PathBuf;

fn main() {
    let matches = cli::build_cli().get_matches();

    let config = match matches.value_of("config") {
        Some(path) => core::config::Config::from_file(path),
        None => core::config::Config::new(),
    };

    let mut kukuri = core::Kukuri::from_config(config);

    if let Some(path) = matches.value_of("dir") {
        let output_dir = PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Can't find output directory: {}", path));
        kukuri.set_output_dir(output_dir)
    }


    let inputs: Vec<_> = matches.values_of("FILE").unwrap().collect();
    // println!("Using input files: {:?}", inputs);

    for path in inputs {
        kukuri.import(path)
    }

    println!("kukuri: {:#?}", kukuri);
}
