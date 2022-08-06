use crate::unitfile::{Unit, Service, Install, UnitFile};
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

    //println!("Unit Description: {}", &u.description);
    //println!("Service ExecStart: {}", &s.exec_start);
    //println!("Install WantedBy: {}", &i.wanted_by);

    println!("\n\n=================================\n");
    println!("Unit file for /etc/systemd/system/{}.service", &u.name);
    println!("=================================\n\n");

    let unit_file = UnitFile::new(u, s, i);
    let boo = serde_ini::to_string(&unit_file);
    match boo {
        Ok(ini) => println!("{ini}"),
        Err(err) => eprintln!("{err}"),
    };
    
}

fn bells_and_whistles() {
    println!("bells");
}
