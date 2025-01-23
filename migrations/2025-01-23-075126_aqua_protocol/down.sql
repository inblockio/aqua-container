-- This file should undo anything in `up.sql`
-- Reverse order of creation to ensure dependencies are resolved correctly.

-- Remove foreign key constraints first
ALTER TABLE "Latest" DROP CONSTRAINT IF EXISTS "Latest_hash_fkey";
ALTER TABLE "Settings" DROP CONSTRAINT IF EXISTS "Settings_user_pub_key_fkey";
ALTER TABLE "Contract" DROP CONSTRAINT IF EXISTS "Contract_hash_fkey";
ALTER TABLE "Contract" DROP CONSTRAINT IF EXISTS "Contract_hash_fkey1";
ALTER TABLE "Revision" DROP CONSTRAINT IF EXISTS "Revision_hash_fkey";
ALTER TABLE "Signature" DROP CONSTRAINT IF EXISTS "Signature_hash_fkey";
ALTER TABLE "Witness" DROP CONSTRAINT IF EXISTS "Witness_hash_fkey";
ALTER TABLE "Content" DROP CONSTRAINT IF EXISTS "Content_hash_fkey";
ALTER TABLE "Link" DROP CONSTRAINT IF EXISTS "Link_hash_fkey";
ALTER TABLE "MerkleNodes" DROP CONSTRAINT IF EXISTS "MerkleNodes_node_hash_fkey";
ALTER TABLE "FileHash" DROP CONSTRAINT IF EXISTS "FileHash_hash_fkey";
ALTER TABLE "AquaForms" DROP CONSTRAINT IF EXISTS "AquaForms_hash_fkey";

-- Drop tables in reverse order of their dependencies
DROP TABLE IF EXISTS "MerkleNodes";
DROP TABLE IF EXISTS "WitnessEvent";
DROP TABLE IF EXISTS "Witness";
DROP TABLE IF EXISTS "Signature";
DROP TABLE IF EXISTS "Index";
DROP TABLE IF EXISTS "Link";
DROP TABLE IF EXISTS "FileHash";
DROP TABLE IF EXISTS "Content";
DROP TABLE IF EXISTS "Revision";
DROP TABLE IF EXISTS "Contract";
DROP TABLE IF EXISTS "Latest";
DROP TABLE IF EXISTS "Settings";
DROP TABLE IF EXISTS "AquaForms";
DROP TABLE IF EXISTS "User";

-- Drop custom domains (hash and pubkey)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'pubkey') THEN
        DROP DOMAIN pubkey;
    END IF;

    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'hash') THEN
        DROP DOMAIN hash;
    END IF;
END;
$$;
