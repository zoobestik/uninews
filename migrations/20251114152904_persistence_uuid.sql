-- Create a table with persistence uuid groups
CREATE TABLE persistence_uuid
(
    internal_id BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    group_type  TEXT             NOT NULL UNIQUE,
    CHECK (group_type IN ('source_telegram', 'source_atom', 'news'))
);
