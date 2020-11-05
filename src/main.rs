mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    if let Some(conf) = matches.value_of("config") {
        println!("Value for config: {}", conf)
    }

    if let Some(dir) = matches.value_of("dir") {
        println!("Value of output: {}", dir);
    }

    let inputs: Vec<_> = matches.values_of("FILE").unwrap().collect();
    println!("Using input files: {:?}", inputs);

    // TODO:
}
