use inquire::Text;

use crate::unitfile::{Unit, Service, Install};

pub fn simple_unit_section(u: &mut Unit) {
    let name = Text::new("Name of Service")
    .with_help_message("Required")
    .prompt()
    .unwrap();
    println!("{name}");
    u.description = name;

    let description = Text::new("Description")
    .with_help_message("Can be empty")
    .prompt()
    .unwrap();
    println!("{description}");
}

pub fn simple_service_section(s: &mut Service) {
    let name = Text::new("ExecStart")
    .with_help_message("command to run (use full paths). Required.")
    .prompt()
    .unwrap();
    println!("{name}");

}

pub fn simple_install_section(i: &mut Install) {

}