use crate::models::Services;

pub fn validate_services(services: &Services) -> bool {
    services.disinsection
        || services.deratization
        || services.disinfection
        || services.subcontractor_prevention
}
