use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct Unit {
    #[serde(skip_serializing)]
    pub name: String,
    pub description: String,
    pub after: String,
    pub wants: String
}

#[derive(Default, Debug, Serialize)]
pub struct Service {
    pub exec_start: String
}

#[derive(Default, Debug, Serialize)]
pub struct Install {
    pub wanted_by: String
}

#[derive(Default, Debug, Serialize)]
pub struct UnitFile {
    pub unit: Unit,
    pub service: Service,
    pub install: Install
}

impl UnitFile {
    pub fn new(u: Unit, s:Service, i: Install) -> Self {
       Self { unit: u, service: s, install: i }
    }
}

