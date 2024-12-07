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
use sqlx::query_as;
use std::{fmt, str::FromStr};
use tokio::sync::Mutex;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct SiweRequest {
    pub message: String,   // The SIWE message
    pub signature: String, // The Ethereum signature
}

#[derive(Debug, Serialize, Clone)]
pub struct SiweSession {
    pub address: String,
    pub nonce: String,
    pub issued_at: String,
    pub expiration_time: Option<String>,
}

#[derive(Deserialize)]
pub struct SiweNonceRequest {
    pub nonce: String,
}

#[derive(Serialize)]
pub struct SiweResponse {
    pub logs: Vec<String>,
    pub success: bool,
    pub session: Option<SiweSession>,
}

#[derive(Debug)]
pub enum SiweError {
    InvalidSignature,
    VerificationFailed,
    AddressMismatch,
    InvalidMessage,
    MessageVerificationFailed,
}

impl fmt::Display for SiweError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SiweError::InvalidSignature => write!(f, "Invalid signature"),
            SiweError::VerificationFailed => write!(f, "Failed to verify SIWE message"),
            SiweError::AddressMismatch => write!(f, "Ethereum address mismatch"),
            SiweError::InvalidMessage => write!(f, "Invalid SIWE Message"),
            SiweError::MessageVerificationFailed => write!(f, "Message verification failed"),
        }
    }
}

impl std::error::Error for SiweError {}

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
            // Insert the session into db
            match sqlx::query!(
                r#"
                INSERT INTO siwe_sessions (address, nonce, issued_at, expiration_time)
                VALUES (?, ?, ?, ?)
                RETURNING id
                "#,
                siwe_session.address,
                siwe_session.nonce,
                siwe_session.issued_at,
                siwe_session.expiration_time
            )
            .fetch_one(&server_database.sqliteDb)
            .await
            {
                Ok(_) => {
                    log_data.push(format!(
                        "SIWE sign-in successful for address: {}",
                        siwe_session.address
                    ));
                    let res: SiweResponse = SiweResponse {
                        logs: log_data.clone(),
                        success: true,
                        session: Some(siwe_session.clone()),
                    };
                    (StatusCode::OK, Json(res))
                }
                Err(e) => {
                    error!("Error occured inserting session into db: {:#?}", e);
                    log_data.push("Failed to create sign in session".to_string());
                    let res = SiweResponse {
                        logs: log_data,
                        success: false,
                        session: None,
                    };

                    (StatusCode::BAD_REQUEST, Json(res))
                }
            }
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
    // Query the database for a session with the given nonce
    tracing::info!("Nonce to fetch for: {}", payload.nonce);
    let session = query_as!(
        SiweSession,
        "SELECT address, nonce, issued_at, expiration_time FROM siwe_sessions WHERE nonce = ?",
        payload.nonce
    )
    .fetch_one(&server_database.sqliteDb)
    .await;

    match session {
        Ok(s) => (StatusCode::OK, Json(Some(s))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(None), // No session found
        ),
    }
}