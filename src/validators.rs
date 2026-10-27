use crate::models::{Pest, Services};

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
