use std::collections::HashMap;

use crate::models::{AcceptanceFormData, Pest, Product, Services};

pub fn validate_services(services: &Services) -> bool {
    services.disinsection
        || services.deratization
        || services.disinfection
        || services.subcontractor_prevention
}

pub fn validate_pests(pests: &[Pest]) -> bool {
    pests
        .iter()
        .any(|pest| pest.monitor || pest.spray || pest.gel)
}

pub fn validate_products(products: &[Product]) -> bool {
    products
        .iter()
        .any(|product| !product.used.trim().is_empty())
}

pub fn validate_spaces(spaces: &HashMap<String, bool>) -> bool {
    spaces.values().any(|&selected| selected)
}

const CRITERIA_IDS: &[&str] = &[
    "1.1", "1.2", "2.1", "2.2", "2.3", "2.4", "2.5", "3.1", "3.2", "4.1", "5.1", "6.1", "6.2",
    "7.1", "7.2", "8.1", "9.1", "10.1", "11.1", "12.1", "12.2", "13.1", "14.1", "15.1", "15.2",
    "16.1", "17.1", "18.1", "18.2", "19.1", "20.1", "20.2", "21.1",
];

pub fn validate_criteria(criteria: &HashMap<String, Option<bool>>) -> bool {
    CRITERIA_IDS.iter().all(|id| criteria.contains_key(*id))
}

pub fn validate_required_fields(form_data: &AcceptanceFormData) -> bool {
    !form_data.customer.representative.name.trim().is_empty()
        && !form_data.start_time.trim().is_empty()
        && !form_data.end_time.trim().is_empty()
        && !form_data.comment.trim().is_empty()
}
