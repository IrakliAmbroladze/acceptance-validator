use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptanceFormData {
    pub services: Services,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub disinsection: bool,
    pub deratization: bool,
    pub disinfection: bool,
    pub subcontractor_prevention: bool,
}
