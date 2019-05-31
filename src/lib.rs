//! A simple CLI tool to check password strength using zxcvbn.
#![deny(missing_docs)]

/// Runs the main logic of the app.
pub fn run(password: &str, inputs: Vec<&str>) -> Result<(), String> {
    if !password.is_ascii() {
        return Err("Only ASCII passwords are supported.".into());
    }

    let entropy = zxcvbn::zxcvbn(password, inputs.as_slice());

    eprintln!("{:#?}", entropy);

    // TODO: actual app starts here

    Ok(())
}
