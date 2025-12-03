//! Contract read operations for CircleView
use crate::contract::dto::{
    CreateNotificationSubscriptionResponse, EventLogsResponse, EventMonitorResponse,
    EventMonitorsResponse, FeeEstimation, NotificationSubscription, PingResponse,
    QueryContractResponse, UpdateNotificationSubscriptionResponse,
};
use crate::contract::views::create_event_monitor::CreateEventMonitorBodyBuilder;
use crate::contract::views::create_notification_subscription::CreateNotificationSubscriptionBodyBuilder;
use crate::contract::views::estimate_contract_deployment::EstimateContractDeploymentBodyBuilder;
use crate::contract::views::estimate_template_deployment_fee::EstimateTemplateDeploymentFeeBodyBuilder;
use crate::contract::views::query_contract_view::QueryContractViewBodyBuilder;
use crate::contract::views::update_event_monitor::UpdateEventMonitorBodyBuilder;
use crate::contract::views::update_notification_subscription::UpdateNotificationSubscriptionBodyBuilder;
use crate::helper::CircleResult;
use crate::{circle_view::circle_view::CircleView, contract::dto::UpdateContractRequest};
// Re-use the Contract struct from CircleOps since it's the same
pub use crate::contract::dto::{
    Contract, ContractResponse, ContractsResponse, EventLog, EventMonitor, ListContractsParams,
    ListEventLogsParams, ListEventMonitorsParams, NotificationType,
};

impl CircleView {
    /// List contracts
    ///
    /// Retrieves a list of all contracts that fit the specified parameters.
    /// Supports filtering by blockchain, wallet set ID, and pagination.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional filter parameters for listing contracts
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::dto::ListContractsParams;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// // List all contracts
    /// let contracts = view.list_contracts(None).await?;
    /// for contract in contracts.contracts {
    ///     println!("Contract: {:?} - {:?}", contract.name, contract.address);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_contracts(
        &self,
        params: Option<ListContractsParams>,
    ) -> CircleResult<ContractsResponse> {
        match params {
            Some(params) => self.get_with_params("/v1/w3s/contracts", &params).await,
            None => self.get("/v1/w3s/contracts").await,
        }
    }

    /// Get a specific contract
    ///
    /// Retrieves detailed information about a specific contract by ID, including
    /// its address, ABI, deployment details, and metadata.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The unique identifier of the contract
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let contract = view.get_contract("contract-id").await?;
    /// println!("Contract name: {:?}", contract.contract.name);
    /// println!("Address: {:?}", contract.contract.address);
    /// println!("Blockchain: {:?}", contract.contract.blockchain);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_contract(&self, contract_id: &str) -> CircleResult<ContractResponse> {
        let path = format!("/v1/w3s/contracts/{}", contract_id);
        self.get(&path).await
    }

    /// Update a contract
    ///
    /// Updates contract metadata such as name and reference ID.
    /// This does not modify the contract code, only the metadata stored in Circle.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - The contract ID to update
    /// * `request` - The update request with new metadata
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::dto::UpdateContractRequest;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let request = UpdateContractRequest {
    ///     name: Some("Updated Contract Name".to_string()),
    ///     ref_id: Some("new-ref-id".to_string()),
    /// };
    ///
    /// let response = view.update_contract("contract-id", request).await?;
    /// println!("Updated contract: {:?}", response.contract.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_contract(
        &self,
        contract_id: &str,
        request: UpdateContractRequest,
    ) -> CircleResult<ContractResponse> {
        let path = format!("/v1/w3s/contracts/{}", contract_id);
        self.patch(&path, &request).await
    }

    /// Estimate fee for contract deployment from bytecode
    ///
    /// Estimates the network fee for deploying a smart contract on a specified blockchain,
    /// given the contract bytecode. Useful for displaying expected costs before deployment.
    ///
    /// You must provide either:
    /// - `wallet_id` (recommended), OR
    /// - Both `blockchain` and `source_address`
    ///
    /// # Arguments
    ///
    /// * `builder` - The contract deployment fee estimation builder
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::estimate_contract_deployment::EstimateContractDeploymentBodyBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let bytecode = "0x608060405234801561001057600080fd5b50...";
    /// let abi_json = r#"[{"inputs":[],"name":"getValue","outputs":[...]...}]"#;
    ///
    /// let builder = EstimateContractDeploymentBodyBuilder::new(bytecode.to_string())
    ///     .abi_json(abi_json.to_string())
    ///     .wallet_id("wallet-id".to_string());
    ///
    /// let estimate = view.estimate_contract_deployment_fee(builder).await?;
    /// println!("Low gas limit: {}", estimate.low.gas_limit);
    /// println!("Medium gas limit: {}", estimate.medium.gas_limit);
    /// println!("High gas limit: {}", estimate.high.gas_limit);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn estimate_contract_deployment_fee(
        &self,
        builder: EstimateContractDeploymentBodyBuilder,
    ) -> CircleResult<FeeEstimation> {
        let body = builder.build();
        self.post("/v1/w3s/contracts/deploy/estimateFee", &body)
            .await
    }

    /// Estimate fee for contract template deployment
    ///
    /// Estimates the gas fee required to deploy a contract from a template.
    /// Useful for displaying expected costs before template deployment.
    ///
    /// # Arguments
    ///
    /// * `builder` - The template deployment fee estimation builder
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::estimate_template_deployment_fee::EstimateTemplateDeploymentFeeBodyBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let builder = EstimateTemplateDeploymentFeeBodyBuilder::new(
    ///     "template-id".to_string(),
    ///     Blockchain::EthSepolia,
    ///     "wallet-id".to_string()
    /// )
    /// .build();
    ///
    /// let estimate = view.estimate_template_deployment_fee(builder).await?;
    /// println!("Estimated gas limit: {}", estimate.medium.gas_limit);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn estimate_template_deployment_fee(
        &self,
        builder: EstimateTemplateDeploymentFeeBodyBuilder,
    ) -> CircleResult<FeeEstimation> {
        let builder = builder.build();
        let id = builder.template_id.clone();

        let mut body = serde_json::json!({
            "blockchain": builder.blockchain,
            "walletId": builder.wallet_id,
        });

        if let Some(template_params) = builder.template_parameters {
            body["templateParameters"] = template_params;
        }

        if let Some(constructor_params) = builder.constructor_params {
            body["constructorParams"] = serde_json::json!(constructor_params);
        }

        self.post(
            format!("/v1/w3s/templates/{}/deploy/estimateFee", &id).as_str(),
            &body,
        )
        .await
    }

    /// Query a contract (read-only)
    ///
    /// Execute a query function on a contract by providing the address and blockchain.
    /// This is used for read-only operations (view/pure functions) that don't modify state.
    ///
    /// You must provide either:
    /// - `abi_function_signature` + `abi_parameters` (recommended), OR
    /// - `call_data` (pre-encoded function call)
    ///
    /// # Arguments
    ///
    /// * `builder` - The contract query builder with function details
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::query_contract_view::QueryContractViewBodyBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    /// use serde_json::json;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let builder = QueryContractViewBodyBuilder::new(
    ///     Blockchain::EthSepolia,
    ///     "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string() // USDC contract
    /// )
    /// .abi_function_signature("balanceOf(address)".to_string())
    /// .abi_parameters(vec![json!("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb")]);
    ///
    /// let response = view.query_contract(builder).await?;
    /// println!("Query result: {:?}", response.output_values);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_contract(
        &self,
        builder: QueryContractViewBodyBuilder,
    ) -> CircleResult<QueryContractResponse> {
        let body = builder.build();
        self.post("/v1/w3s/contracts/query", &body).await
    }

    /// List all notification subscriptions
    ///
    /// Retrieves an array of existing notification subscriptions configured for your account.
    /// These subscriptions define webhook endpoints that receive notifications for various events.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let subscriptions = view.list_notification_subscriptions().await?;
    /// for sub in subscriptions {
    ///     println!("Subscription: {} - {}", sub.id, sub.endpoint);
    ///     println!("Enabled: {}", sub.enabled);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_notification_subscriptions(
        &self,
    ) -> CircleResult<Vec<NotificationSubscription>> {
        self.get("/v2/notifications/subscriptions").await
    }

    /// Get a notification
    ///
    /// Retrieves detailed information about a specific notification subscription by ID.
    ///
    /// # Arguments
    ///
    /// * `notification_id` - The unique identifier of the notification subscription
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let subscription = view.get_notification("notification-id").await?;
    /// println!("Endpoint: {}", subscription.endpoint);
    /// println!("Enabled: {}", subscription.enabled);
    /// println!("Notification types: {:?}", subscription.notification_types);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_notification(
        &self,
        notification_id: &str,
    ) -> CircleResult<NotificationSubscription> {
        let path = format!("/v2/notifications/subscriptions/{}", notification_id);
        self.get(&path).await
    }

    /// Get the notification signature public key
    ///
    /// Retrieves the public key used to sign notifications. This key is used to verify
    /// that notifications received at your webhook endpoint are authentic and from Circle.
    ///
    /// # Arguments
    ///
    /// * `public_key` - The public key identifier
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let pub_key = view.get_notification_sig_pub_key("key-id").await?;
    /// println!("Public key: {}", pub_key);
    /// // Use this key to verify notification signatures in your webhook handler
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_notification_sig_pub_key(&self, public_key: &str) -> CircleResult<String> {
        self.get(format!("/v2/notifications/publicKey/{}", public_key).as_str())
            .await
    }

    /// Create a notification subscription
    ///
    /// Creates a notification subscription by configuring an endpoint to receive notifications.
    /// For details, see the Notification Flows guide.
    ///
    /// # Arguments
    ///
    /// * `builder` - The notification subscription builder with endpoint and notification types
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::create_notification_subscription::CreateNotificationSubscriptionBodyBuilder;
    /// use inf_circle_sdk::contract::dto::NotificationType;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let builder = CreateNotificationSubscriptionBodyBuilder::new(
    ///     "https://example.com/webhook".to_string()
    /// )
    /// .notification_types(vec![
    ///     NotificationType::TransactionsInbound,
    ///     NotificationType::TransactionsOutbound,
    /// ]);
    ///
    /// let response = view.create_notification_subscription(builder).await?;
    /// println!("Created subscription: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_notification_subscription(
        &self,
        builder: CreateNotificationSubscriptionBodyBuilder,
    ) -> CircleResult<CreateNotificationSubscriptionResponse> {
        let request = builder.build();
        self.post("/v2/notifications/subscriptions", &request).await
    }

    /// Update a notification subscription
    ///
    /// Updates an existing notification subscription by modifying the enabled status and/or name.
    /// For details, see the Notification Flows guide.
    ///
    /// # Arguments
    ///
    /// * `builder` - The notification subscription update builder
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::update_notification_subscription::UpdateNotificationSubscriptionBodyBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let builder = UpdateNotificationSubscriptionBodyBuilder::new("notification-id".to_string())
    ///     .enabled(false)  // Disable the subscription
    ///     .name("Updated Subscription Name".to_string());
    ///
    /// let response = view.update_notification_subscription(builder).await?;
    /// println!("Updated subscription: {}", response.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_notification_subscription(
        &self,
        builder: UpdateNotificationSubscriptionBodyBuilder,
    ) -> CircleResult<UpdateNotificationSubscriptionResponse> {
        let (notification_id, request) = builder.build();
        let path = format!("/v2/notifications/subscriptions/{}", notification_id);
        self.patch(&path, &request).await
    }

    /// Delete a notification subscription
    ///
    /// Deletes an existing notification subscription by ID. Once deleted, your webhook
    /// endpoint will no longer receive notifications for this subscription.
    ///
    /// # Arguments
    ///
    /// * `notification_id` - The unique identifier of the notification subscription to delete
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// view.delete_notification_subscription("notification-id").await?;
    /// println!("✅ Notification subscription deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_notification_subscription(
        &self,
        notification_id: &str,
    ) -> CircleResult<()> {
        self.delete_no_content(
            format!("/v2/notifications/subscriptions/{}", notification_id).as_str(),
        )
        .await
    }

    /// Get health of Circle API
    ///
    /// Retrieves the health status of the Circle API. This is a simple endpoint
    /// to check if the API is operational.
    /// Note: This endpoint returns plain JSON (not wrapped in data field)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let ping = view.get_ping().await?;
    /// println!("API Message: {}", ping.message);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_ping(&self) -> CircleResult<PingResponse> {
        self.get_plain("/ping").await
    }

    /// Create an event monitor
    ///
    /// Creates a new event monitor based on the provided blockchain, contract address,
    /// and event signature. Event monitors allow you to subscribe to specific blockchain
    /// events emitted by smart contracts.
    ///
    /// # Example Event Signature
    /// ```text
    /// "Transfer(address indexed from, address indexed to, uint256 value)"
    /// ```
    ///
    /// Note: Ensure no spaces are included in the event signature.
    ///
    /// # Arguments
    ///
    /// * `builder` - The event monitor builder with contract address, blockchain, and event signature
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::create_event_monitor::CreateEventMonitorBodyBuilder;
    /// use inf_circle_sdk::types::Blockchain;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// use uuid::Uuid;
    /// let builder = CreateEventMonitorBodyBuilder::new(
    ///     Uuid::new_v4().to_string(),
    ///     "Transfer(address indexed,address indexed,uint256)".to_string(),
    ///     "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string(), // USDC contract
    ///     Blockchain::EthSepolia
    /// );
    ///
    /// let response = view.create_event_monitor(builder).await?;
    /// println!("Created event monitor: {}", response.event_monitor.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_event_monitor(
        &self,
        builder: CreateEventMonitorBodyBuilder,
    ) -> CircleResult<EventMonitorResponse> {
        let body = builder.build();
        self.post("/v1/w3s/contracts/monitors", &body).await
    }

    /// Update an event monitor
    ///
    /// Updates an existing event monitor given its ID. You can enable or disable the monitor.
    /// When disabled, the monitor will not emit notifications for events.
    ///
    /// # Arguments
    ///
    /// * `builder` - The event monitor update builder with monitor ID and enabled status
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    /// use inf_circle_sdk::contract::views::update_event_monitor::UpdateEventMonitorBodyBuilder;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// let builder = UpdateEventMonitorBodyBuilder::new("monitor-id".to_string(), false);
    ///
    /// let response = view.update_event_monitor(builder).await?;
    /// println!("Updated monitor: {} - Enabled: {}", response.event_monitor.id, response.event_monitor.is_enabled);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_event_monitor(
        &self,
        builder: UpdateEventMonitorBodyBuilder,
    ) -> CircleResult<EventMonitorResponse> {
        let (monitor_id, request) = builder.build();
        let path = format!("/v1/w3s/contracts/monitors/{}", monitor_id);
        self.put(&path, &request).await
    }

    /// Delete an event monitor
    ///
    /// Deletes an existing event monitor given its ID. Once deleted, the monitor will no longer
    /// emit notifications for the configured contract events.
    ///
    /// # Arguments
    ///
    /// * `monitor_id` - The unique identifier of the event monitor to delete
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// view.delete_event_monitor("monitor-id").await?;
    /// println!("✅ Event monitor deleted");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_event_monitor(&self, monitor_id: &str) -> CircleResult<()> {
        self.delete_no_content(format!("/v1/w3s/contracts/monitors/{}", monitor_id).as_str())
            .await
    }

    /// List event monitors
    ///
    /// Fetches a list of event monitors, optionally filtered by blockchain, contract address,
    /// and event signature.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional filter parameters for listing event monitors
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// // List all event monitors
    /// let response = view.list_event_monitors(None).await?;
    /// for monitor in response.event_monitors {
    ///     println!("Monitor: {} - Contract: {} - Enabled: {}",
    ///         monitor.id, monitor.contract_address, monitor.is_enabled);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_event_monitors(
        &self,
        params: Option<ListEventMonitorsParams>,
    ) -> CircleResult<EventMonitorsResponse> {
        match params {
            Some(params) => {
                self.get_with_params("/v1/w3s/contracts/monitors", &params)
                    .await
            }
            None => self.get("/v1/w3s/contracts/monitors").await,
        }
    }

    /// List event logs
    ///
    /// Fetches all event logs generated from monitored contract events, optionally filtered
    /// by blockchain and contract address. These are the actual events that were emitted
    /// by contracts and captured by your event monitors.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional filter parameters for listing event logs
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use inf_circle_sdk::circle_view::circle_view::CircleView;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let view = CircleView::new()?;
    ///
    /// // List all event logs
    /// let response = view.list_event_logs(None).await?;
    /// for log in response.event_logs {
    ///     println!("Event: {} - Block: {} - Tx: {}",
    ///         log.event_signature, log.block_height, log.tx_hash);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_event_logs(
        &self,
        params: Option<ListEventLogsParams>,
    ) -> CircleResult<EventLogsResponse> {
        match params {
            Some(params) => {
                self.get_with_params("/v1/w3s/contracts/events", &params)
                    .await
            }
            None => self.get("/v1/w3s/contracts/events").await,
        }
    }
}
