
use strum_macros::{EnumString, Display};

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Options {
    #[strum(serialize="Give me systemd service file quickly!")]
    Simple,
    #[strum(serialize="Give me systemd service file with bells and whisles")]
    BellsAndWhistles
}