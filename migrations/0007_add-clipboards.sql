-- Migration number: 0007 	 2026-09-22T14:25:06.042Z
CREATE TABLE clipboards (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    description TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted TEXT
);
