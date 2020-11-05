fn main() {
    println!("Clap Test");

    let matches = clap::App::new("Kukuri")
        .version("0.1.0")
        .author("dettalant")
        .arg(clap::Arg::with_name("config")
            .help("Set a config file")
            .short("c")
            .long("config")
            .value_name("FILE")
            .takes_value(true)
        )
        .arg(clap::Arg::with_name("FILE")
            .help("Set input file(s)")
            .required(true)
            .min_values(1)
        )
        .get_matches();

    if let Some(conf) = matches.value_of("config") {
        println!("Value for config: {}", conf)
    }

    let inputs: Vec<_> = matches.values_of("INPUTS").unwrap().collect();
    println!("Using input files: {:?}", inputs);
}
