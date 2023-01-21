use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use colored::Colorize;
use crossterm::{cursor, QueueableCommand};
use std::io::{self, Write};

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("password")
                .help("Password that you want to test. If empty, will be asked via stdin.")
                .index(1),
        )
        .arg(
            Arg::new("secure")
                .short('s')
                .long("secure")
                .action(clap::ArgAction::SetTrue)
                .help("Do not output password and sequence."),
        )
        .get_matches();

    let password = match matches.get_one::<String>("password") {
        Some(password) => password.to_string(),
        None => {
            let password = rpassword::prompt_password(&"➜ ".magenta().to_string())
                .expect("unable to read password");

            let mut stdout = io::stdout();
            stdout
                .queue(cursor::MoveUp(1))
                .expect("unable to queue moving terminal cursor up");
            stdout
                .flush()
                .expect("unable to flush stdout to move terminal cursor up");

            password
        }
    };

    let hide_password = matches.get_one::<bool>("secure").copied().unwrap_or(false);

    if let Err(e) = zxcvbn_cli::run(password.as_str(), hide_password) {
        eprintln!("{} {}", "error:".red().bold(), e);
    }
}
