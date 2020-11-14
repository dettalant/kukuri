mod cli;
mod config;
mod core;
mod export;
mod import;
mod utils;

fn main() {
    let matches = cli::build_cli().get_matches();

    let conf = match matches.value_of("config") {
        Some(path) => config::Config::from_file(path),
        None => config::Config::new(),
    };

    let mut kukuri = core::Kukuri::from_config(conf);

    if let Some(path) = matches.value_of("dir") {
        kukuri.set_output_dir(path)
    }

    if let Some(path) = matches.value_of("l10n_dir") {
        kukuri.set_l10n_output_dir(path)
    }

    for path in matches
        .values_of("FILE")
        .expect("Failed to get input files")
    {
        kukuri.append_input(path);
        // kukuri.import(path);
    }

    kukuri.run();
}
