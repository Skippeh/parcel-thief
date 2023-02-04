mod add_missions;
pub mod auth;
mod create_object;
mod create_road;
mod delete_missions;
mod delete_object;
mod devote_highway_resources;
mod find_missions;
mod find_qpid_objects;
mod get_highway_resources;
mod get_like_history;
mod get_ordered_missions;
mod get_player_profile;
mod get_player_ranking_records;
mod get_qpid_objects;
mod get_ranking_schedules;
mod get_relationships;
mod get_road_data;
mod get_version;
mod get_wasted_baggages;
mod lookup;
mod reverse_lookup;
mod send_like;
mod set_construction_materials;
mod set_mission_progress;
mod set_player_profile;
mod set_strand;

use actix_web::web::ServiceConfig;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(devote_highway_resources::devote_highway_resources)
        .service(get_highway_resources::get_highway_resources)
        .service(get_like_history::get_like_history)
        .service(get_player_profile::get_player_profile)
        .service(get_ranking_schedules::get_ranking_schedules)
        .service(get_version::get_version)
        .service(lookup::lookup)
        .service(reverse_lookup::reverse_lookup)
        .service(send_like::send_like)
        .service(set_player_profile::set_player_profile)
        .service(get_relationships::get_relationships)
        .service(delete_missions::delete_missions)
        .service(get_road_data::get_road_data)
        .service(delete_object::delete_object)
        .service(add_missions::add_missions)
        .service(find_missions::find_missions)
        .service(get_ordered_missions::get_ordered_missions)
        .service(set_mission_progress::set_mission_progress)
        .service(set_strand::set_strand)
        .service(create_object::create_object)
        .service(set_construction_materials::set_construction_materials)
        .service(create_road::create_road)
        .service(get_qpid_objects::get_qpid_objects)
        .service(find_qpid_objects::find_qpid_objects)
        .service(get_wasted_baggages::get_wasted_baggages)
        .service(get_player_ranking_records::get_player_ranking_records);
}
