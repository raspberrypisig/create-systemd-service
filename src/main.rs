mod wizard;
mod choices;
mod unitfile;

//use unitfile::UnitFile;
mod cli_controller;

fn main() {
    cli_controller::run();
    //let unit_file = UnitFile::default();
}
