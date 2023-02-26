// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Bpchar,
        display_name -> Varchar,
        provider -> Int4,
        provider_id -> Varchar,
        last_login_date -> Timestamp,
    }
}

diesel::table! {
    player_profiles (account_id) {
        account_id -> Bpchar,
        banner_id -> Int4,
        num_bronze_medals -> Int4,
        num_bronze_medals_large -> Int4,
        delivered_baggage -> Int4,
        delivery_rank -> Int4,
        delivered_weight -> Int4,
        evaluation_bridge -> Int4,
        evaluation_delivery -> Int4,
        evaluation_safety -> Int4,
        evaluation_speed -> Int4,
        evaluation_service -> Int4,
        num_gold_medals -> Int4,
        num_gold_medals_large -> Int4,
        legend_count -> Int4,
        last_login -> Timestamp,
        distance_traveled -> Int4,
        music_open_tracks -> Int8,
        name_hash -> Int4,
        num_platinum_medals -> Int4,
        num_platinum_medals_large -> Int4,
        num_likes_received_npc -> Int4,
        num_likes_received_online -> Int4,
        num_rainbow_medals -> Int4,
        num_rainbow_medals_large -> Int4,
        super_legend_count -> Int4,
        num_silver_medals -> Int4,
        num_silver_medals_large -> Int4,
        ss_legend_count -> Int4,
    }
}

diesel::table! {
    qpid_object_baggages (id) {
        id -> Int8,
        object_id -> Varchar,
        creator -> Bpchar,
        item_name_hash -> Int4,
        mission_id -> Int4,
        life -> Int4,
        endurance -> Int4,
        handle -> Int4,
    }
}

diesel::table! {
    qpid_object_bridge_infos (object_id) {
        object_id -> Varchar,
        angle -> Int4,
    }
}

diesel::table! {
    qpid_object_comments (id) {
        id -> Int8,
        object_id -> Varchar,
        writer -> Bpchar,
        likes -> Int8,
        parent_index -> Int2,
        is_deleted -> Bool,
        reference_object -> Varchar,
    }
}

diesel::table! {
    qpid_object_construction_materials (id) {
        id -> Int8,
        object_id -> Varchar,
        contributor -> Bpchar,
        mats_0 -> Int4,
        mats_1 -> Int4,
        mats_2 -> Int4,
        mats_3 -> Int4,
        mats_4 -> Int4,
        mats_5 -> Int4,
        repair_0 -> Int4,
        repair_1 -> Int4,
        repair_2 -> Int4,
        repair_3 -> Int4,
        repair_4 -> Int4,
        repair_5 -> Int4,
        contribute_time -> Timestamp,
    }
}

diesel::table! {
    qpid_object_customize_infos (object_id) {
        object_id -> Varchar,
        customize_param -> Int8,
        customize_color -> Int8,
    }
}

diesel::table! {
    qpid_object_extra_infos (object_id) {
        object_id -> Varchar,
        alternative_qpid_id -> Int4,
    }
}

diesel::table! {
    qpid_object_parking_infos (object_id) {
        object_id -> Varchar,
        location_id -> Int4,
        dynamic_location_id -> Varchar,
        current_qpid_id -> Int4,
        is_parking -> Bool,
    }
}

diesel::table! {
    qpid_object_recycle_materials (id) {
        id -> Int8,
        object_id -> Varchar,
        contributor -> Bpchar,
        mats_0 -> Int4,
        mats_1 -> Int4,
        mats_2 -> Int4,
        mats_3 -> Int4,
        mats_4 -> Int4,
        mats_5 -> Int4,
        recycle_time -> Timestamp,
    }
}

diesel::table! {
    qpid_object_rope_infos (object_id) {
        object_id -> Varchar,
        pitch -> Int4,
        heading -> Int4,
        len -> Int4,
    }
}

diesel::table! {
    qpid_object_stone_infos (object_id) {
        object_id -> Varchar,
        resting_count -> Int4,
    }
}

diesel::table! {
    qpid_object_vehicle_infos (object_id) {
        object_id -> Varchar,
        location_id -> Int4,
        dynamic_location_id -> Varchar,
        current_qpid_id -> Int4,
        is_parking -> Bool,
        is_lost -> Bool,
        is_race -> Bool,
        customize_type -> Int4,
        customize_color -> Int4,
        new_pos_x -> Int4,
        new_pos_y -> Int4,
        new_pos_z -> Int4,
        new_rot_x -> Int4,
        new_rot_y -> Int4,
        new_rot_z -> Int4,
        exponent -> Int4,
    }
}

diesel::table! {
    qpid_objects (id) {
        id -> Varchar,
        creator_id -> Bpchar,
        exponent -> Int4,
        likes -> Int8,
        pos_x -> Int4,
        pos_y -> Int4,
        pos_z -> Int4,
        rot_x -> Int4,
        rot_y -> Int4,
        rot_z -> Int4,
        grid_x -> Int4,
        grid_y -> Int4,
        area_id -> Int4,
        qpid_id -> Int4,
        object_type -> Int4,
        sub_type -> Varchar,
        updated_time -> Timestamp,
    }
}

diesel::joinable!(player_profiles -> accounts (account_id));
diesel::joinable!(qpid_object_baggages -> accounts (creator));
diesel::joinable!(qpid_object_baggages -> qpid_objects (object_id));
diesel::joinable!(qpid_object_bridge_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_object_comments -> accounts (writer));
diesel::joinable!(qpid_object_comments -> qpid_objects (object_id));
diesel::joinable!(qpid_object_construction_materials -> accounts (contributor));
diesel::joinable!(qpid_object_construction_materials -> qpid_objects (object_id));
diesel::joinable!(qpid_object_customize_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_object_extra_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_object_parking_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_object_recycle_materials -> accounts (contributor));
diesel::joinable!(qpid_object_recycle_materials -> qpid_objects (object_id));
diesel::joinable!(qpid_object_rope_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_object_stone_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_object_vehicle_infos -> qpid_objects (object_id));
diesel::joinable!(qpid_objects -> accounts (creator_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    player_profiles,
    qpid_object_baggages,
    qpid_object_bridge_infos,
    qpid_object_comments,
    qpid_object_construction_materials,
    qpid_object_customize_infos,
    qpid_object_extra_infos,
    qpid_object_parking_infos,
    qpid_object_recycle_materials,
    qpid_object_rope_infos,
    qpid_object_stone_infos,
    qpid_object_vehicle_infos,
    qpid_objects,
);
