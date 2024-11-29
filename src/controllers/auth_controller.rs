use crate::auth::SiweRequest;
use crate::db::siwe::delete_siwe_session_by_nonce;
use crate::db::siwe::fetch_siwe_data;
use crate::db::siwe::fetch_siwe_session_by_nonce;
use crate::db::siwe::insert_siwe_data;
use crate::Db;
use axum::{extract::State, http::StatusCode, Form, Json};
use ethers::types::Signature;
use ethers_core::k256::schnorr::SigningKey;
use ethers_core::types::Address;
use ethers_core::utils::keccak256;
use hex::FromHex;
use serde::{Deserialize, Serialize};
use sha3::Digest;
use sha3::Keccak256;
use siwe::{Message, VerificationOpts};
use std::ops::Deref;
use std::{fmt, str::FromStr};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::auth::{SiweError, SiweNonceRequest, SiweResponse, SiweSession};

pub async fn siwe_sign_in(
    State(server_database): State<Db>,
    Form(payload): Form<SiweRequest>,
) -> (StatusCode, Json<SiweResponse>) {
    let mut log_data: Vec<String> = Vec::new();

    // Log the received message and signature
    log_data.push(format!("Received SIWE message: {}", payload.message));
    log_data.push(format!("Received signature: {}", payload.signature));

    // Verify the SIWE message
    match verify_siwe_message(payload.message, payload.signature).await {
        Ok(siwe_session) => {
            let mut conn = match server_database.pool.get() {
                Ok(connection) => connection,
                Err(e) => {
                    error!("Failed to get database connection: {}", e);
                    log_data.push("Failed to get database connection".to_string());
                    let res = SiweResponse {
                        logs: log_data,
                        success: false,
                        session: None,
                    };
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
                }
            };

            let res = insert_siwe_data(siwe_session.clone(), &mut conn);
            if res.is_err() {
                let e = res.err().unwrap();

                error!("Error occured inserting session into db: {:#?}", e);
                log_data.push("Failed to create sign in session".to_string());
                let res = SiweResponse {
                    logs: log_data,
                    success: false,
                    session: None,
                };

                return (StatusCode::BAD_REQUEST, Json(res));
            }

            log_data.push(format!(
                "SIWE sign-in successful for address: {}",
                siwe_session.address.clone()
            ));
            let res: SiweResponse = SiweResponse {
                logs: log_data.clone(),
                success: true,
                session: Some(siwe_session.clone()),
            };
            return (StatusCode::OK, Json(res));
        }
        Err(e) => {
            let error_message = format!("SIWE sign-in failed: {:?}", e);
            log_data.push(error_message.clone());
            error!("Error --> {}", error_message);

            let res = SiweResponse {
                logs: log_data,
                success: false,
                session: None,
            };

            (StatusCode::BAD_REQUEST, Json(res))
        }
    }
}

// Async verification function using tokio
pub async fn verify_siwe_message(
    message: String,
    signature: String,
) -> Result<SiweSession, SiweError> {
    // Hash the message according to Ethereum's EIP-191 standard
    // Prepend "\x19Ethereum Signed Message:\n" + length of the message
    let eth_signed_message = format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
    // // let eth_signed_message = message;
    let message_hash = keccak256(eth_signed_message.as_bytes());

    // // Parse the signature
    let sig = Signature::from_str(&signature).map_err(|_| SiweError::InvalidSignature)?;

    // // Recover the Ethereum address from the signature and message hash
    let recovered_address = sig
        .recover(message_hash)
        .map_err(|_| SiweError::VerificationFailed)?;

    // SIWE Message
    let _message = Message::from_str(&message).unwrap();
    let expiration_time = _message.clone().expiration_time;

    // // Expected address (this could be dynamically provided or extracted from the SIWE message)
    // let expected_address = Address::from_str(&user_address).unwrap();
    // // Compare the recovered address with the expected address
    if recovered_address != ethers::types::H160(_message.address) {
        return Err(SiweError::AddressMismatch);
    }

    let sig = <[u8; 65]>::from_hex(format!(r#"{}"#, signature)).unwrap();

    let verification_opts = VerificationOpts {
        domain: Some("localhost:5173".parse().unwrap()),
        // We can adjust the fields once we create database sessions. There is a table already
        // nonce: Some("kEWepMt9knR6lWJ6A".into()),
        // timestamp: Some(OffsetDateTime::parse("2021-12-08T00:00:00Z", &Rfc3339).unwrap()),
        ..Default::default()
    };

    let msg_verification = _message.verify(&sig, &verification_opts).await;

    // Confirm whether the message verification is successful
    if msg_verification.is_ok() {
        info!("Message is Okay and recovered address is correct");
        let siwe_session = SiweSession {
            address: format!("{:?}", recovered_address),
            nonce: _message.nonce.to_string(),
            issued_at: _message.issued_at.to_string(),
            expiration_time: _message.expiration_time.map(|time| time.to_string()),
        };

        // Ok(format!("{:?}", recovered_address))
        Ok(siwe_session)
    } else {
        error!("Quack Message");
        Err(SiweError::MessageVerificationFailed)
    }
}

pub async fn fetch_nonce_session(
    State(server_database): State<Db>,
    Form(payload): Form<SiweNonceRequest>,
) -> (StatusCode, Json<Option<SiweSession>>) {
    let mut log_data: Vec<String> = Vec::new();

    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            log_data.push("Failed to get database connection".to_string());
            let res = SiweResponse {
                logs: log_data,
                success: false,
                session: None,
            };
            println!("Error Fetching connection {:#?}", res);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    let session = fetch_siwe_session_by_nonce(&payload.nonce, &mut conn);

    if session.is_err() {
        return (StatusCode::NOT_FOUND, Json(None));
    }

    let _session = session.unwrap();

    let siwe = SiweSession {
        address: _session.address.clone(),
        nonce: _session.address.clone(),
        issued_at: _session.issued_at.clone(),
        expiration_time: _session.expiration_time.clone(),
    };
    return (StatusCode::OK, Json(Some(siwe)));
}


pub async fn session_logou_by_nonce(
    State(server_database): State<Db>,
    Form(payload): Form<SiweNonceRequest>,
) -> (StatusCode, Json<SiweResponse>) {
    let mut log_data: Vec<String> = Vec::new();

    // Get database connection
    let mut conn = match server_database.pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            log_data.push("Failed to get database connection".to_string());
            let res = SiweResponse {
                logs: log_data,
                success: false,
                session: None,
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(res));
        }
    };

    // Call helper function to delete session
    match delete_siwe_session_by_nonce(&payload.nonce, &mut conn) {
        Ok(affected_rows) => {
            if affected_rows == 0 {
                log_data.push("No session found to delete".to_string());
                let res = SiweResponse {
                    logs: log_data,
                    success: false,
                    session: None,
                };
                return (StatusCode::NOT_FOUND, Json(res));
            }

            log_data.push(format!("Successfully deleted {} session(s)", affected_rows));
            let res = SiweResponse {
                logs: log_data,
                success: true,
                session: None,
            };
            (StatusCode::OK, Json(res))
        }
        Err(e) => {
            error!("Failed to delete session: {}", e);
            log_data.push(format!("Failed to delete session: {}", e));
            let res = SiweResponse {
                logs: log_data,
                success: false,
                session: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(res))
        }
    }
}
