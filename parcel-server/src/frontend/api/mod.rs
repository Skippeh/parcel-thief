pub mod endpoints;

use actix_web::web::ServiceConfig;

use endpoints::*;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(auth::auth)
        .service(auth::check_auth)
        .service(auth::steam::steam_callback)
        .service(auth::local::auth_local)
        .service(baggages::list_shared_cargo)
        .service(baggages::list_lost_cargo)
        .service(baggages::list_wasted_cargo)
        .service(accounts::list_accounts)
        .service(accounts::get_frontend_account)
        .service(accounts::set_account_permissions)
        .service(accounts::create_credentials)
        .service(accounts::reset_password);
}
