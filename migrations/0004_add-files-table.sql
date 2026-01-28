-- Migration number: 0004 	 2026-01-28T15:26:50.315Z
CREATE TABLE files (
    id TEXT PRIMARY KEY NOT NULL,
    key TEXT NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted TEXT
);
