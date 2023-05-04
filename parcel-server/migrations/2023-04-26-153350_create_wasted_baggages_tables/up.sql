CREATE TABLE wasted_baggages (
    id VARCHAR NOT NULL PRIMARY KEY,
    qpid_id INTEGER NOT NULL,
    creator_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    created_at TIMESTAMP NOT NULL,
    item_hash INTEGER NOT NULL,
    broken BOOLEAN NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z INTEGER NOT NULL
);

CREATE INDEX wasted_baggages_qpid_id_idx ON wasted_baggages(qpid_id);

CREATE INDEX wasted_baggages_creator_id_idx ON wasted_baggages(creator_id);