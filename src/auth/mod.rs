use crate::db::siwe::fetch_siwe_data;
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
use std::{fmt, str::FromStr};
use tokio::sync::Mutex;
use tracing::{error, info};
use crate::db::siwe::{insert_siwe_data };
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
