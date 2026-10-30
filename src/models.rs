use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptanceFormData {
    pub services: Services,
    pub pests: Vec<Pest>,
    pub products: Vec<Product>,
    pub spaces: HashMap<String, bool>,
    pub criteria: HashMap<String, Option<bool>>,
    pub customer: Customer,
    pub start_time: String,
    pub end_time: String,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub representative: Representative,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Representative {
    pub name: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub disinsection: bool,
    pub deratization: bool,
    pub disinfection: bool,
    pub subcontractor_prevention: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pest {
    pub name: String,
    pub checked: bool,
    pub monitor: bool,
    pub spray: bool,
    pub gel: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub name: String,
    pub checked: bool,
    pub dosage: String,
    pub used: String,
}
