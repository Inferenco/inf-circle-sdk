//! CircleOps module for write operations (POST, PUT, PATCH)
//!
//! This module handles all write operations that require entity secret authentication.

use crate::{
    encrypt_entity_secret,
    helper::{get_env_var, CircleResult, HttpClient},
    CircleError,
};
use reqwest::Method;
use serde::Serialize;

/// CircleOps handles write operations (POST, PUT, PATCH) with entity secret authentication
pub struct CircleOps {
    client: HttpClient,
    entity_secret: String,
    public_key: String,
}

impl CircleOps {
    /// Create a new CircleOps instance
    ///
    /// Reads CIRCLE_API_KEY and CIRCLE_BASE_URL from environment variables
    pub fn new() -> CircleResult<Self> {
        dotenv::dotenv().ok(); // Load .env file if present

        let api_key = get_env_var("CIRCLE_API_KEY")?;
        let base_url = get_env_var("CIRCLE_BASE_URL")?;

        let entity_secret = get_env_var("CIRCLE_ENTITY_SECRET")?;
        let public_key = get_env_var("CIRCLE_PUBLIC_KEY")?;

        let client = HttpClient::with_api_key(&base_url, api_key)?;

        Ok(Self {
            client,
            entity_secret,
            public_key,
        })
    }

    /// Generic request method for write operations
    pub async fn request<T, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let mut request = self.client.request(method, path)?;

        if let Some(body) = body {
            request = request.json(body);
        }

        self.client.execute(request).await
    }

    /// POST request helper
    pub async fn post<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request(Method::POST, path, Some(body)).await
    }

    /// PUT request helper
    pub async fn put<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// PATCH request helper
    pub async fn patch<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request(Method::PATCH, path, Some(body)).await
    }

    pub fn entity_secret(&self) -> CircleResult<String> {
        let entity_secret_ciphertext = encrypt_entity_secret(&self.entity_secret, &self.public_key)
            .map_err(|e| CircleError::Config(format!("Failed to encrypt entity secret: {}", e)))?;

        Ok(entity_secret_ciphertext)
    }
}
