# aqua-container
Aqua container is a rust implmentation of aqua protocol.It enables data to be signed , witnessed and verified.
This project has  an axum web server (with ssr  web page) and  a solid js frontend to display  the above capabilities.

## Phase 1 - Features:

Functionality milestones Fileupload & Hasher:
1) [DONE] Upload file to Database [Support: doc,ots,pdf,png,jpg, more...]
2) [DONE] Turn file into Aqua-Chain
3) [DONE] Export Aqua-Chain (no signature, no witness) as JSON
4) [DONE] Sign Aqua-Chain with Metamask-Wallet
5) [DONE] Import Aqua-Chain
6) [DONE] Extend existing Aqua-Chain with new Signature 
7) [DONE] Witness Aqua-Chain on Chain
8) [ONGOIN] Allow for choosing witness network from config.
9) [TODO] Make user profile to handle some information ie config, email, payments

UI-Interface:
1) [DONE] Beautify website with CSS
2) [DONE] Make it mobile-responsive

Functionality milestones Verifier: 
1) [DONE] Upload Aqua-Chain for verification
2) [DONE] Verification of Aqua-Chain (print to console)
3) [DONE] Display verification results on the website
4) [DONE] Independent upload for verification. Show verification with detailed verification details.

Pages:
1) [DONE][v1.2] Hasher 
2) [DONE][v1.2] Verifier
3) [ONGOIN] Config (configure Witness, see version of Software etc.)

## Phase 2 - Features:

Multi-User File-Management
1) [DONE] Support multi-user access rights for DB
2) [ONGOING] IntTODOegrate SIWE-OIDC login functionality
3) [] File-Sharing between different accounts (Workflow, share file with wallet (see other wallets registered on server))

## Known Limitation.
1. The metamask siwe-oidc session is not persisted as such api  restart log you out
2.  
## Requirements

1. Rust and `sqlx` (` cargo install sqlx-cli`)
2. node and npm.

## How to run
1. `export DATABASE_URL="sqlite:pages.db"`
   `sqlx database create`
   `sqlx migrate run`
    `cd web && npm i  `
2. `cargo run `
3. `cd web && npm run dev`
4. `http://localhost:3000/` use local host domain for oidc to work.

## How to clear db and set it up again 

```bash
rm pages.db  && export DATABASE_URL="sqlite:pages.db" && sqlx database create && sqlx migrate run
```

## Hot to create a new table 

```bash
sqlx migrate add table_name
```




