-- Your SQL goes here
-- Add migration script here
CREATE TABLE IF NOT EXISTS aqua_chain (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     file_hash TEXT NOT NULL,
                                     file_name TEXT NOT NULL,
                                     revisions TEXT NOT NULL,
                                     file_content TEXT NOT NULL,
                                     owner TEXT NOT NULL,
                                     mode TEXT NOT NULL,
                                     share_code TEXT DEFAULT NULL,
                                     is_shared BOOLEAN DEFAULT false NOT NULL,
                                     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);