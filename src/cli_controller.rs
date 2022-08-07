use inquire::ui::{RenderConfig, StyleSheet, Attributes, Color};

use crate::unitfile::{Unit, Service, Install, UnitFile};
use crate::wizard::step1::simple_or_complex_mode;
use crate::choices::step1::Options::{Simple, BellsAndWhistles};
use crate::wizard::step2a::{ simple_unit_section, simple_service_section, simple_install_section };
use std::borrow::Cow;

pub fn run () {
    inquire::set_global_render_config(get_render_config());

    let mode = simple_or_complex_mode();
    match mode {
            Simple => simple(),
            BellsAndWhistles => bells_and_whistles(),
    }        
}

pub fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();  
    render_config.prompt = StyleSheet::new()
    .with_attr(Attributes::BOLD)
    .with_fg(Color::LightYellow);  
    render_config
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
    println!("=================================");

    let unit_file = UnitFile::new(u, s, i);
    let boo = serde_ini::to_string(&unit_file);
    let needle = vec![
        ("[Service]", "\n[Service]"), 
        ("[Install]", "\n[Install]")
    ];
    match boo {
        Ok(ini) => {
            let m = needle.iter().fold(Cow::from(ini), |s, &(from, too)|  {s.replace(from, too).into()});
            println!("{}", m);
            
        },
        Err(err) => eprintln!("{err}"),
    };
    
}

fn bells_and_whistles() {
    println!("bells");
}
