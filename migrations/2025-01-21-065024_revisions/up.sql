-- Your SQL goes here
-- Add migration script here
CREATE TABLE IF NOT EXISTS revisions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    revision_hash TEXT NOT NULL,
    previous_verification_hash TEXT NOT NULL,
    nonce TEXT NOT NULL,
    local_timestamp TEXT NOT NULL,
    revision_type TEXT NOT NULL,
    file_hash TEXT DEFAULT NULL,
    content TEXT DEFAULT NULL,
    link_type TEXT DEFAULT NULL,
    link_require_indepth_verification BOOLEAN DEFAULT false,
    link_verification_hash TEXT DEFAULT NULL,
    link_uri TEXT DEFAULT NULL,
    signature_data TEXT DEFAULT NULL,
    signature_public_key TEXT DEFAULT NULL,
    signature_wallet_address TEXT DEFAULT NULL,
    signature_type TEXT DEFAULT NULL,
    witness_merkle_root TEXT DEFAULT NULL,
    witness_timestamp TEXT DEFAULT NULL,
    witness_network TEXT DEFAULT NULL,
    witness_smart_contract_address TEXT DEFAULT NULL,
    witness_transaction_hash TEXT DEFAULT NULL,
    witness_sender_account_address TEXT DEFAULT NULL,
    leaves TEXT DEFAULT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);