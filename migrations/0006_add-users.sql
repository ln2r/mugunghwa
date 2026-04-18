-- Migration number: 0006 	 2026-04-11T11:47:43.274Z
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    avatar TEXT,
    url TEXT,
    provider_id TEXT NOT NULL UNIQUE,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted TEXT
);
