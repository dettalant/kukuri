use clap::{crate_authors, crate_description, crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Kukuri")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
            .help("Set a config file")
            .short("c")
            .long("config")
            .value_name("FILE")
            .takes_value(true)
        )
        .arg(Arg::with_name("dir")
            .help("Set output directory")
            .short("d")
            .long("dir")
            .value_name("DIRECTORY")
            .takes_value(true)
        )
        .arg(Arg::with_name("l10n_dir")
            .help("Set l10n output directory")
            .short("l")
            .long("l10n_dir")
            .value_name("DIRECTORY")
            .takes_value(true)
        )
        .arg(Arg::with_name("FILE")
            .help("Set input file(s)")
            .required(true)
            .min_values(1)
        )
}
