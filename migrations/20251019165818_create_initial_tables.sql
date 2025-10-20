-- Create a user table
CREATE TABLE IF NOT EXISTS users
(
    id            BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    username      TEXT             NOT NULL,
    email         TEXT UNIQUE      NOT NULL,
    password_hash TEXT             NOT NULL,
    created_at    TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at    TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Create table for mapping UUIDv7 <-> UUIDv5
CREATE TABLE IF NOT EXISTS uuid_mappings
(
    internal_id BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    external_id BLOB UNIQUE      NOT NULL, -- UUIDv5
    created_at  TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_external_id ON uuid_mappings (external_id);

-- Create a table for data sources
CREATE TABLE IF NOT EXISTS sources
(
    id         BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    source_id  BLOB             NOT NULL, -- UUIDv5
    source     TEXT             NOT NULL CHECK (source IN ('rss', 'telegram')),
    created_at TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

-- Create an article table
CREATE TABLE IF NOT EXISTS articles
(
    id         BLOB PRIMARY KEY NOT NULL, -- UUIDv7
    parent_id  BLOB             NOT NULL, -- UUIDv7
    source_id  BLOB             NOT NULL, -- UUIDv5
    created_at TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT             NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    -- key
    FOREIGN KEY (source_id) REFERENCES uuid_mappings (external_id)
);

CREATE INDEX IF NOT EXISTS idx_articles_source_id ON articles (source_id);
CREATE INDEX IF NOT EXISTS idx_articles_parent_id ON articles (parent_id);
