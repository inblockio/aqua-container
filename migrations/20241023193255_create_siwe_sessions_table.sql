CREATE TABLE IF NOT EXISTS siwe_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    nonce TEXT NOT NULL,
    issued_at TEXT NOT NULL,
    expiration_time TEXT
);