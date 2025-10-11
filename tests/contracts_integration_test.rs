mod common;

use common::get_or_create_test_wallet;
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    contract::{
        dto::{
            ListContractsParams, ListEventLogsParams, ListEventMonitorsParams, NotificationType,
            UpdateContractRequest,
        },
        ops::{
            deploy_contract::DeployContractRequestBuilder,
            deploy_contract_from_template::DeployContractFromTemplateRequestBuilder,
            import_contract::ImportContractRequestBuilder,
        },
        views::{
            create_event_monitor::CreateEventMonitorBodyBuilder,
            create_notification_subscription::CreateNotificationSubscriptionBodyBuilder,
            estimate_contract_deployment::EstimateContractDeploymentBodyBuilder,
            estimate_template_deployment_fee::EstimateTemplateDeploymentFeeBodyBuilder,
            query_contract_view::QueryContractViewBodyBuilder,
            update_event_monitor::UpdateEventMonitorBodyBuilder,
            update_notification_subscription::UpdateNotificationSubscriptionBodyBuilder,
        },
    },
    helper::PaginationParams,
    types::Blockchain,
};
use std::env;

#[tokio::test]
async fn test_ping() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("🏓 Testing ping endpoint...");
    let ping_response = view.get_ping().await.expect("Failed to get ping");

    println!("✅ Ping response: {}", ping_response.message);
    assert!(!ping_response.message.is_empty());
}

#[tokio::test]
async fn test_list_contracts() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("📋 Testing list contracts...");

    // Test with no params
    let contracts = view
        .list_contracts(None)
        .await
        .expect("Failed to list contracts");

    println!("✅ Found {} contracts", contracts.contracts.len());

    // Test with params
    let params = ListContractsParams {
        address: None,
        blockchain: Some(Blockchain::EthSepolia),
        template_id: None,
        ref_id: None,
        from: None,
        to: None,
        pagination: PaginationParams {
            page_size: Some(10),
            page_before: None,
            page_after: None,
        },
    };

    let filtered_contracts = view
        .list_contracts(Some(params))
        .await
        .expect("Failed to list contracts with params");

    println!(
        "✅ Found {} contracts on ETH-SEPOLIA",
        filtered_contracts.contracts.len()
    );
}

#[tokio::test]
async fn test_get_contract() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("🔍 Testing get contract...");

    // First, list contracts to get an ID
    let contracts = view
        .list_contracts(None)
        .await
        .expect("Failed to list contracts");

    if let Some(contract) = contracts.contracts.first() {
        let contract_id = contract.id.as_ref().expect("Contract should have an ID");
        println!("📝 Getting contract with ID: {}", contract_id);

        let fetched_response = view
            .get_contract(contract_id)
            .await
            .expect("Failed to get contract");

        println!("✅ Contract fetched: {:?}", fetched_response.contract.id);
        println!("   Address: {:?}", fetched_response.contract.address);
        println!("   Blockchain: {:?}", fetched_response.contract.blockchain);
        println!("   State: {:?}", fetched_response.contract.state);

        assert_eq!(
            fetched_response.contract.id.as_deref(),
            Some(contract_id.as_str())
        );
    } else {
        println!("⚠️  No contracts found to test get_contract. This is expected if no contracts have been deployed.");
    }
}

#[tokio::test]
async fn test_update_contract() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("✏️  Testing update contract...");

    // First, list contracts to get an ID
    let contracts = view
        .list_contracts(None)
        .await
        .expect("Failed to list contracts");

    if let Some(contract) = contracts.contracts.first() {
        let contract_id = contract.id.as_ref().expect("Contract should have an ID");
        println!("📝 Updating contract with ID: {}", contract_id);

        let update_request = UpdateContractRequest {
            name: Some("Updated Test Contract".to_string()),
            ref_id: Some(format!("updated-ref-{}", uuid::Uuid::new_v4())),
        };

        let updated_response = view
            .update_contract(contract_id, update_request)
            .await
            .expect("Failed to update contract");

        println!("✅ Contract updated: {:?}", updated_response.contract.id);
        println!("   New name: {:?}", updated_response.contract.name);
        println!("   New ref_id: {:?}", updated_response.contract.ref_id);

        assert_eq!(
            updated_response.contract.id.as_deref(),
            Some(contract_id.as_str())
        );
        assert_eq!(
            updated_response.contract.name.as_deref(),
            Some("Updated Test Contract")
        );
    } else {
        println!("⚠️  No contracts found to test update_contract. This is expected if no contracts have been deployed.");
    }
}

#[tokio::test]
async fn test_estimate_template_deployment_fee() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Contract Test",
    )
    .await
    .expect("Failed to get or create wallet");

    println!("💰 Testing estimate template deployment fee...");

    // Get template ID from env or use Circle's sample NFT template
    let template_id = env::var("CIRCLE_TEMPLATE_ID").expect("CIRCLE_TEMPLATE_ID not set");

    // NFT template requires defaultAdmin parameter
    let template_params = serde_json::json!({
        "defaultAdmin": wallet.address
    });

    let estimate_request = EstimateTemplateDeploymentFeeBodyBuilder::new(
        template_id,
        Blockchain::EthSepolia,
        wallet.id.clone(),
    )
    .template_parameters(template_params)
    .build();

    let fee_estimation = view
        .estimate_template_deployment_fee(estimate_request)
        .await
        .expect("Failed to estimate template deployment fee");

    println!("✅ Fee estimated:");
    println!(
        "   Low:    gas_limit={}, max_fee={:?}",
        fee_estimation.low.gas_limit, fee_estimation.low.max_fee
    );
    println!(
        "   Medium: gas_limit={}, max_fee={:?}",
        fee_estimation.medium.gas_limit, fee_estimation.medium.max_fee
    );
    println!(
        "   High:   gas_limit={}, max_fee={:?}",
        fee_estimation.high.gas_limit, fee_estimation.high.max_fee
    );

    assert!(!fee_estimation.low.gas_limit.is_empty());
    assert!(!fee_estimation.medium.gas_limit.is_empty());
    assert!(!fee_estimation.high.gas_limit.is_empty());
}

#[tokio::test]
async fn test_estimate_contract_deployment_fee() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Contract Test",
    )
    .await
    .expect("Failed to get or create wallet");

    println!("💰 Testing estimate contract deployment fee from bytecode...");

    // Real bytecode for a simple storage contract
    // Source: contract SimpleStorage { uint256 public storedData; function set(uint256 x) public { storedData = x; } }
    // Compiled with solc 0.8.19
    let bytecode = "0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806360fe47b11461003b5780632a1afcd914610057575b600080fd5b61005560048036038101906100509190610096565b610075565b005b61005f61007f565b60405161006c91906100d2565b60405180910390f35b8060008190555050565b60005481565b60008135905061009081610103565b92915050565b6000602082840312156100ac576100ab6100fe565b5b60006100ba84828501610081565b91505092915050565b6100cc816100eb565b82525050565b60006020820190506100e760008301846100c3565b92915050565b6000819050919050565b600080fd5b61010c816100eb565b811461011757600080fd5b5056fea2646970667358221220abcdef1234567890abcdef1234567890abcdef1234567890abcdef123456789064736f6c63430008130033";

    println!("   Using bytecode (length: {} bytes)", bytecode.len());

    let fee_estimation = view
        .estimate_contract_deployment_fee(
            EstimateContractDeploymentBodyBuilder::new(bytecode.to_string())
                .wallet_id(wallet.id.clone()),
        )
        .await
        .expect("Failed to estimate contract deployment fee");

    println!("✅ Fee estimated for contract deployment:");
    println!(
        "   Low:    gas_limit={}, network_fee={:?}",
        fee_estimation.low.gas_limit, fee_estimation.low.network_fee
    );
    println!(
        "   Medium: gas_limit={}, network_fee={:?}",
        fee_estimation.medium.gas_limit, fee_estimation.medium.network_fee
    );
    println!(
        "   High:   gas_limit={}, network_fee={:?}",
        fee_estimation.high.gas_limit, fee_estimation.high.network_fee
    );

    assert!(!fee_estimation.low.gas_limit.is_empty());
    assert!(!fee_estimation.medium.gas_limit.is_empty());
    assert!(!fee_estimation.high.gas_limit.is_empty());
}

#[tokio::test]
async fn test_deploy_contract_from_template() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Contract Test",
    )
    .await
    .expect("Failed to get or create wallet");

    // Ensure wallet is funded before deploying
    println!("💰 Checking wallet balance...");
    common::ensure_wallet_funded(&view, &wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to ensure wallet is funded");

    println!("🚀 Testing deploy contract from template...");

    // Get template ID from env or use Circle's sample NFT template
    let template_id = env::var("CIRCLE_TEMPLATE_ID").expect("CIRCLE_TEMPLATE_ID not set");

    // NFT template requires defaultAdmin parameter
    let template_params = serde_json::json!({
        "defaultAdmin": wallet.address
    });

    let builder = DeployContractFromTemplateRequestBuilder::new(
        template_id,
        "Test Contract Deployment".to_string(),
        wallet.id.clone(),
        Blockchain::EthSepolia.as_str().to_string(),
    )
    .expect("Failed to create builder")
    .template_parameters(template_params)
    .fee_level("MEDIUM".to_string())
    .ref_id(format!("test-contract-{}", uuid::Uuid::new_v4()))
    .idempotency_key(uuid::Uuid::new_v4().to_string())
    .build();

    let deployment_response = ops
        .deploy_contract_from_template(builder)
        .await
        .expect("Failed to deploy contract from template");

    println!("✅ Contract deployment initiated!");
    println!("   Transaction ID: {}", deployment_response.transaction_id);
    println!("   Contract IDs: {:?}", deployment_response.contract_ids);

    assert!(!deployment_response.transaction_id.is_empty());
    assert!(!deployment_response.contract_ids.is_empty());
}

#[tokio::test]
async fn test_deploy_contract_from_bytecode() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Contract Test",
    )
    .await
    .expect("Failed to get or create wallet");

    // Ensure wallet is funded before deploying
    println!("💰 Checking wallet balance...");
    common::ensure_wallet_funded(&view, &wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to ensure wallet is funded");

    println!("🚀 Testing deploy contract from bytecode...");

    // Real bytecode and ABI for a simple storage contract
    // Source: contract SimpleStorage { uint256 public storedData; function set(uint256 x) public { storedData = x; } }
    let bytecode = "0x608060405234801561001057600080fd5b50610150806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806360fe47b11461003b5780632a1afcd914610057575b600080fd5b61005560048036038101906100509190610096565b610075565b005b61005f61007f565b60405161006c91906100d2565b60405180910390f35b8060008190555050565b60005481565b60008135905061009081610103565b92915050565b6000602082840312156100ac576100ab6100fe565b5b60006100ba84828501610081565b91505092915050565b6100cc816100eb565b82525050565b60006020820190506100e760008301846100c3565b92915050565b6000819050919050565b600080fd5b61010c816100eb565b811461011757600080fd5b5056fea2646970667358221220abcdef1234567890abcdef1234567890abcdef1234567890abcdef123456789064736f6c63430008130033";

    let abi_json = r#"[{"inputs":[],"name":"storedData","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"x","type":"uint256"}],"name":"set","outputs":[],"stateMutability":"nonpayable","type":"function"}]"#;

    // Contract name must be alphanumeric only
    let uuid_hex = uuid::Uuid::new_v4().to_string().replace("-", "");
    let name = format!("SimpleStorage{}", &uuid_hex[..8]);

    let builder = DeployContractRequestBuilder::new(
        bytecode.to_string(),
        abi_json.to_string(),
        wallet.id.clone(),
        name.clone(),
        Blockchain::EthSepolia,
    )
    .description("Test contract deployment from bytecode".to_string())
    .fee_level("MEDIUM".to_string())
    .ref_id(format!("test-deploy-{}", uuid::Uuid::new_v4()))
    .idempotency_key(uuid::Uuid::new_v4().to_string());

    let deployment_response = ops
        .deploy_contract(builder)
        .await
        .expect("Failed to deploy contract from bytecode");

    println!("✅ Contract deployment initiated!");
    println!("   Contract ID: {}", deployment_response.contract_id);
    println!("   Transaction ID: {}", deployment_response.transaction_id);
    println!("   Name: {}", name);

    assert!(!deployment_response.contract_id.is_empty());
    assert!(!deployment_response.transaction_id.is_empty());
}

#[tokio::test]
async fn test_import_contract() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");

    println!("📥 Testing import contract...");

    // Note: This test requires a real deployed contract address
    // Set TEST_CONTRACT_ADDRESS env var with an actual contract address on the blockchain
    let address = match env::var("TEST_CONTRACT_ADDRESS") {
        Ok(addr) if addr != "0x1234567890123456789012345678901234567890" => addr,
        _ => {
            println!(
                "⚠️  TEST_CONTRACT_ADDRESS not set or using placeholder address. Skipping test."
            );
            println!("   To run this test:");
            println!("   1. Deploy a contract to a testnet (e.g., Ethereum Sepolia)");
            println!("   2. Export the address: export TEST_CONTRACT_ADDRESS=\"0xYourRealContractAddress\"");
            println!("   3. Run the test again");
            println!("   Note: The contract must actually exist on the blockchain for Circle to import it.");
            return;
        }
    };

    // Contract name must be alphanumeric only (no spaces or special characters)
    let uuid_hex = uuid::Uuid::new_v4().to_string().replace("-", "");
    let name = format!("ImportedContract{}", &uuid_hex[..8]); // Use first 8 chars of UUID
    let description = Some("Test contract imported via SDK".to_string());

    println!("🔍 Importing contract at address: {}", address);

    let contract_response = ops
        .import_contract(
            ImportContractRequestBuilder::new(
                Blockchain::EthSepolia,
                address.clone(),
                name.clone(),
            )
            .description(description)
            .build(),
        )
        .await
        .expect("Failed to import contract");

    println!("✅ Contract imported!");
    println!("   Contract ID: {:?}", contract_response.contract.id);
    println!("   Address: {:?}", contract_response.contract.address);
    println!("   Blockchain: {:?}", contract_response.contract.blockchain);
    println!("   Name: {:?}", contract_response.contract.name);

    assert!(contract_response.contract.id.is_some());
    if let Some(contract_address) = &contract_response.contract.address {
        assert_eq!(contract_address.to_lowercase(), address.to_lowercase());
    }
}

#[tokio::test]
async fn test_notification_subscriptions_crud() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("🔔 Testing notification subscriptions CRUD operations...");

    // Note: Circle validates webhook endpoints by making an HTTP request to them.
    // You need to provide a publicly accessible endpoint via the CIRCLE_TEST_WEBHOOK_URL env variable.
    // You can use services like webhook.site, ngrok, or any publicly accessible server.
    // Example: export CIRCLE_TEST_WEBHOOK_URL="https://webhook.site/your-unique-id"

    let webhook_endpoint = match env::var("CIRCLE_TEST_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            println!(
                "⚠️  CIRCLE_TEST_WEBHOOK_URL not set. Skipping notification subscription test."
            );
            println!("   To run this test, set a publicly accessible webhook URL:");
            println!("   export CIRCLE_TEST_WEBHOOK_URL=\"https://webhook.site/your-unique-id\"");
            println!("   You can get a free webhook URL from https://webhook.site");
            return;
        }
    };

    println!("🌐 Using webhook endpoint: {}", webhook_endpoint);

    // 1. Check if a subscription already exists for this endpoint and clean it up
    println!("\n1️⃣  Checking for existing subscriptions with this endpoint...");
    let existing_subscriptions = view
        .list_notification_subscriptions()
        .await
        .expect("Failed to list notification subscriptions");

    // Delete any existing subscription with our endpoint
    for sub in &existing_subscriptions {
        if sub.endpoint == webhook_endpoint {
            println!(
                "   Found existing subscription ({}), deleting it...",
                sub.id
            );
            view.delete_notification_subscription(&sub.id)
                .await
                .expect("Failed to delete existing subscription");
            println!("   ✅ Deleted existing subscription");
        }
    }

    // 2. Create a new subscription
    println!("\n2️⃣  Creating a new notification subscription...");
    let create_builder = CreateNotificationSubscriptionBodyBuilder::new(webhook_endpoint.clone())
        .add_notification_type(NotificationType::ContractsAll)
        .add_notification_type(NotificationType::ContractsEventLog);

    let created_subscription = view
        .create_notification_subscription(create_builder)
        .await
        .expect("Failed to create notification subscription");

    println!("✅ Subscription created!");
    println!("   ID: {}", created_subscription.id);
    println!("   Name: {}", created_subscription.name);
    println!("   Endpoint: {}", created_subscription.endpoint);
    println!("   Enabled: {}", created_subscription.enabled);
    println!(
        "   Notification types: {:?}",
        created_subscription.notification_types
    );

    let subscription_id = created_subscription.id.clone();

    assert!(!subscription_id.is_empty());
    assert_eq!(created_subscription.endpoint, webhook_endpoint);

    // 3. List subscriptions to verify it was created
    println!("\n3️⃣  Listing notification subscriptions...");
    let subscriptions = view
        .list_notification_subscriptions()
        .await
        .expect("Failed to list notification subscriptions");

    println!(
        "✅ Found {} subscriptions (including our newly created one)",
        subscriptions.len()
    );

    // Verify our subscription is in the list
    let found = subscriptions.iter().any(|s| s.id == subscription_id);
    assert!(found, "Created subscription should be in the list");
    println!("✅ Verified subscription appears in the list");

    // 4. Get the subscription by ID
    println!("\n4️⃣  Getting notification by ID...");
    let fetched_subscription = view
        .get_notification(&subscription_id)
        .await
        .expect("Failed to get notification");

    println!("✅ Subscription fetched: {}", fetched_subscription.id);
    assert_eq!(fetched_subscription.id, subscription_id);

    // 5. Update the subscription
    println!("\n5️⃣  Updating notification subscription...");
    let update_builder = UpdateNotificationSubscriptionBodyBuilder::new(subscription_id.clone())
        .name("Updated Test Subscription".to_string())
        .enabled(false);

    let updated_subscription = view
        .update_notification_subscription(update_builder)
        .await
        .expect("Failed to update notification subscription");

    println!("✅ Subscription updated!");
    println!("   ID: {}", updated_subscription.id);
    println!("   Name: {}", updated_subscription.name);
    println!("   Enabled: {}", updated_subscription.enabled);

    assert_eq!(updated_subscription.id, subscription_id);
    assert_eq!(updated_subscription.name, "Updated Test Subscription");
    assert!(!updated_subscription.enabled);

    // 6. Delete the subscription
    println!("\n6️⃣  Deleting notification subscription...");
    view.delete_notification_subscription(&subscription_id)
        .await
        .expect("Failed to delete notification subscription");

    println!("✅ Subscription deleted: {}", subscription_id);

    // Verify deletion
    match view.get_notification(&subscription_id).await {
        Err(_) => println!("✅ Confirmed: Subscription no longer exists"),
        Ok(_) => panic!("Subscription still exists after deletion"),
    }
}

#[tokio::test]
async fn test_get_notification_sig_pub_key() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("🔑 Testing get notification signature public key...");

    // Note: This endpoint is used to verify webhook signatures from Circle.
    //
    // How to get a public key ID:
    // 1. Create a notification subscription (see test_notification_subscriptions_crud)
    // 2. Trigger an event that sends a webhook (e.g., deploy a contract)
    // 3. Check the webhook headers for:
    //    - X-Circle-Signature: digital signature generated by Circle
    //    - X-Circle-Key-Id: UUID to use as the public key ID
    // 4. Use this UUID to retrieve the public key for signature verification
    //
    // Set CIRCLE_TEST_PUBLIC_KEY_ID env var to test this endpoint.

    let public_key_id = match env::var("CIRCLE_TEST_PUBLIC_KEY_ID") {
        Ok(key_id) => key_id,
        Err(_) => {
            println!("⚠️  CIRCLE_TEST_PUBLIC_KEY_ID not set. Skipping test.");
            println!("   To run this test:");
            println!("   1. Create a webhook subscription");
            println!("   2. Trigger a webhook event");
            println!("   3. Get the X-Circle-Key-Id header from the webhook");
            println!("   4. Export it: export CIRCLE_TEST_PUBLIC_KEY_ID=\"your-uuid-from-header\"");
            println!("   5. Run this test to retrieve and verify the public key");
            return;
        }
    };

    let public_key = view
        .get_notification_sig_pub_key(&public_key_id)
        .await
        .expect("Failed to get notification signature public key");

    println!("✅ Public key retrieved: {}", public_key);
    assert!(!public_key.is_empty());
}

#[tokio::test]
async fn test_list_contracts_with_pagination() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("📄 Testing list contracts with pagination...");

    let params = ListContractsParams {
        address: None,
        blockchain: None,
        template_id: None,
        ref_id: None,
        from: None,
        to: None,
        pagination: PaginationParams {
            page_size: Some(5),
            page_before: None,
            page_after: None,
        },
    };

    let contracts = view
        .list_contracts(Some(params))
        .await
        .expect("Failed to list contracts with pagination");

    println!(
        "✅ Retrieved page with {} contracts (max 5)",
        contracts.contracts.len()
    );
    assert!(contracts.contracts.len() <= 5);
}

#[tokio::test]
async fn test_list_contracts_by_blockchain() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("⛓️  Testing list contracts filtered by blockchain...");

    let blockchains = vec![
        Blockchain::EthSepolia,
        Blockchain::MaticAmoy,
        Blockchain::AvaxFuji,
    ];

    for blockchain in blockchains {
        let params = ListContractsParams {
            address: None,
            blockchain: Some(blockchain.clone()),
            template_id: None,
            ref_id: None,
            from: None,
            to: None,
            pagination: PaginationParams::default(),
        };

        let contracts = view
            .list_contracts(Some(params))
            .await
            .expect("Failed to list contracts by blockchain");

        println!(
            "✅ Found {} contracts on {}",
            contracts.contracts.len(),
            blockchain.as_str()
        );

        // Verify all contracts are on the correct blockchain
        for contract in &contracts.contracts {
            if let Some(ref bc) = contract.blockchain {
                assert_eq!(bc.to_uppercase(), blockchain.as_str().to_uppercase());
            }
        }
    }
}

#[tokio::test]
async fn test_notification_type_serialization() {
    dotenv::dotenv().ok();

    println!("🔄 Testing notification type serialization...");

    // Test all notification type enum values
    let notification_types = vec![
        NotificationType::All,
        NotificationType::TransactionsAll,
        NotificationType::TransactionsInbound,
        NotificationType::TransactionsOutbound,
        NotificationType::ChallengesAll,
        NotificationType::ContractsAll,
        NotificationType::ContractsEventLog,
        NotificationType::ModularWalletAll,
    ];

    for notification_type in notification_types {
        let as_str = notification_type.as_str();
        println!("   {:?} -> {}", notification_type, as_str);
        assert!(!as_str.is_empty());
    }

    println!("✅ All notification types serialize correctly");
}

#[tokio::test]
async fn test_query_contract() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("🔍 Testing contract query...");

    // Use USDC on Sepolia - a well-known ERC20 token that's always available and indexed
    let usdc_sepolia = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";

    // Standard ERC20 ABI for view functions
    let abi_json = r#"[{"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"name","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"symbol","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"decimals","outputs":[{"internalType":"uint8","name":"","type":"uint8"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"balanceOf","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#;

    println!("📋 Using USDC on Sepolia: {}", usdc_sepolia);
    println!("   This is a well-known contract that's always available!");
    println!("   ✅ No deployment needed - instant test\n");

    // Query 1: totalSupply()
    println!("🔍 Query 1: Querying totalSupply()...");

    let total_supply_result = view
        .query_contract(
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, usdc_sepolia.to_string())
                .abi_function_signature("totalSupply()".to_string())
                .abi_parameters(vec![])
                .abi_json(abi_json.to_string()),
        )
        .await
        .expect("Failed to query USDC totalSupply");

    println!("✅ totalSupply() query successful!");
    println!("   Output values: {:?}", total_supply_result.output_values);
    println!("   Output data (hex): {}", total_supply_result.output_data);

    assert!(!total_supply_result.output_data.is_empty());

    // Query 2: name()
    println!("\n🔍 Query 2: Querying name()...");

    let name_result = view
        .query_contract(
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, usdc_sepolia.to_string())
                .abi_function_signature("name()".to_string())
                .abi_parameters(vec![]),
        )
        .await
        .expect("Failed to query USDC name");

    println!("✅ name() query successful!");
    println!("   Token name: {:?}", name_result.output_values);

    // Query 3: symbol()
    println!("\n🔍 Query 3: Querying symbol()...");

    let symbol_result = view
        .query_contract(
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, usdc_sepolia.to_string())
                .abi_function_signature("symbol()".to_string())
                .abi_parameters(vec![]),
        )
        .await
        .expect("Failed to query USDC symbol");

    println!("✅ symbol() query successful!");
    println!("   Token symbol: {:?}", symbol_result.output_values);

    // Query 4: decimals()
    println!("\n🔍 Query 4: Querying decimals()...");

    let decimals_result = view
        .query_contract(
            QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, usdc_sepolia.to_string())
                .abi_function_signature("decimals()".to_string())
                .abi_parameters(vec![]),
        )
        .await
        .expect("Failed to query USDC decimals");

    println!("✅ decimals() query successful!");
    println!("   Decimals: {:?}", decimals_result.output_values);

    println!("\n🎉 All USDC contract queries successful!");
}

#[tokio::test]
async fn test_list_event_logs() {
    dotenv::dotenv().ok();

    let view = CircleView::new().expect("Failed to create CircleView");

    println!("📜 Testing list event logs...");
    println!("=======================================\n");

    // List all event logs (no filters)
    println!("1️⃣  Listing all event logs...");

    let all_logs = view
        .list_event_logs(None)
        .await
        .expect("Failed to list all event logs");

    println!("✅ Found {} event logs total", all_logs.event_logs.len());

    if !all_logs.event_logs.is_empty() {
        let log = &all_logs.event_logs[0];
        println!("\n   Sample event log:");
        println!("   - ID: {}", log.id);
        println!("   - Contract: {}", log.contract_address);
        println!("   - Event: {}", log.event_signature);
        println!("   - Block Height: {}", log.block_height);
        println!("   - Tx Hash: {}", log.tx_hash);
        println!("   - First Confirmed: {}", log.first_confirm_date);
    }

    // Test 2: List event logs filtered by blockchain
    println!("\n2️⃣  Listing event logs filtered by blockchain...");

    let params = ListEventLogsParams {
        contract_address: None,
        blockchain: Some(Blockchain::EthSepolia),
        from: None,
        to: None,
        pagination: PaginationParams::default(),
    };

    let blockchain_logs = view
        .list_event_logs(Some(params))
        .await
        .expect("Failed to list event logs by blockchain");

    println!(
        "✅ Found {} event logs on ETH-SEPOLIA",
        blockchain_logs.event_logs.len()
    );

    // Verify all logs are on the correct blockchain
    for log in &blockchain_logs.event_logs {
        // The blockchain should match ETH-SEPOLIA
        println!(
            "   - Event: {} at block {}",
            log.event_signature, log.block_height
        );
    }

    // Test 3: List event logs filtered by contract address (if we have logs)
    if !all_logs.event_logs.is_empty() {
        println!("\n3️⃣  Listing event logs filtered by contract address...");

        let contract_addr = &all_logs.event_logs[0].contract_address;

        let params2 = ListEventLogsParams {
            contract_address: Some(contract_addr.clone()),
            blockchain: None,
            from: None,
            to: None,
            pagination: PaginationParams::default(),
        };

        let contract_logs = view
            .list_event_logs(Some(params2))
            .await
            .expect("Failed to list event logs by contract");

        println!(
            "✅ Found {} event logs for contract {}",
            contract_logs.event_logs.len(),
            contract_addr
        );

        // All filtered logs should be for the correct contract
        for log in &contract_logs.event_logs {
            assert_eq!(
                log.contract_address.to_lowercase(),
                contract_addr.to_lowercase()
            );
        }
    }

    println!("\n🎉 All event log tests passed successfully!");
    println!("   ✅ Listed all event logs");
    println!("   ✅ Listed event logs by blockchain");
    if !all_logs.event_logs.is_empty() {
        println!("   ✅ Listed event logs by contract address");
    }
}

#[tokio::test]
async fn test_create_event_monitor() {
    dotenv::dotenv().ok();

    let _ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    println!("📡 Testing create event monitor...");
    println!("=======================================\n");

    // Step 0: Find a contract with events in your Circle account
    println!("\n0️⃣  Finding a contract with events for testing...");

    // List all contracts to find one with events defined
    let all_contracts = view
        .list_contracts(None)
        .await
        .expect("Failed to list contracts");

    // Find a contract that has events in its ABI (required for event monitoring)
    let test_contract = all_contracts.contracts.iter().find(|c| {
        c.contract_address.is_some()
            && c.events.is_some()
            && !c.events.as_ref().unwrap().is_null()
            && c.events
                .as_ref()
                .unwrap()
                .as_array()
                .map(|arr| !arr.is_empty())
                .unwrap_or(false)
            && c.blockchain
                .as_ref()
                .map(|b| b.contains("SEPOLIA"))
                .unwrap_or(false)
    });

    let (contract_address, _contract_name, event_signatures) = match test_contract {
        Some(contract) => {
            let addr = contract.contract_address.as_ref().unwrap().clone();
            let name = contract
                .name
                .clone()
                .unwrap_or_else(|| "Unknown".to_string());

            // Extract event signatures from the contract's events
            let events = contract.events.as_ref().unwrap().as_array().unwrap();
            let mut sigs = Vec::new();

            for event in events {
                if let Some(event_name) = event.get("name").and_then(|n| n.as_str()) {
                    // Try to construct the event signature from the event definition
                    if let Some(inputs) = event.get("inputs").and_then(|i| i.as_array()) {
                        let params: Vec<String> = inputs
                            .iter()
                            .filter_map(|input| {
                                input
                                    .get("type")
                                    .and_then(|t| t.as_str())
                                    .map(|s| s.to_string())
                            })
                            .collect();

                        let signature = format!("{}({})", event_name, params.join(","));
                        sigs.push(signature);
                    }
                }
            }

            println!("✅ Found contract with events:");
            println!("   Name: {}", name);
            println!("   Address: {}", addr);
            println!("   Contract ID: {:?}", contract.id);
            println!("   Events available: {}", sigs.len());
            for (i, sig) in sigs.iter().enumerate() {
                println!("      {}. {}", i + 1, sig);
            }

            if sigs.is_empty() {
                println!("   ⚠️  Contract has events array but couldn't extract signatures");
                return;
            }

            (addr, name, sigs)
        }
        None => {
            println!("⚠️  No contracts with events found in your Circle account.");
            println!("   Event monitors require contracts that:");
            println!("   1. Were deployed through Circle, AND");
            println!("   2. Have an ABI with event definitions");
            println!();
            println!("   To test event monitors:");
            println!("   1. Run: cargo run --example deploy_contract_example");
            println!("   2. Then run this test again");
            println!();
            println!("   Skipping event monitor tests for now...");
            return;
        }
    };

    // Test 1: Create event monitor for the first event
    println!("\n1️⃣  Creating event monitor for first event...");

    // Use the first event signature from the contract
    let first_event_signature = &event_signatures[0];
    println!("   Event: {}", first_event_signature);

    // Generate unique idempotency key
    let idempotency_key = uuid::Uuid::new_v4().to_string();
    println!("   Idempotency Key: {}", idempotency_key);

    let builder = CreateEventMonitorBodyBuilder::new(
        idempotency_key.clone(),
        first_event_signature.clone(),
        contract_address.clone(),
        Blockchain::EthSepolia,
    );

    let response = view
        .create_event_monitor(builder)
        .await
        .expect("Failed to create event monitor");

    println!("✅ Event monitor created successfully!");
    println!("   Monitor ID: {}", response.event_monitor.id);
    println!(
        "   Contract Address: {}",
        response.event_monitor.contract_address
    );
    println!(
        "   Event Signature: {}",
        response.event_monitor.event_signature
    );
    println!(
        "   Event Signature Hash: {}",
        response.event_monitor.event_signature_hash
    );
    println!("   Is Enabled: {}", response.event_monitor.is_enabled);

    // Verify the response
    assert!(!response.event_monitor.id.is_empty());
    assert_eq!(
        response.event_monitor.contract_address.to_lowercase(),
        contract_address.to_lowercase()
    );
    assert_eq!(
        response.event_monitor.event_signature,
        *first_event_signature
    );
    assert!(!response.event_monitor.event_signature_hash.is_empty());
    assert!(response.event_monitor.is_enabled);

    // Test 2: Create event monitor for second event (if available)
    println!("\n2️⃣  Creating event monitor for second event...");

    let second_event_signature = if event_signatures.len() > 1 {
        &event_signatures[1]
    } else {
        // Use the same event again if only one is available
        &event_signatures[0]
    };
    println!("   Event: {}", second_event_signature);

    let idempotency_key2 = uuid::Uuid::new_v4().to_string();
    println!("   Idempotency Key: {}", idempotency_key2);

    let builder2 = CreateEventMonitorBodyBuilder::new(
        idempotency_key2.clone(),
        second_event_signature.clone(),
        contract_address.clone(),
        Blockchain::EthSepolia,
    );

    let response2 = view
        .create_event_monitor(builder2)
        .await
        .expect("Failed to create event monitor for second event");

    println!("✅ Second event monitor created successfully!");
    println!("   Monitor ID: {}", response2.event_monitor.id);
    println!(
        "   Event Signature: {}",
        response2.event_monitor.event_signature
    );

    // Verify the response
    assert!(!response2.event_monitor.id.is_empty());
    assert_eq!(
        response2.event_monitor.contract_address.to_lowercase(),
        contract_address.to_lowercase()
    );
    assert_eq!(
        response2.event_monitor.event_signature,
        *second_event_signature
    );

    // Test 3: Test idempotency - creating with same idempotency key should return the same monitor
    println!("\n3️⃣  Testing idempotency with same idempotency key...");

    let builder3 = CreateEventMonitorBodyBuilder::new(
        idempotency_key.clone(), // Reuse the first idempotency key
        first_event_signature.clone(),
        contract_address.clone(),
        Blockchain::EthSepolia,
    );

    let response3 = view
        .create_event_monitor(builder3)
        .await
        .expect("Failed to create event monitor with same idempotency key");

    println!("✅ Idempotency test successful!");
    println!("   Monitor ID: {}", response3.event_monitor.id);
    println!(
        "   Should be same as first monitor: {}",
        response.event_monitor.id
    );

    // With idempotency, it should return the same monitor ID
    assert_eq!(response3.event_monitor.id, response.event_monitor.id);

    // Test 4: Update event monitor (disable it)
    println!("\n4️⃣  Testing update event monitor - disabling it...");

    let update_builder =
        UpdateEventMonitorBodyBuilder::new(response.event_monitor.id.clone(), false);

    let updated_response = view
        .update_event_monitor(update_builder)
        .await
        .expect("Failed to update event monitor");

    println!("✅ Event monitor updated successfully!");
    println!("   Monitor ID: {}", updated_response.event_monitor.id);
    println!(
        "   Is Enabled: {}",
        updated_response.event_monitor.is_enabled
    );

    // Verify the update
    assert_eq!(updated_response.event_monitor.id, response.event_monitor.id);
    assert!(!updated_response.event_monitor.is_enabled); // Should be disabled now

    // Test 5: Update event monitor (re-enable it)
    println!("\n5️⃣  Testing update event monitor - re-enabling it...");

    let update_builder2 =
        UpdateEventMonitorBodyBuilder::new(response.event_monitor.id.clone(), true);

    let updated_response2 = view
        .update_event_monitor(update_builder2)
        .await
        .expect("Failed to re-enable event monitor");

    println!("✅ Event monitor re-enabled successfully!");
    println!("   Monitor ID: {}", updated_response2.event_monitor.id);
    println!(
        "   Is Enabled: {}",
        updated_response2.event_monitor.is_enabled
    );

    // Verify the update
    assert_eq!(
        updated_response2.event_monitor.id,
        response.event_monitor.id
    );
    assert!(updated_response2.event_monitor.is_enabled); // Should be enabled again

    // Test 6: List all event monitors
    println!("\n6️⃣  Testing list all event monitors...");

    let all_monitors = view
        .list_event_monitors(None)
        .await
        .expect("Failed to list all event monitors");

    println!(
        "✅ Found {} event monitors total",
        all_monitors.event_monitors.len()
    );

    // Verify our monitors are in the list
    let has_transfer = all_monitors
        .event_monitors
        .iter()
        .any(|m| m.id == response.event_monitor.id);
    let has_approval = all_monitors
        .event_monitors
        .iter()
        .any(|m| m.id == response2.event_monitor.id);

    assert!(has_transfer, "Transfer monitor should be in the list");
    assert!(has_approval, "Approval monitor should be in the list");

    // Test 7: List event monitors filtered by contract address
    println!("\n7️⃣  Testing list event monitors filtered by contract address...");

    let params = ListEventMonitorsParams {
        contract_address: Some(contract_address.clone()),
        blockchain: None,
        event_signature: None,
        from: None,
        to: None,
        pagination: PaginationParams::default(),
    };

    let filtered_monitors = view
        .list_event_monitors(Some(params))
        .await
        .expect("Failed to list filtered event monitors");

    println!(
        "✅ Found {} event monitors for contract {}",
        filtered_monitors.event_monitors.len(),
        contract_address
    );

    // All filtered monitors should be for the correct contract
    for monitor in &filtered_monitors.event_monitors {
        assert_eq!(
            monitor.contract_address.to_lowercase(),
            contract_address.to_lowercase()
        );
    }

    // Test 8: List event monitors filtered by blockchain
    println!("\n8️⃣  Testing list event monitors filtered by blockchain...");

    let params2 = ListEventMonitorsParams {
        contract_address: None,
        blockchain: Some(Blockchain::EthSepolia),
        event_signature: None,
        from: None,
        to: None,
        pagination: PaginationParams::default(),
    };

    let blockchain_filtered = view
        .list_event_monitors(Some(params2))
        .await
        .expect("Failed to list event monitors by blockchain");

    println!(
        "✅ Found {} event monitors on ETH-SEPOLIA",
        blockchain_filtered.event_monitors.len()
    );

    // Test 9: Delete the Transfer event monitor
    println!("\n9️⃣  Testing delete event monitor...");

    view.delete_event_monitor(&response.event_monitor.id)
        .await
        .expect("Failed to delete Transfer event monitor");

    println!("✅ Transfer event monitor deleted successfully!");

    // Test 10: Delete the Approval event monitor
    println!("\n🔟 Deleting Approval event monitor...");

    view.delete_event_monitor(&response2.event_monitor.id)
        .await
        .expect("Failed to delete Approval event monitor");

    println!("✅ Approval event monitor deleted successfully!");

    println!("\n🎉 All event monitor tests passed successfully!");
    println!(
        "   ✅ First event monitor created: {}",
        first_event_signature
    );
    println!(
        "   ✅ Second event monitor created: {}",
        second_event_signature
    );
    println!("   ✅ Idempotency verified");
    println!("   ✅ Event monitor disabled");
    println!("   ✅ Event monitor re-enabled");
    println!("   ✅ Listed all event monitors");
    println!("   ✅ Listed event monitors by contract address");
    println!("   ✅ Listed event monitors by blockchain");
    println!("   ✅ First event monitor deleted");
    println!("   ✅ Second event monitor deleted");
}
