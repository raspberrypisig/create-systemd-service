use crate::unitfile::{Unit, Service, Install};
use crate::wizard::step1::simple_or_complex_mode;
use crate::choices::step1::Options::{Simple, BellsAndWhistles};
use crate::wizard::step2a::{ simple_unit_section, simple_service_section, simple_install_section };

pub fn run () {
    let mode = simple_or_complex_mode();
    match mode {
            Simple => simple(),
            BellsAndWhistles => bells_and_whistles(),
    }        
}

fn simple() {
    println!("simple");
    let mut u = Unit::default();
    let mut s = Service::default();
    let mut i = Install::default();

    simple_unit_section(&mut u);
    simple_service_section(&mut s);
    simple_install_section(&mut i);

    println!("Unit: {}", u.description);
}

fn bells_and_whistles() {
    println!("bells");
}
