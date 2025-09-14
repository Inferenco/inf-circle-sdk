//! CircleView module for read operations (GET)
//!
//! This module handles all read operations that only require base URL configuration.

use crate::helper::{build_query_params, get_env_var, CircleResult, HttpClient};
use reqwest::Method;
use serde::Serialize;

/// CircleView handles read operations (GET) with base URL configuration
pub struct CircleView {
    client: HttpClient,
}

impl CircleView {
    /// Create a new CircleView instance
    ///
    /// Reads CIRCLE_BASE_URL from environment variables
    pub fn new() -> CircleResult<Self> {
        dotenv::dotenv().ok(); // Load .env file if present

        let api_key = get_env_var("CIRCLE_API_KEY")?;
        let base_url = get_env_var("CIRCLE_BASE_URL")?;
        let client = HttpClient::with_api_key(&base_url, api_key)?;

        Ok(Self { client })
    }

    /// Generic request method for read operations
    pub async fn request<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let request = self.client.request(Method::GET, path)?;
        self.client.execute(request).await
    }

    /// GET request with query parameters
    pub async fn request_with_params<T, R>(&self, path: &str, params: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let query_string = build_query_params(params)?;
        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{}?{}", path, query_string)
        };

        self.request(&full_path).await
    }

    /// GET request helper
    pub async fn get<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request(path).await
    }

    /// GET request with query parameters helper
    pub async fn get_with_params<T, R>(&self, path: &str, params: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request_with_params(path, params).await
    }
}
