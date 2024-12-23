# aqua-container

Aqua container is a rust implmentation of aqua protocol.It enables data to be signed , witnessed and verified.
This project has an axum web server (with ssr web page) and a solid js frontend to display the above capabilities.

## Phase 1 - Features:

Functionality milestones Fileupload & Hasher:

1) [DONE] Upload file to Database [Support: doc,ots,pdf,png,jpg, more...]
2) [DONE] Turn file into Aqua-Chain
3) [DONE] Export Aqua-Chain (no signature, no witness) as JSON
4) [DONE] Sign Aqua-Chain with Metamask-Wallet
5) [DONE] Import Aqua-Chain
6) [DONE] Extend existing Aqua-Chain with new Signature
7) [DONE] Witness Aqua-Chain on Chain
8) [DONE] Allow for choosing witness network from config.
9) [DONE] Make user profile to handle some information i.e. config, email, payments

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
3) [DONE] Config (configure Witness, see version of Software etc.)

## Phase 2 - Features:

Multi-User File-Management

1) [DONE] Support multi-user access rights for DB
2) [Done] Integrate SIWE-OIDC login functionality
3) [Done] File-Sharing between different accounts (Via Share-Links)

## Known Limitation.
 - File double in size when a new revision is added, this is a protocol v1.2 limitation and is resolved in v1.3
 - File linking is not implemented in this version
 - Support for the Guardian is not provided in this version (use other prototype implementation https://github.com/inblockio/aqua-PKC)

## Disclaimer
This is a prototype software in active development. 
Use at your own risk.

# Running the Rust build locally
## Requirements

1. Rust and `sqlx` (` cargo install sqlx-cli`)
2. node and npm.

## How to run

1. `export DATABASE_URL="sqlite:users.db"`
   `sqlx database create`
   `sqlx migrate run`
   `cd web && npm i  `
2. `cargo run `
3. `cd web && npm run dev`
4. `http://localhost:5173/` use local host domain for oidc to work.

## How to run with Docker-Compose

### requirements:

1. docker compose
2. docker

### Run:

```bash
docker compose up
```

### Update:
docker compose checks the lastest version on every startup

### Running a local image

Run with local changes:

```
#    image: ghcr.io/inblockio/aqua-container:github-action
    build:
      context: .
      dockerfile: actionfiles/aqua_container/dockerfile/Dockerfile
```

uncomment the build section and add a '#' to the image config

start:

```
docker compose up --build
```

## Good to know.

1. The project uses rust nighly.Use `rustup toolchain install nightly`  or check the rust documentation on switching the
   channel.
2. Use the latest verion of node and npm.

### run with existing nginx proxy and acme

just remove everything from the compose file except the aqua-container section. If the nginx is in a another compose
file, make sure that the nginx can reach the aqua container via network. see https://stackoverflow.com/a/38089080

## Config

The configuration template can be found in .env.template. Please copy this file and make your changes.
```cp .env.template .env```

Must be configured for the application to run:

configure two domains and your e-mail address for the let's encrypt certificate before starting the container.
 - BACKEND_URL= sub.domain.com
 - FRONTEND_URL=sub-api.domain.com
 - SSL_EMAIL= your-email@mail.com

## Limit

### upload limit

Currently, our NGINX has a limit of 100 MB. If this needs to be increased, the Dockerfile in the
repository https://github.com/inblockio/nginx-proxy must be adjusted. To apply the changes, wait briefly (~1 minute) and
pull the latest image using docker compose pull.
