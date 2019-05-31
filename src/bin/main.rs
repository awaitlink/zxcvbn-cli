use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .settings(&[AppSettings::UnifiedHelpMessage, AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("password")
                .help("Password that you want to test")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("inputs")
                .help("Other inputs, such as username, email, name")
                .multiple(true)
                .index(2),
        )
        .get_matches();

    let password = matches.value_of("password").unwrap();
    let input = matches
        .values_of("input")
        .unwrap_or_default()
        .collect::<Vec<_>>();

    if let Err(e) = zxcvbn_cli::run(password, input) {
        eprintln!("{}", e);
    }
}
