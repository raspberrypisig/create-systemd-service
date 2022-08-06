use std::str::FromStr;

use crate::choices::step1::Options;
use inquire::Select;
//use std::{str::FromStr};

pub fn simple_or_complex_mode() -> Options {
    let options:Vec<String>= vec![
        Options::Simple.to_string(),
        Options::BellsAndWhistles.to_string()
    ];

    let ans = Select::new(
        "How would you like to create your systemd unit file?",
        options,
    )
    .prompt().unwrap();

    Options::from_str(&ans).unwrap()
}
