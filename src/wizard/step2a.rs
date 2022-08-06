use inquire::Text;

use crate::unitfile::{Unit, Service, Install};



pub fn simple_unit_section(u: &mut Unit) {
    let name = Text::new("Name of Service")
    .with_help_message("Required")
    .prompt()
    .unwrap();
    //println!("{name}");
    u.name = name;

    let description = Text::new("Description")
    .with_help_message("Can be empty")
    .prompt()
    .unwrap();
    //println!("{description}");
    u.description = description;

    u.after = String::from("");
    u.wants = String::from("");
}

pub fn simple_service_section(s: &mut Service) {
    let exec_start = Text::new("ExecStart")
    .with_help_message("command to run (use full paths). Required.")
    .prompt()
    .unwrap();
    //println!("{exec_start}");
    s.exec_start = exec_start;
}

pub fn simple_install_section(i: &mut Install) {
    let wanted_by = Text::new("Install WantedBy")
    .with_help_message("usually multi-users.target or graphical.target")
    .prompt()
    .unwrap();
    //println!("{wanted_by}");
    i.wanted_by = wanted_by;
}

