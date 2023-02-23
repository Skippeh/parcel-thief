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

diesel::joinable!(player_profiles -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    player_profiles,
);
