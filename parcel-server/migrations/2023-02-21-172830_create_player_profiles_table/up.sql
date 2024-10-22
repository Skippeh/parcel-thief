CREATE TABLE player_profiles (
    account_id VARCHAR PRIMARY KEY,
    banner_id INTEGER NOT NULL,
    num_bronze_medals INTEGER NOT NULL,
    num_bronze_medals_large INTEGER NOT NULL,
    delivered_baggage INTEGER NOT NULL,
    delivery_rank INTEGER NOT NULL,
    delivered_weight INTEGER NOT NULL,
    evaluation_bridge INTEGER NOT NULL,
    evaluation_delivery INTEGER NOT NULL,
    evaluation_safety INTEGER NOT NULL,
    evaluation_speed INTEGER NOT NULL,
    evaluation_service INTEGER NOT NULL,
    num_gold_medals INTEGER NOT NULL,
    num_gold_medals_large INTEGER NOT NULL,
    legend_count INTEGER NOT NULL,
    last_login TIMESTAMP NOT NULL,
    distance_traveled INTEGER NOT NULL,
    -- note: this column's value is u64, but is stored as i64
    music_open_tracks BIGINT NOT NULL,
    name_hash INTEGER NOT NULL,
    num_platinum_medals INTEGER NOT NULL,
    num_platinum_medals_large INTEGER NOT NULL,
    num_likes_received_npc INTEGER NOT NULL,
    num_likes_received_online INTEGER NOT NULL,
    num_rainbow_medals INTEGER NOT NULL,
    num_rainbow_medals_large INTEGER NOT NULL,
    super_legend_count INTEGER NOT NULL,
    num_silver_medals INTEGER NOT NULL,
    num_silver_medals_large INTEGER NOT NULL,
    ss_legend_count INTEGER NOT NULL,
    CONSTRAINT account_id_fkey FOREIGN KEY(account_id) REFERENCES accounts(id) ON DELETE CASCADE ON UPDATE CASCADE
);