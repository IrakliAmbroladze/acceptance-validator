use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptanceFormData {
    pub services: Services,
    pub pests: Vec<Pest>,
    pub products: Vec<Product>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub disinsection: bool,
    pub deratization: bool,
    pub disinfection: bool,
    pub subcontractor_prevention: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pest {
    pub name: String,
    pub checked: bool,
    pub monitor: bool,
    pub spray: bool,
    pub gel: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub name: String,
    pub checked: bool,
    pub dosage: String,
    pub used: String,
}
