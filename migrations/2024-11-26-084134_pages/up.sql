-- Your SQL goes here
-- Add migration script here
CREATE TABLE IF NOT EXISTS pages (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     name TEXT NOT NULL,
                                     extension TEXT NOT NULL,
                                     page_data TEXT NOT NULL,
                                     owner TEXT NOT NULL,
                                     mode TEXT NOT NULL,
                                     created_at TEXT NOT NULL,
                                     is_shared BOOLEAN DEFAULT false NOT NULL
                                    --  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);