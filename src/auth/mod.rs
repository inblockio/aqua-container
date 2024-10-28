use axum::{extract::State, http::StatusCode, Form, Json};
use ethers::types::Signature;
use ethers_core::k256::schnorr::SigningKey;
use ethers_core::types::Address;
use ethers_core::utils::keccak256;
use serde::{Deserialize, Serialize};
use sha3::Keccak256;
use siwe::{Message, VerificationOpts};
use std::{fmt, str::FromStr};
use crate::Db;
use hex::FromHex;
use sha3::Digest;
use tokio::sync::Mutex;
use tracing::{error, info};

#[derive(Deserialize)]
pub struct SiweRequest {
    pub message: String,   // The SIWE message
    pub signature: String, // The Ethereum signature
}

#[derive(Serialize)]
pub struct SiweResponse {
    pub logs: Vec<String>,
    pub success: bool,
    pub address: Option<String>,
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
    State(state): State<Db>,
    Form(payload): Form<SiweRequest>,
) -> (StatusCode, Json<SiweResponse>) {
    let mut log_data: Vec<String> = Vec::new();

    // Log the received message and signature
    log_data.push(format!("Received SIWE message: {}", payload.message));
    log_data.push(format!("Received signature: {}", payload.signature));

    // Verify the SIWE message
    match verify_siwe_message(payload.message, payload.signature).await {
        Ok(address) => {
            log_data.push(format!("SIWE sign-in successful for address: {}", address));
            info!("SIWE sign-in successful for address: {}", address);

            let res = SiweResponse {
                logs: log_data,
                success: true,
                address: Some(address),
            };
            (StatusCode::OK, Json(res))
        }
        Err(e) => {
            let error_message = format!("SIWE sign-in failed: {:?}", e);
            log_data.push(error_message.clone());
            error!("Error --> {}", error_message);

            let res = SiweResponse {
                logs: log_data,
                success: false,
                address: None,
            };

            (StatusCode::BAD_REQUEST, Json(res))
        }
    }
}

// Async verification function using tokio
pub async fn verify_siwe_message(message: String, signature: String) -> Result<String, SiweError> {

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

    // // Expected address (this could be dynamically provided or extracted from the SIWE message)
    // let expected_address = Address::from_str(&user_address).unwrap();
    // // Compare the recovered address with the expected address
    if recovered_address != ethers::types::H160(_message.address) {
        return Err(SiweError::AddressMismatch);
    }

    let sig = <[u8; 65]>::from_hex(format!(r#"{}"#, signature)).unwrap();

    let verification_opts = VerificationOpts {
        domain: Some("localhost:3000".parse().unwrap()),
        // We can adjust the fields once we create database sessions. There is a table already
        // nonce: Some("kEWepMt9knR6lWJ6A".into()),
        // timestamp: Some(OffsetDateTime::parse("2021-12-08T00:00:00Z", &Rfc3339).unwrap()),
        ..Default::default()
    };

    let msg_verification = _message.verify(&sig, &verification_opts).await;

    // Confirm whether the message verification is successful
    if msg_verification.is_ok() {
        info!("Message is Okay and recovered address is correct");
        Ok(format!("{:?}", recovered_address))
    } else {
        error!("Quack Message");
        Err(SiweError::MessageVerificationFailed)
    }
}