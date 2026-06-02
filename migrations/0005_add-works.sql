-- Migration number: 0005 	 2026-04-05T15:38:33.960Z
CREATE TABLE works (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    url TEXT,
    stacks TEXT,
    created TEXT NOT NULL,
    updated TEXT NOT NULL,
    deleted TEXT
);
