CREATE TABLE likes (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    time TIMESTAMP NOT NULL,
    from_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    to_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    online_id VARCHAR NOT NULL,
    likes_manual INTEGER NOT NULL,
    likes_auto INTEGER NOT NULL,
    type VARCHAR NOT NULL,
    acknowledged BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX likes_acknowledged_idx ON likes(acknowledged);

CREATE INDEX likes_time_idx ON likes(time);

CREATE INDEX likes_to_id_idx ON likes(to_id);