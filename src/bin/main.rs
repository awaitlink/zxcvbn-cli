use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use colored::Colorize;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .settings(&[AppSettings::UnifiedHelpMessage, AppSettings::ColoredHelp])
        .arg(
            Arg::with_name("password")
                .help("Password that you want to test. If empty, will be asked via stdin.")
                .index(1),
        )
        .get_matches();

    let password = match matches.value_of("password") {
        Some(password) => password.to_string(),
        None => {
            let password = rpassword::prompt_password_stdout(&"➜ ".magenta().to_string())
                .expect("unable to read password");
            crossterm_cursor::cursor().move_up(1);
            password
        }
    };

    if let Err(e) = zxcvbn_cli::run(password.as_str()) {
        eprintln!("{} {}", "error:".red().bold(), e);
    }
}
