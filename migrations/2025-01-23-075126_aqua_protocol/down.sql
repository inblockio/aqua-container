-- This file should undo anything in `up.sql`
-- Reverse order of creation to ensure dependencies are resolved correctly.

-- Drop tables with foreign key dependencies first.
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

-- Drop custom domains (hash and pubkey).
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
