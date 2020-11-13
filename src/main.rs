mod cli;
mod config;
mod core;
mod export;
mod import;
mod utils;

use std::path::PathBuf;

fn main() {
    let matches = cli::build_cli().get_matches();

    let conf = match matches.value_of("config") {
        Some(path) => config::Config::from_file(path),
        None => config::Config::new(),
    };

    let mut kukuri = core::Kukuri::from_config(conf);

    if let Some(path) = matches.value_of("dir") {
        let new_dir = PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Can't find output directory: {}", path));
        kukuri.set_output_dir(new_dir)
    }

    if let Some(path) = matches.value_of("l10n_dir") {
        let new_dir = PathBuf::from(path)
            .canonicalize()
            .unwrap_or_else(|_| panic!("Can't find l10n output directory: {}", path));
        kukuri.set_l10n_output_dir(new_dir)
    }

    let inputs: Vec<_> = matches.values_of("FILE").unwrap().collect();
    // println!("Using input files: {:?}", inputs);

    for path in inputs {
        kukuri.import(path)
    }

    if kukuri.conf.use_l10n_output {
        kukuri.l10n_export();
    }
}
