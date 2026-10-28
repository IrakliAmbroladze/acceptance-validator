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
    products.iter().any(|product| product.name.trim() != "")
}
