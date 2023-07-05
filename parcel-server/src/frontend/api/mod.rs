mod endpoints;
pub mod models;

use actix_web::web::ServiceConfig;

use endpoints::*;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(auth::auth);
}
