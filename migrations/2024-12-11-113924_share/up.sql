-- Your SQL goes here
CREATE TABLE IF NOT EXISTS share_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name TEXT NOT NULL,
    identifier TEXT NOT NULL,
    created_time TEXT NOT NULL
);