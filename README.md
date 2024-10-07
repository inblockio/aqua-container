# aqua-container
A rust implementation of the Aqua protocol.

Features:

Functionality milestones Fileupload & Hasher:
1) [done] Upload file to Database
2) [done] Turn file into Aqua-Chain 
3) [done] Export Aqua-Chain (no signature, no witness) as JSON
4) [done] Sign Aqua-Chain with Metamask-Wallet
5) [todo] Import Aqua-Chain
6) [done] Extend existing Aqua-Chain with new Signature 
7) [todo] Witness Aqua-Chain on Chain

UI-Interface:
1) [done] Beautify website with CSS
2) [todo] Make it mobile-responsive

Functionality milestones Verifier: 
1) [todo] Upload Aqua-Chain for verification
2) [todo] Verification of Aqua-Chain (print to console)
3) [todo] Display verification results on the website

Pages:
1) [in progress] Hasher
2) [todo] Verifier
3) [todo] Config (configure Witness, see version of Software etc.)



## Requirements

Rust and `sqlx` (` cargo install sqlx-cli`)

## How to run



```bash
export DATABASE_URL="sqlite:pages.db"
sqlx database create
sqlx migrate run
cargo run
```

