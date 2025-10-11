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
    /// Retrieves a list of all contracts that fit the specified parameters
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
    /// Retrieves details of a specific contract by ID
    pub async fn get_contract(&self, contract_id: &str) -> CircleResult<ContractResponse> {
        let path = format!("/v1/w3s/contracts/{}", contract_id);
        self.get(&path).await
    }

    /// Update a contract
    ///
    /// Updates contract metadata such as name and reference ID
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
    /// given the contract bytecode.
    ///
    /// You must provide either:
    /// - `wallet_id` (recommended), OR
    /// - Both `blockchain` and `source_address`
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
    /// Estimates the gas fee required to deploy a contract from a template
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
    pub async fn query_contract(
        &self,
        builder: QueryContractViewBodyBuilder,
    ) -> CircleResult<QueryContractResponse> {
        let body = builder.build();
        self.post("/v1/w3s/contracts/query", &body).await
    }

    /// List all notification subscriptions
    ///
    /// Retrieves an array of existing notification subscriptions
    pub async fn list_notification_subscriptions(
        &self,
    ) -> CircleResult<Vec<NotificationSubscription>> {
        self.get("/v2/notifications/subscriptions").await
    }

    /// Get a notification
    ///
    /// Retrieves a notification by ID
    pub async fn get_notification(
        &self,
        notification_id: &str,
    ) -> CircleResult<NotificationSubscription> {
        let path = format!("/v2/notifications/subscriptions/{}", notification_id);
        self.get(&path).await
    }

    /// Get the notification signature public key
    ///
    /// Retrieves the public key used to sign notifications.
    pub async fn get_notification_sig_pub_key(&self, public_key: &str) -> CircleResult<String> {
        self.get(format!("/v2/notifications/publicKey/{}", public_key).as_str())
            .await
    }

    /// Create a notification subscription
    ///
    /// Creates a notification subscription by configuring an endpoint to receive notifications.
    /// For details, see the Notification Flows guide.
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
    /// Deletes an existing notification subscription by ID.
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
    /// Retrieves the health of the Circle API
    /// Note: This endpoint returns plain JSON (not wrapped in data field)
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
    pub async fn delete_event_monitor(&self, monitor_id: &str) -> CircleResult<()> {
        self.delete_no_content(format!("/v1/w3s/contracts/monitors/{}", monitor_id).as_str())
            .await
    }

    /// List event monitors
    ///
    /// Fetches a list of event monitors, optionally filtered by blockchain, contract address,
    /// and event signature.
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
    /// by blockchain and contract address.
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
