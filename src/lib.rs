//! A simple CLI tool to check password strength using zxcvbn.
//!
//! See [README.md](https://github.com/u32i64/zxcvbn-cli#readme) for more information.
#![deny(missing_docs)]

use colored::Colorize;
use zxcvbn::{Entropy, Match};

macro_rules! info {
    ($name:expr) => {
        println!("{}", $name.bright_blue().bold());
    };
    ($name:expr, $value:expr) => {
        println!("{} {}", $name.bright_blue().bold(), $value);
    };
}

/// Runs the main logic of the app.
pub fn run(password: &str) -> Result<(), String> {
    if password.is_empty() {
        return Err("empty password".into());
    }

    let entropy = zxcvbn::zxcvbn(password, &[]).unwrap();

    print_password_tokenized(&entropy);
    main_information(&entropy);
    guesses(&entropy);
    crack_time(&entropy);

    info!("sequence");
    sequence(entropy.sequence.clone(), 0);

    println!();
    println!(
        "{}",
        format!("zxcvbn done in {} ms", entropy.calc_time).bright_black()
    );

    Ok(())
}

fn print_password_tokenized(entropy: &Entropy) {
    print!("{} ", "➜".magenta());

    for (i, token) in entropy.sequence.iter().map(|m| &m.token).enumerate() {
        print!(
            "{}",
            if (i + 1) % 2 == 0 {
                token.on_bright_black()
            } else {
                token.normal()
            }
        );
    }

    println!();
}

fn main_information(entropy: &Entropy) {
    info!(
        "score",
        format!(
            "{}{}",
            match entropy.score {
                0 => "0".bright_red(),
                1 => "1".red(),
                2 => "2".yellow(),
                3 => "3".green(),
                4 => "4".bright_green(),
                _ => unreachable!(),
            },
            " / 4"
        )
        .bold()
    );

    if let Some(feedback) = entropy.feedback.clone() {
        if let Some(warning) = feedback.warning {
            println!("{} {}", "warning".bright_yellow().bold(), warning);
        }

        if !feedback.suggestions.is_empty() {
            println!("{}", "suggestions".bright_green().bold());

            let i_last = feedback.suggestions.len() - 1;
            for (i, suggestion) in feedback.suggestions.iter().enumerate() {
                println!(
                    "{} {}",
                    if i < i_last { "  ├" } else { "  └" }
                        .bright_green()
                        .bold(),
                    suggestion
                );
            }
        }
    }
}

fn guesses(entropy: &Entropy) {
    info!(
        "guesses",
        "1e".to_string() + &entropy.guesses_log10.to_string()
    );
}

macro_rules! fmt_crack_time {
    ($entropy:expr, $name:ident, $desc:expr) => {
        format!(
            "{} {}",
            $entropy.crack_times_display.$name,
            $desc.bright_black(),
        )
    };
}

fn crack_time(entropy: &Entropy) {
    info!("crack time");
    info!(
        "  ├ 10²/hour",
        fmt_crack_time!(entropy, online_throttling_100_per_hour, "online, throttled")
    );
    info!(
        "  ├ 10  /sec",
        fmt_crack_time!(
            entropy,
            online_no_throttling_10_per_second,
            "online, not throttled"
        )
    );
    info!(
        "  ├ 10⁴ /sec",
        fmt_crack_time!(
            entropy,
            offline_slow_hashing_1e4_per_second,
            "offline, slow hash"
        )
    );
    info!(
        "  └ 10¹⁰/sec",
        fmt_crack_time!(
            entropy,
            offline_fast_hashing_1e10_per_second,
            "offline, fast hash"
        )
    );
}

macro_rules! pattern_info {
    ($name:expr, $value:expr, $indent:expr) => {
        println!(
            "  {}{} {} {}",
            " ".repeat($indent),
            "├".bright_blue(),
            $name.bright_blue(),
            $value
        );
    };
    (last $name:expr, $value:expr, $indent:expr) => {
        println!(
            "  {}{} {} {}",
            " ".repeat($indent),
            "└".bright_blue(),
            $name.bright_blue(),
            $value
        );
    };
    (type $name:expr, $indent:expr) => {
        println!(
            "  {}{} type {}",
            " ".repeat($indent),
            "├".bright_blue(),
            $name.blue()
        );
    };
    (type last $name:expr, $indent:expr) => {
        println!(
            "  {}{} type {}",
            " ".repeat($indent),
            "└".bright_blue(),
            $name.blue()
        );
    };
}

fn sequence(seq: Vec<Match>, indent: usize) {
    use zxcvbn::matching::patterns::MatchPattern::*;

    for (i, part) in seq.iter().enumerate() {
        println!(
            "  {}{}",
            " ".repeat(indent),
            if (i + 1) % 2 == 0 {
                part.token.underline().on_bright_black()
            } else {
                part.token.underline()
            }
        );

        match &part.pattern {
            Dictionary(pattern) => {
                pattern_info!(type "dictionary", indent);
                pattern_info!("word", pattern.matched_word, indent);
                pattern_info!("rank", pattern.rank, indent);
                pattern_info!("dictionary", pattern.dictionary_name, indent);
                pattern_info!("reversed?", pattern.reversed, indent);
                pattern_info!("l33t?", pattern.l33t, indent);
                if let Some(substitutions) = &pattern.sub_display {
                    let fixed_substitutions = substitutions
                        .chars()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .chunks_exact(6)
                        .map(|slice| slice.join(""))
                        .collect::<Vec<_>>()
                        .join(", ");

                    pattern_info!("substitutions", fixed_substitutions, indent);
                }
                pattern_info!("uppercase variations", pattern.uppercase_variations, indent);
                pattern_info!("l33t variations", pattern.l33t_variations, indent);
                pattern_info!(last "base guesses", pattern.base_guesses, indent);
            }
            Spatial(pattern) => {
                pattern_info!(type "spatial", indent);
                pattern_info!("graph", pattern.graph, indent);
                pattern_info!("turns", pattern.turns, indent);
                pattern_info!(last "shifts", pattern.shifted_count, indent);
            }
            Repeat(pattern) => {
                pattern_info!(type "repeat", indent);
                pattern_info!("base token", pattern.base_token, indent);
                pattern_info!("base matches", "", indent);
                sequence(pattern.base_matches.clone(), indent + 2);
                pattern_info!("base guesses", pattern.base_guesses, indent);
                pattern_info!(last "repeat count", pattern.repeat_count, indent);
            }
            Sequence(pattern) => {
                pattern_info!(type "sequence", indent);
                pattern_info!("name", pattern.sequence_name, indent);
                pattern_info!("space", pattern.sequence_space, indent);
                pattern_info!(last "ascending?", pattern.ascending, indent);
            }
            Regex(pattern) => {
                pattern_info!(type "regex", indent);
                pattern_info!("name", pattern.regex_name, indent);
                pattern_info!(last "match", format!("{:?}", pattern.regex_match), indent);
            }
            Date(pattern) => {
                pattern_info!(type "date", indent);
                pattern_info!("separator", pattern.separator, indent);
                pattern_info!("year", pattern.year, indent);
                pattern_info!("month", pattern.month, indent);
                pattern_info!(last "day", pattern.day, indent);
            }
            BruteForce => pattern_info!(type last "bruteforce", indent),
        }
    }
}
