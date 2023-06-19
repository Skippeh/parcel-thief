-- Set timestamp accuracy to milliseconds
ALTER TABLE
    accounts
ALTER COLUMN
    last_login_date TYPE TIMESTAMP(3);

ALTER TABLE
    player_profiles
ALTER COLUMN
    last_login TYPE TIMESTAMP(3);

ALTER TABLE
    qpid_objects
ALTER COLUMN
    updated_time TYPE TIMESTAMP(3);

ALTER TABLE
    qpid_object_construction_materials
ALTER COLUMN
    contribute_time TYPE TIMESTAMP(3);

ALTER TABLE
    qpid_object_recycle_materials
ALTER COLUMN
    recycle_time TYPE TIMESTAMP(3);

ALTER TABLE
    missions
ALTER COLUMN
    registered_time TYPE TIMESTAMP(3);

ALTER TABLE
    missions
ALTER COLUMN
    expiration_time TYPE TIMESTAMP(3);

ALTER TABLE
    likes
ALTER COLUMN
    time TYPE TIMESTAMP(3);

ALTER TABLE
    devoted_highway_resources
ALTER COLUMN
    time TYPE TIMESTAMP(3);

ALTER TABLE
    wasted_baggages
ALTER COLUMN
    created_at TYPE TIMESTAMP(3);

ALTER TABLE
    account_histories
ALTER COLUMN
    encountered_at TYPE TIMESTAMP(3);

ALTER TABLE
    account_strand_contracts
ALTER COLUMN
    created_at TYPE TIMESTAMP(3);

ALTER TABLE
    roads
ALTER COLUMN
    created_at TYPE TIMESTAMP(3);