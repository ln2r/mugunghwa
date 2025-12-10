-- Migration number: 0001 	 2025-12-10T15:32:58.489Z
CREATE TABLE writings (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted TEXT
);
