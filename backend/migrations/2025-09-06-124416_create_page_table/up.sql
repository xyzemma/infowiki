-- Your SQL goes here
CREATE TABLE page (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at INTEGER,
    text TEXT NOT NULL,
    currentversion INTEGER
)