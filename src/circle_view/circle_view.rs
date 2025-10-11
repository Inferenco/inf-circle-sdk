//! CircleView module for read operations (GET)
//!
//! This module handles all read operations that only require API key authentication.
//! No entity secret is needed, making it safe for read-only processes.
//!
//! # Features
//!
//! - Query wallet balances and NFTs
//! - List transactions and wallets
//! - Query smart contracts
//! - List and manage event monitors
//! - Estimate fees before transactions
//! - Validate addresses
//!
//! # Example
//!
//! ```rust,no_run
//! use inf_circle_sdk::circle_view::circle_view::CircleView;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Requires CIRCLE_API_KEY and CIRCLE_BASE_URL in env
//!     let view = CircleView::new()?;
//!     
//!     // Now you can perform read operations
//!     Ok(())
//! }
//! ```

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
    /// Initializes a Circle SDK client for read-only operations. Reads configuration from
    /// environment variables:
    /// - `CIRCLE_API_KEY`: Your Circle API key
    /// - `CIRCLE_BASE_URL`: Circle API base URL (e.g., https://api.circle.com)
    ///
    /// Unlike `CircleOps`, this does not require entity secret or public key since it only
    /// performs read operations.
    ///
    /// # Returns
    ///
    /// Returns a configured `CircleView` instance ready to make API queries.
    ///
    /// # Errors
    ///
    /// Returns an error if any required environment variable is missing or invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Ensure .env file is loaded or environment variables are set
    /// dotenv::dotenv().ok();
    ///
    /// let view = CircleView::new()?;
    /// println!("CircleView initialized successfully!");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> CircleResult<Self> {
        dotenv::dotenv().ok(); // Load .env file if present

        let api_key = get_env_var("CIRCLE_API_KEY")?;
        let base_url = get_env_var("CIRCLE_BASE_URL")?;
        let client = HttpClient::with_api_key(&base_url, api_key)?;

        Ok(Self { client })
    }

    /// Generic request method for read operations
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

        self.request::<(), R>(Method::GET, &full_path, None).await
    }

    /// GET request helper
    pub async fn get<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<(), R>(Method::GET, path, None).await
    }

    /// GET request helper for endpoints that return plain JSON (not wrapped in data field)
    pub async fn get_plain<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let request = self.client.request(Method::GET, path)?;
        let response = request.send().await?;

        let status = response.status();
        let response_text = response.text().await?;
        println!("Response text: {}", response_text);

        if status.is_success() {
            let result: R = serde_json::from_str(&response_text)?;
            Ok(result)
        } else {
            use crate::helper::{CircleError, CircleErrorResponse};
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

    /// GET request with query parameters helper
    pub async fn get_with_params<T, R>(&self, path: &str, params: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request_with_params(path, params).await
    }

    /// POST request helper
    pub async fn post<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<T, R>(Method::POST, path, Some(body)).await
    }

    /// PUT request helper
    pub async fn put<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<T, R>(Method::PUT, path, Some(body)).await
    }

    /// PATCH request helper
    pub async fn patch<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<T, R>(Method::PATCH, path, Some(body)).await
    }

    /// DELETE request helper
    pub async fn delete<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<(), R>(Method::DELETE, path, None).await
    }

    /// DELETE request helper that expects no response body
    pub async fn delete_no_content(&self, path: &str) -> CircleResult<()> {
        use crate::helper::{CircleError, CircleErrorResponse};

        let request = self.client.request(Method::DELETE, path)?;
        let response = request.send().await?;

        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let response_text = response.text().await?;

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
