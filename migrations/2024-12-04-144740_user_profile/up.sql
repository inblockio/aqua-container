-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    chain TEXT NOT NULL,
    theme TEXT NOT NULL,
    contract_address TEXT NOT NULL,
    file_mode TEXT NOT NULL,
    domain_name TEXT NOT NULL
);