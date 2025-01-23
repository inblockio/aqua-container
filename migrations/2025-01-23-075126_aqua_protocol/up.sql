DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'hash') THEN
        CREATE DOMAIN hash AS TEXT
        CHECK (LENGTH(VALUE) <= 64); -- Enforce a maximum length
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'pubkey') THEN
        CREATE DOMAIN pubkey AS TEXT
        CHECK (LENGTH(VALUE) <= 64); -- Enforce a maximum length
    END IF;
END;
$$;


CREATE TABLE "User" (
  "user" pubkey PRIMARY KEY
);

CREATE TABLE "Contract" (
  "hash" hash PRIMARY KEY,
  "latest" TEXT[],
  "sender" pubkey,
  "receiver" pubkey,
  "option" TEXT,
  "reference_count" int
);

CREATE TABLE "Latest" (
  "hash" hash PRIMARY KEY,
  "owner" pubkey
);

CREATE TABLE "Revision" (
  "hash" hash PRIMARY KEY,
  "owner" pubkey,
  "nonce" TEXT,
  "shared" TEXT[],--pubkey[],
  "contract" TEXT[],
  "previous" varchar,
  "children" TEXT,
  "local_timestamp" timestamp,
  "Revision_type" TEXT,
  "Verification_leaves"  TEXT --hash_map
);

CREATE TABLE "Content" (
  "hash" hash PRIMARY KEY,
  "content" TEXT,--utf8_content,
  "reference_count" int
);

CREATE TABLE "FileHash" (
  "hash" hash PRIMARY KEY,
  "file_hash" hash,
  "reference_count" int
);

CREATE TABLE "Link" (
  "hash" hash PRIMARY KEY,
  "link_type" TEXT,
  "link_require_indepth_verification" boolean,
  "link_verification_hash"  TEXT, -- hash_map,
  "reference_count" int
);

CREATE TABLE "Index" (
  "hash"  TEXT, --TEXT [],--hash [],
  "file_hash" hash,
  "uri" TEXT,  -- URLPath_and_Title,
  PRIMARY KEY ("hash", "file_hash")
);

CREATE TABLE "Signature" (
  "hash" hash PRIMARY KEY,
  "signature_digest" TEXT,
  "signature_wallet_address" varchar,
  "signature_type" hash
);

CREATE TABLE "Witness" (
  "hash" hash PRIMARY KEY,
  "Witness_merkle_root" hash
);

CREATE TABLE "WitnessEvent" (
  "Witness_merkle_root" hash PRIMARY KEY,
  "Witness_timestamp" timestamp,
  "Witness_network" TEXT, -- chain_id,
  "Witness_smart_contract_address" hash,
  "Witness_transaction_hash" TEXT, -- tx_hash,
  "Witness_sender_account_address" TEXT -- pubkey
);

CREATE TABLE "MerkleNodes" (
  "node_hash" TEXT,
  "parent_hash" TEXT,
  "height" INTEGER,
  "is_leaf" BOOLEAN,
  "left_child_hash" TEXT,
  "right_child_hash" TEXT
);

CREATE TABLE "AquaForms" (
  "hash" hash PRIMARY KEY,
  "key" TEXT,
  "value" JSONB, -- Replaced 'object' with JSONB.
  "type" TEXT
);

CREATE TABLE "Settings" (
  "user_pub_key" pubkey PRIMARY KEY,
  "cli_pub_key"  pubkey,
  "cli_priv_key" TEXT,-- private_key,
  "Witness_network"  TEXT,-- chain_id,
  "Witness_contract_address" hash,
  "theme" TEXT
);

ALTER TABLE "Latest" ADD FOREIGN KEY ("hash") REFERENCES "User" ("user");

ALTER TABLE "Settings" ADD FOREIGN KEY ("user_pub_key") REFERENCES "User" ("user");

ALTER TABLE "Contract" ADD FOREIGN KEY ("hash") REFERENCES "User" ("user");

ALTER TABLE "Revision" ADD FOREIGN KEY ("hash") REFERENCES "Latest" ("hash");

ALTER TABLE "Signature" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");

ALTER TABLE "Witness" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");

ALTER TABLE "Content" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");

ALTER TABLE "Link" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");

ALTER TABLE "Contract" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");

ALTER TABLE "Index" ADD FOREIGN KEY ("hash") REFERENCES "FileHash" ("hash");

ALTER TABLE "Link" ADD FOREIGN KEY ("link_verification_hash") REFERENCES "Index" ("hash");

ALTER TABLE "Revision" ADD FOREIGN KEY ("hash") REFERENCES "Index" ("file_hash");

ALTER TABLE "Witness" ADD FOREIGN KEY ("Witness_merkle_root") REFERENCES "WitnessEvent" ("Witness_merkle_root");

-- ALTER TABLE "MerkleNodes" ADD FOREIGN KEY ("node_hash") REFERENCES "Witness" ("hash");

-- ALTER TABLE "MerkleNodes" ADD FOREIGN KEY ("node_hash") REFERENCES "WitnessEvent" ("Witness_merkle_root");

-- ALTER TABLE "MerkleNodes" ADD FOREIGN KEY ("node_hash") REFERENCES "MerkleNodes" ("parent_hash");

-- ALTER TABLE "MerkleNodes" ADD FOREIGN KEY ("node_hash") REFERENCES "MerkleNodes" ("left_child_hash");

-- ALTER TABLE "MerkleNodes" ADD FOREIGN KEY ("node_hash") REFERENCES "MerkleNodes" ("right_child_hash");

ALTER TABLE "FileHash" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");

ALTER TABLE "AquaForms" ADD FOREIGN KEY ("hash") REFERENCES "Revision" ("hash");