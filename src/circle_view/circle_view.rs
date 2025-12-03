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
#[derive(Clone)]
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
    ///
    /// This is an internal helper method used by other methods in this struct.
    /// Typically, you should use the specific methods like `get`, `post`, `put`, or `patch` instead.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method (GET, POST, PUT, PATCH, DELETE)
    /// * `path` - API endpoint path
    /// * `body` - Optional request body to serialize
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use reqwest::Method;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// // Usually you'd use view.get() instead
    /// let response: serde_json::Value = view.request(
    ///     Method::GET,
    ///     "/v1/w3s/wallets",
    ///     None::<&serde_json::Value>
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// Sends a GET request with query parameters serialized from the provided params object.
    /// This is an internal helper method typically used by `get_with_params`.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    /// * `params` - Query parameters to serialize
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let params = serde_json::json!({
    ///     "pageSize": 10,
    ///     "pageAfter": "cursor"
    /// });
    ///
    /// let response: serde_json::Value = view.request_with_params("/v1/w3s/wallets", &params).await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// Sends a GET request to the specified endpoint and deserializes the response.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let response: serde_json::Value = view.get("/v1/w3s/wallets/wallet-id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<(), R>(Method::GET, path, None).await
    }

    /// GET request helper for endpoints that return plain JSON (not wrapped in data field)
    ///
    /// Some Circle API endpoints return plain JSON responses instead of the standard
    /// `{ "data": {...} }` format. This method handles those cases.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// // The /ping endpoint returns plain JSON
    /// let response: serde_json::Value = view.get_plain("/ping").await?;
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// Sends a GET request with query parameters serialized from the provided params object.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    /// * `params` - Query parameters to serialize and append to the URL
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let params = serde_json::json!({
    ///     "pageSize": 10,
    ///     "blockchain": "ETH-SEPOLIA"
    /// });
    ///
    /// let response: serde_json::Value = view.get_with_params("/v1/w3s/wallets", &params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_with_params<T, R>(&self, path: &str, params: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request_with_params(path, params).await
    }

    /// POST request helper
    ///
    /// Sends a POST request to the specified endpoint with the given body.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    /// * `body` - Request body to serialize and send
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let request_body = serde_json::json!({
    ///     "address": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"
    /// });
    ///
    /// let response: serde_json::Value = view.post("/v1/w3s/transactions/validateAddress", &request_body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<T, R>(Method::POST, path, Some(body)).await
    }

    /// PUT request helper
    ///
    /// Sends a PUT request to the specified endpoint with the given body.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    /// * `body` - Request body to serialize and send
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let request_body = serde_json::json!({
    ///     "isEnabled": false
    /// });
    ///
    /// let response: serde_json::Value = view.put("/v1/w3s/contracts/monitors/monitor-id", &request_body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn put<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<T, R>(Method::PUT, path, Some(body)).await
    }

    /// PATCH request helper
    ///
    /// Sends a PATCH request to the specified endpoint with the given body.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    /// * `body` - Request body to serialize and send
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let request_body = serde_json::json!({
    ///     "name": "Updated Name"
    /// });
    ///
    /// let response: serde_json::Value = view.patch("/v1/w3s/contracts/contract-id", &request_body).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn patch<T, R>(&self, path: &str, body: &T) -> CircleResult<R>
    where
        T: Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<T, R>(Method::PATCH, path, Some(body)).await
    }

    /// DELETE request helper
    ///
    /// Sends a DELETE request to the specified endpoint and deserializes the response.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let response: serde_json::Value = view.delete("/v1/w3s/contracts/monitors/monitor-id").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete<R>(&self, path: &str) -> CircleResult<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        self.request::<(), R>(Method::DELETE, path, None).await
    }

    /// DELETE request helper that expects no response body
    ///
    /// Sends a DELETE request to the specified endpoint and expects an empty response (204 No Content).
    /// This is used for endpoints that don't return a response body on successful deletion.
    ///
    /// # Arguments
    ///
    /// * `path` - API endpoint path
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// // Delete a notification subscription (returns 204 No Content)
    /// view.delete_no_content("/v2/notifications/subscriptions/subscription-id").await?;
    /// println!("✅ Deleted successfully");
    /// # Ok(())
    /// # }
    /// ```
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
