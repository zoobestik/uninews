-- Create table for mapping UUIDv7 <-> UUIDv5
CREATE TABLE IF NOT EXISTS uuid_mappings
(
    internal_id BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    external_id BLOB UNIQUE      NOT NULL, -- UUIDv5
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    -- metadata
    CONSTRAINT fk_source_id FOREIGN KEY (internal_id) REFERENCES sources (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_external_id ON uuid_mappings (external_id);

-- Create a table for data sources
CREATE TABLE IF NOT EXISTS sources
(
    id         BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    source     TEXT             NOT NULL CHECK (source IN ('atom', 'telegram')),
    created_at TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Create a table for atom-feed additional data
CREATE TABLE IF NOT EXISTS source_atom_details
(
    atom_details_id  BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    url TEXT UNIQUE NOT NULL,
    -- metadata
    CONSTRAINT fk_sources_id FOREIGN KEY (atom_details_id) REFERENCES sources (id) ON DELETE CASCADE
);

-- Create a table for telegram-channel additional data
CREATE TABLE IF NOT EXISTS source_telegram_details
(
    telegram_details_id BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    username            TEXT UNIQUE      NOT NULL,
    -- metadata
    CONSTRAINT fk_sources_id FOREIGN KEY (telegram_details_id) REFERENCES sources (id) ON DELETE CASCADE
);

-- Create an article table
CREATE TABLE IF NOT EXISTS articles
(
    id         BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    parent_id  BLOB             NOT NULL, -- UUIDv7
    created_at TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    -- metadata
    CONSTRAINT fk_sources_id FOREIGN KEY (parent_id) REFERENCES sources (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_articles_parent_id ON articles (parent_id);
