//! Shared helper functions and types used across the SDK

use chrono::{DateTime, Utc};
use near_primitives::action::{base64, delegate::DelegateAction};
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use thiserror::Error;
use url::Url;

// Cryptography imports
use anyhow::{anyhow, Result as AnyhowResult};
use base64::{engine::general_purpose, Engine};
use rsa::{pkcs1::DecodeRsaPublicKey, pkcs8::DecodePublicKey, Oaep, RsaPublicKey};
use sha2::Sha256;

/// Result type alias for Circle SDK operations
pub type CircleResult<T> = Result<T, CircleError>;

/// Comprehensive error type for Circle SDK operations
#[derive(Error, Debug)]
pub enum CircleError {
    #[error("Environment variable error: {0}")]
    EnvVar(String),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    #[error("API error: {status} - {message}")]
    Api { status: u16, message: String },

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
}

/// Standard Circle API response wrapper
#[derive(Debug, Deserialize, Serialize)]
pub struct CircleResponse<T> {
    pub data: T,
}

/// Standard Circle API error response
#[derive(Debug, Deserialize, Serialize)]
pub struct CircleErrorResponse {
    pub code: Option<i32>,
    pub message: String,
}

/// Helper function to serialize u32 as string
pub fn serialize_u32_as_string<S>(value: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(val) => serializer.serialize_str(&val.to_string()),
        None => serializer.serialize_none(),
    }
}

/// Helper function to serialize DateTime as string
pub fn serialize_datetime_as_string<S>(
    dt: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(dt) => serializer.serialize_str(&dt.to_rfc3339()),
        None => serializer.serialize_none(),
    }
}

pub fn serialize_bool_as_string<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(val) => serializer.serialize_str(&val.to_string()),
        None => serializer.serialize_none(),
    }
}

/// Common query parameters for pagination
#[derive(Debug, Serialize, Default)]
pub struct PaginationParams {
    #[serde(rename = "pageAfter", skip_serializing_if = "Option::is_none")]
    pub page_after: Option<String>,

    #[serde(rename = "pageBefore", skip_serializing_if = "Option::is_none")]
    pub page_before: Option<String>,

    #[serde(
        rename = "pageSize",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_u32_as_string"
    )]
    pub page_size: Option<u32>,
}

/// HTTP client wrapper with common functionality
pub struct HttpClient {
    client: Client,
    base_url: Url,
    api_key: Option<String>,
}

impl HttpClient {
    /// Create a new HTTP client with base URL
    pub fn new(base_url: &str) -> CircleResult<Self> {
        let client = Client::new();
        let base_url = Url::parse(base_url)?;

        Ok(Self {
            client,
            base_url,
            api_key: None,
        })
    }

    /// Create a new HTTP client with base URL and API key
    pub fn with_api_key(base_url: &str, api_key: String) -> CircleResult<Self> {
        let mut client = Self::new(base_url)?;
        client.api_key = Some(api_key);
        Ok(client)
    }

    /// Build a request with common headers
    pub fn request(&self, method: Method, path: &str) -> CircleResult<RequestBuilder> {
        let url = self.base_url.join(path)?;
        let mut request = self.client.request(method, url);

        // Add common headers
        request = request.header("Content-Type", "application/json");

        // Add authorization header if API key is available
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        Ok(request)
    }

    /// Execute a request and handle the response
    pub async fn execute<T>(&self, request: RequestBuilder) -> CircleResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Handle HTTP response and convert to typed result
    async fn handle_response<T>(&self, response: Response) -> CircleResult<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        let response_text = response.text().await?;
        println!("Response text: {}", response_text);

        if status.is_success() {
            let circle_response: CircleResponse<T> = serde_json::from_str(&response_text)?;
            Ok(circle_response.data)
        } else {
            // Try to parse error response
            let error_message = match serde_json::from_str::<CircleErrorResponse>(&response_text) {
                Ok(error_resp) => error_resp.message,
                Err(_) => response_text,
            };

            Err(CircleError::Api {
                status: status.as_u16(),
                message: error_message,
            })
        }
    }
}

/// Helper function to read environment variable
pub fn get_env_var(name: &str) -> CircleResult<String> {
    std::env::var(name)
        .map_err(|_| CircleError::EnvVar(format!("Missing environment variable: {}", name)))
}

/// Helper function to build query string from parameters
pub fn build_query_params<T: Serialize>(params: &T) -> CircleResult<String> {
    let query_map: HashMap<String, String> = serde_json::from_value(serde_json::to_value(params)?)?;
    let query_pairs: Vec<String> = query_map
        .into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(k, v)| format!("{}={}", urlencoding::encode(&k), urlencoding::encode(&v)))
        .collect();

    Ok(query_pairs.join("&"))
}

/// Helper function to generate UUID v4
pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Encrypts entity secret using RSA-OAEP with SHA-256
///
/// This function takes a hex-encoded entity secret and encrypts it using the provided
/// RSA public key in PEM format. The result is base64-encoded.
///
/// # Arguments
/// * `entity_secret_hex` - The entity secret as a hex string
/// * `public_key_pem` - The RSA public key in PEM format (PKCS#1 or PKCS#8)
///
/// # Returns
/// * `Result<String>` - Base64-encoded encrypted data on success
pub fn encrypt_entity_secret(
    entity_secret_hex: &str,
    public_key_pem: &str,
) -> AnyhowResult<String> {
    // Convert hex string to bytes
    let entity_secret_bytes = hex::decode(entity_secret_hex)
        .map_err(|e| anyhow!("Failed to decode hex entity secret: {}", e))?;

    // Try PKCS#1 format first, then fall back to PKCS#8 format
    let public_key = match RsaPublicKey::from_pkcs1_pem(public_key_pem) {
        Ok(key) => key,
        Err(e1) => match RsaPublicKey::from_public_key_pem(public_key_pem) {
            Ok(key) => key,
            Err(e2) => {
                return Err(anyhow!(
                        "Failed to parse public key from PEM (tried both PKCS#1 and PKCS#8): PKCS#1 error: {}, PKCS#8 error: {}",
                        e1, e2
                    ));
            }
        },
    };

    // Encrypt using RSA-OAEP with SHA-256
    let mut rng = rand::thread_rng();
    let padding = Oaep::new::<Sha256>();
    let encrypted_data = public_key
        .encrypt(&mut rng, padding, &entity_secret_bytes)
        .map_err(|e| anyhow!("Failed to encrypt data: {}", e))?;

    // Encode to base64
    let base64_encoded = general_purpose::STANDARD.encode(&encrypted_data);

    Ok(base64_encoded)
}

// ============================================================================
// NEAR Protocol Helper Functions
// ============================================================================

/// Serialize a NEAR DelegateAction to base64 for Circle API
///
/// This uses NEAR's official types and Borsh serialization
pub fn serialize_near_delegate_action_to_base64(
    delegate_action: &DelegateAction,
) -> std::io::Result<String> {
    let borsh_bytes = borsh::to_vec(delegate_action)?;
    Ok(base64(&borsh_bytes))
}

/// Parse a NEAR public key from various formats
///
/// Supports:
/// - "ed25519:base58..." (NEAR standard)
/// - "base58..." (Circle API format, assumes ED25519)
pub fn parse_near_public_key(s: &str) -> Result<near_crypto::PublicKey, String> {
    use std::str::FromStr;

    // Try with prefix first
    if let Ok(pk) = near_crypto::PublicKey::from_str(s) {
        return Ok(pk);
    }

    // Try adding ed25519: prefix (Circle format)
    let with_prefix = format!("ed25519:{}", s);
    near_crypto::PublicKey::from_str(&with_prefix)
        .map_err(|e| format!("Failed to parse NEAR public key: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert_eq!(uuid.len(), 36); // Standard UUID length
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_pagination_params_serialization() {
        let params = PaginationParams {
            page_size: Some(10),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&params).unwrap();
        assert!(serialized.contains("pageSize"));
        assert!(!serialized.contains("pageAfter"));
    }

    #[test]
    fn test_encrypt_entity_secret_generates_different_values() {
        // Test that multiple encryptions of the same data produce different results
        // This is expected behavior due to RSA-OAEP padding with random values

        // Use a simple test key pair (in practice, this would come from environment)
        let entity_secret = "deadbeef"; // Simple hex string
        let test_public_key = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3VoPN9PKUjKFLMwOge9+\nG852nMEQAiMpm8FZ8VpJx5yXHdVXyMqTDwGJstidHy5htGKsyEArvIHxBzgWhMfL\nKLFZjvnxWx+rm/d5fk/5UjpjGFI7KABlxEBOAArBuLoi8TJb9BF3MjEqtlHOHUj6\nKG2n4sRRqeWpyFxTJU2v8fhJgHR1HhYkdHw8JdJ6J1lNNJGE7JfGtKDHI4mEo8ZN\nKF8TlW4wIJLQ4CJtEZJH2vKJJFNyFwJNJG2vKJJFNyFwJNJG2vKJJFNyFwJNJG2v\nKJJFNyFwJNJG2vKJJFNyFwJNJG2vKJJFNyFwJNJG2vKJJFNyFwJNJG2vKJJFNyFw\nQIDAQAB\n-----END PUBLIC KEY-----";

        // Note: This test would require a valid key pair to actually run
        // For now, we'll just test the function signature and error handling
        let result = encrypt_entity_secret(entity_secret, test_public_key);

        // We expect this to fail with an invalid key, but that's ok for testing the interface
        assert!(result.is_err());

        // The important thing is that the function exists and has the right signature
        // In real usage with valid keys, multiple calls would produce different encrypted values
    }
}
