CREATE TABLE IF NOT EXISTS siwe_sessions (
    id SERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    nonce TEXT NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL, -- Using TIMESTAMPTZ for timestamp with time zone
    expiration_time TIMESTAMPTZ    -- TIMESTAMPTZ to handle expiration timestamps with time zone
);
