
#[derive(Default, Debug)]
pub struct Unit {
    pub description: String,
    pub after: Option<String>,
    pub wants: Option<String>
}

#[derive(Default, Debug)]
pub struct Service {
    execStart: String
}

#[derive(Default, Debug)]
pub struct Install {
    wantedBy: String
}

#[derive(Default, Debug)]
pub struct UnitFile {
    unit: Unit,
    service: Service,
    install: Install
}

impl UnitFile {
    fn create_unit_file() {

    }
}