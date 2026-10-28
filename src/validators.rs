use std::collections::HashMap;

use crate::models::{Pest, Product, Services};

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
