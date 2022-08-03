
use inquire::Select;

use std::{str::FromStr, fs::OpenOptions};
use strum_macros::{EnumString, Display};

#[derive(Debug, PartialEq, EnumString, Display)]
enum Options {
    #[strum(serialize="Give me systemd service file quickly!")]
    Simple,
    #[strum(serialize="Give me systemd service file with bells and whisles")]
    BellsAndWhistles
}

pub fn simple_or_complex_mode() {
    /*let options = vec![
        "Give me systemd service file quickly!",
        "Give me systemd service file with bells and whisles",
    ];*/
    let options:Vec<String>= vec![
        Options::Simple.to_string(),
        Options::BellsAndWhistles.to_string()
    ];

    let ans = Select::new(
        "How would you like to create your systemd unit file?",
        options,
    )
    .prompt();
    match ans {
        Ok(answer) => match Options::from_str(&answer) {
            Ok(val) => println!("{}", &val.to_string()),
            Err(_) => (),
        },
        Err(_) => println!("Don't know which mode you want"),
    }
}
