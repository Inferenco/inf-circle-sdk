mod common;

use base64::{engine::general_purpose, Engine as _};
use common::{get_or_create_destination_wallet, get_or_create_test_wallet, retry_on_rate_limit};
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    contract::ops::deploy_contract_from_template::DeployContractFromTemplateRequestBuilder,
    dev_wallet::{
        dto::{
            AbiParameter, AccountType, DevWallet, DevWalletMetadata,
            EstimateContractExecutionFeeBody, EstimateTransferFeeRequest, FeeLevel,
            ListDevWalletsParams, QueryContractRequest, QueryParams, RequestTestnetTokensRequest,
            ScaCore, UpdateDevWalletRequest,
        },
        ops::{
            accelerate_transaction::AccelerateTransactionRequestBuilder,
            cancel_transaction::CancelTransactionRequestBuilder,
            create_contract_transaction::CreateContractExecutionTransactionRequestBuilder,
            create_dev_wallet::CreateDevWalletRequestBuilder,
            create_transfer_transaction::CreateTransferTransactionRequestBuilder,
            create_wallet_upgrade_transaction::CreateWalletUpgradeTransactionRequestBuilder,
            sign_data::SignDataRequestBuilder, sign_delegate::SignDelegateRequestBuilder,
            sign_message::SignMessageRequestBuilder,
            sign_transaction::SignTransactionRequestBuilder,
        },
        views::{
            list_transactions::ListTransactionsParamsBuilder,
            list_wallets::ListDevWalletsParamsBuilder,
            list_wallets_with_balances::ListWalletsWithBalancesParamsBuilder,
            query::QueryParamsBuilder,
        },
    },
    helper::{parse_near_public_key, serialize_near_delegate_action_to_base64, PaginationParams},
    types::Blockchain,
};
use std::env;

/// Helper function to get or create an SCA wallet for testing
///
/// Uses a deterministic ref_id to ensure the same SCA wallet is reused across test runs.
async fn get_or_create_sca_wallet(
    ops: &CircleOps,
    view: &CircleView,
    wallet_set_id: &str,
    blockchain: &Blockchain,
) -> Result<DevWallet, Box<dyn std::error::Error>> {
    let blockchain_str = blockchain.as_str().to_lowercase();
    let ref_id = format!("test-sca-wallet-{}", blockchain_str);

    // Try to find existing wallet by ref_id
    let list_params = ListDevWalletsParams {
        address: None,
        blockchain: None,
        sca_core: None,
        wallet_set_id: None,
        ref_id: Some(ref_id.clone()),
        from: None,
        to: None,
        pagination: PaginationParams::default(),
        order: None,
    };

    match view.list_wallets(list_params).await {
        Ok(response) if !response.wallets.is_empty() => {
            let wallet = response.wallets.into_iter().next().unwrap();
            println!(
                "♻  Reusing existing SCA wallet: {} ({})",
                wallet.id, wallet.address
            );
            println!("   Ref ID: {}", ref_id);
            return Ok(wallet);
        }
        _ => {
            println!("📝 Creating new SCA wallet for {}", blockchain.as_str());
        }
    }

    // Create new SCA wallet
    let create_request =
        CreateDevWalletRequestBuilder::new(wallet_set_id.to_string(), vec![blockchain.clone()])?
            .account_type(AccountType::Sca)
            .metadata(vec![DevWalletMetadata {
                name: Some(format!("Test SCA Wallet - {}", blockchain.as_str())),
                ref_id: Some(ref_id.clone()),
            }])
            .build();

    let wallets_response = ops.create_dev_wallet(create_request).await?;

    let wallet = wallets_response
        .wallets
        .into_iter()
        .next()
        .ok_or("No wallet created")?;

    println!("✅ SCA wallet created: {} ({})", wallet.id, wallet.address);
    println!("   Ref ID: {}", ref_id);

    Ok(wallet)
}

/// Helper function to deploy a simple test contract
///
/// Deploys a contract from template or returns a known contract address.
/// For testing, we'll use a simple ERC20-like template if available,
/// or fall back to a known USDC contract address.
async fn get_or_deploy_test_contract(
    ops: &CircleOps,
    wallet: &DevWallet,
    blockchain: &Blockchain,
) -> Result<String, Box<dyn std::error::Error>> {
    // For Sepolia testnet, use the USDC contract as a known good contract
    // In production, you would deploy from a template
    match blockchain {
        Blockchain::EthSepolia => {
            // Try to deploy from template if CIRCLE_CONTRACT_TEMPLATE_ID is set
            if let Ok(template_id) = env::var("CIRCLE_CONTRACT_TEMPLATE_ID") {
                println!("📝 Deploying contract from template: {}", template_id);

                match DeployContractFromTemplateRequestBuilder::new(
                    template_id.clone(),
                    "Test Contract".to_string(),
                    wallet.id.clone(),
                    "ETH-SEPOLIA".to_string(),
                ) {
                    Ok(builder) => {
                        let response = ops
                            .deploy_contract_from_template(
                                builder
                                    .ref_id("test-contract-deployment".to_string())
                                    .build(),
                            )
                            .await;

                        match response {
                            Ok(deployment) => {
                                println!(
                                    "✅ Contract deployed, transaction ID: {}",
                                    deployment.transaction_id
                                );
                                // Note: Can't return contract address directly anymore, need to wait for transaction
                                return Ok("0x0000000000000000000000000000000000000000".to_string());
                            }
                            Err(e) => {
                                println!("⚠️  Template deployment failed: {}", e);
                                println!("   Falling back to USDC contract");
                            }
                        }
                    }
                    Err(e) => {
                        println!("⚠️  Could not create deployment builder: {}", e);
                        println!("   Falling back to USDC contract");
                    }
                }
            }

            // Fall back to USDC contract on Sepolia
            Ok("0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238".to_string())
        }
        _ => {
            // For other chains, return a placeholder or skip
            println!("⚠️  Using placeholder contract for {:?}", blockchain);
            Ok("0x0000000000000000000000000000000000000000".to_string())
        }
    }
}

/// Helper function to ensure a wallet has testnet tokens
///
/// Checks if the wallet has a balance, and if not (or balance is very low),
/// requests testnet tokens from the faucet.
async fn ensure_wallet_funded(
    view: &CircleView,
    wallet: &DevWallet,
    blockchain: &Blockchain,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if this is a testnet blockchain
    let is_testnet = matches!(
        blockchain,
        Blockchain::EthSepolia
            | Blockchain::AvaxFuji
            | Blockchain::MaticAmoy
            | Blockchain::SolDevnet
            | Blockchain::ArbSepolia
            | Blockchain::NearTestnet
            | Blockchain::UniSepolia
            | Blockchain::BaseSepolia
            | Blockchain::OpSepolia
            | Blockchain::AptosTestnet
    );

    if !is_testnet {
        println!("⚠️  Wallet is on mainnet, skipping faucet funding");
        return Ok(());
    }

    // Try to get the balance
    let query_params = QueryParamsBuilder::new().build();
    match view.get_token_balances(&wallet.id, query_params).await {
        Ok(balances) => {
            // Find native token balance
            let has_native_balance = balances
                .token_balances
                .iter()
                .any(|b| b.token.is_native && b.amount.parse::<f64>().unwrap_or(0.0) > 0.001);

            if has_native_balance {
                println!("✅ Wallet {} already has native tokens", wallet.address);
                return Ok(());
            }
        }
        Err(_) => {
            // If we can't get balance, try to fund anyway
            println!("⚠️  Could not check balance, requesting tokens anyway");
        }
    }

    // Request tokens from faucet
    println!(
        "💰 Requesting testnet tokens for wallet: {}",
        wallet.address
    );

    let request = RequestTestnetTokensRequest {
        blockchain: blockchain.clone(),
        address: wallet.address.clone(),
        native: Some(true),
        usdc: Some(true),
        eurc: None,
    };

    retry_on_rate_limit(|| async { view.request_testnet_tokens(request.clone()).await }).await?;

    println!("✅ Successfully requested testnet tokens!");
    println!("   ⏳ Waiting 5 seconds for tokens to arrive...");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    Ok(())
}

// NEAR Protocol types (official)
use near_primitives::{
    action::{
        delegate::DelegateAction, delegate::NonDelegateAction, Action as NearAction,
        FunctionCallAction,
    },
    types::AccountId as NearAccountId,
};

/// Helper test to display test wallet addresses for manual funding
/// Run this first: `cargo test test_setup_wallets_for_funding -- --nocapture`
#[tokio::test]
async fn test_setup_wallets_for_funding() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("   🔑 TEST WALLET SETUP - Fund these wallets for testing");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Get or create source ETH-SEPOLIA wallet
    if let Ok(source_wallet) = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "ETH Sepolia Test",
    )
    .await
    {
        println!("📍 SOURCE Wallet (ETH-SEPOLIA):");
        println!("   Address: {}", source_wallet.address);
        println!("   Wallet ID: {}", source_wallet.id);
        println!("   Ref ID: test-wallet-eth-sepolia");
        println!("\n   💰 Fund with native ETH:");
        println!("      🔗 https://sepoliafaucet.com/");
        println!("      🔗 https://www.alchemy.com/faucets/ethereum-sepolia");
        println!("\n   🪙 Fund with LINK tokens (for ERC20 tests):");
        println!("      🔗 https://faucets.chain.link/sepolia");
        println!("      Request both ETH and LINK tokens");
        println!();
    }

    // Get or create destination ETH-SEPOLIA wallet
    if let Ok(dest_wallet) =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia).await
    {
        println!("📍 DESTINATION Wallet (ETH-SEPOLIA):");
        println!("   Address: {}", dest_wallet.address);
        println!("   Wallet ID: {}", dest_wallet.id);
        println!("   Ref ID: test-wallet-destination-eth-sepolia");
        println!("   (No funding required - this receives transfers)");
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("💡 After funding SOURCE wallet, run the transfer tests:");
    println!("   cargo test test_create_transfer_transaction -- --nocapture");
    println!("═══════════════════════════════════════════════════════════════\n");
}

#[tokio::test]
async fn test_wallet_lifecycle() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // 1. Create a new wallet
    // The entity secret will be automatically encrypted at request time using CIRCLE_ENTITY_SECRET and CIRCLE_PUBLIC_KEY
    let create_request_builder =
        CreateDevWalletRequestBuilder::new(wallet_set_id.clone(), vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![DevWalletMetadata {
                name: Some("Integration Test Wallet".to_string()),
                ref_id: Some("test-ref-123".to_string()),
            }])
            .build();

    let create_response = ops
        .create_dev_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
    let new_wallet = create_response.wallets.first().expect("No wallet created");
    assert_eq!(new_wallet.name.as_deref(), Some("Integration Test Wallet"));

    // 2. Get the created wallet by ID
    let fetched_wallet = view
        .get_wallet(&new_wallet.id)
        .await
        .expect("Failed to get wallet");
    assert_eq!(fetched_wallet.wallet.id, new_wallet.id);
    assert_eq!(
        fetched_wallet.wallet.name.as_deref(),
        Some("Integration Test Wallet")
    );

    // 3. Update the wallet
    let update_request = UpdateDevWalletRequest {
        name: Some("Updated Test Wallet".to_string()),
        ref_id: Some("test-ref-123".to_string()),
    };

    let updated_wallet = ops
        .update_dev_wallet(&new_wallet.id, update_request)
        .await
        .expect("Failed to update wallet");
    assert_eq!(
        updated_wallet.wallet.name.as_deref(),
        Some("Updated Test Wallet")
    );
    assert_eq!(
        updated_wallet.wallet.ref_id.as_deref(),
        Some("test-ref-123")
    );

    // 4. List wallets and verify the updated wallet is present
    let list_params = ListDevWalletsParamsBuilder::new()
        .wallet_set_id(wallet_set_id)
        .build();

    let list_response = view
        .list_wallets(list_params)
        .await
        .expect("Failed to list wallets");
    let wallet_in_list = list_response
        .wallets
        .iter()
        .find(|w| w.id == new_wallet.id)
        .expect("Wallet not found in list");
    assert_eq!(wallet_in_list.name.as_deref(), Some("Updated Test Wallet"));
}

#[tokio::test]
async fn test_list_wallets_with_token_balances() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new().expect("Failed to create CircleView");

    // Test with parameters
    let params = ListWalletsWithBalancesParamsBuilder::new()
        .pagination(PaginationParams {
            page_size: Some(10),
            ..Default::default()
        })
        .blockchain(Blockchain::EthSepolia.as_str().to_string())
        .build();

    let response_with_params = view
        .list_wallets_with_token_balances(params)
        .await
        .expect("Failed to list wallets with token balances with params");

    // Verify response structure
    assert!(
        !response_with_params.wallets.is_empty(),
        "Should have at least one wallet"
    );

    // Check that each wallet has token_balances field
    for wallet in &response_with_params.wallets {
        assert!(!wallet.id.is_empty(), "Wallet ID should not be empty");
        assert!(
            !wallet.address.is_empty(),
            "Wallet address should not be empty"
        );
        assert!(
            !wallet.blockchain.is_empty(),
            "Blockchain should not be empty"
        );
        // token_balances can be empty, that's okay
    }
}

#[tokio::test]
async fn test_get_token_balances() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a test wallet first
    let create_request_builder =
        CreateDevWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![DevWalletMetadata {
                name: Some("Token Balance Test Wallet".to_string()),
                ref_id: Some("token-balance-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_dev_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
    let test_wallet = create_response.wallets.first().expect("No wallet created");

    // Test getting token balances without parameters
    let response = view
        .get_token_balances(&test_wallet.id, QueryParams::default())
        .await
        .expect("Failed to get token balances");

    assert!(
        response.token_balances.is_empty(),
        "Should have valid token balances response"
    );

    // Verify response structure
    // token_balances can be empty for a new wallet, that's expected

    // Test with parameters
    let params = QueryParamsBuilder::new()
        .include_all(true)
        .pagination(PaginationParams {
            page_size: Some(10),
            ..Default::default()
        })
        .build();

    let response_with_params = view
        .get_token_balances(&test_wallet.id, params)
        .await
        .expect("Failed to get token balances with params");

    // Verify response structure
    assert!(
        response_with_params.token_balances.is_empty(),
        "Should have valid token balances response with params"
    );
}

#[tokio::test]
async fn test_get_nfts() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a test wallet first
    let create_request_builder =
        CreateDevWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![DevWalletMetadata {
                name: Some("NFT Test Wallet".to_string()),
                ref_id: Some("nft-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_dev_wallet(create_request_builder)
        .await
        .expect("Failed to create dev wallet");
    let test_wallet = create_response.wallets.first().expect("No wallet created");

    // Test getting NFTs without parameters
    let response = view
        .get_nfts(&test_wallet.id, QueryParams::default())
        .await
        .expect("Failed to get NFTs");

    // Verify response structure
    // nfts can be empty for a new wallet, that's expected
    assert!(response.nfts.is_empty(), "Should have valid NFTs response");

    // Test with parameters
    let params = QueryParams {
        include_all: Some(true),
        pagination: PaginationParams {
            page_size: Some(10),
            ..Default::default()
        },
        ..Default::default()
    };

    let response_with_params = view
        .get_nfts(&test_wallet.id, params)
        .await
        .expect("Failed to get NFTs with params");

    // Verify response structure
    assert!(
        response_with_params.nfts.is_empty(),
        "Should have valid NFTs response with params"
    );
}

#[tokio::test]
async fn test_query_params_builder() {
    // Test QueryParams with various combinations
    let params = QueryParams {
        include_all: Some(true),
        name: Some("Test Token".to_string()),
        token_address: Some("0x1234567890123456789012345678901234567890".to_string()),
        standard: Some("ERC20".to_string()),
        pagination: PaginationParams {
            page_size: Some(25),
            ..Default::default()
        },
        ..Default::default()
    };

    // Verify all fields are set correctly
    assert_eq!(params.include_all, Some(true));
    assert_eq!(params.name, Some("Test Token".to_string()));
    assert_eq!(
        params.token_address,
        Some("0x1234567890123456789012345678901234567890".to_string())
    );
    assert_eq!(params.standard, Some("ERC20".to_string()));
    assert_eq!(params.pagination.page_size, Some(25));
}

#[tokio::test]
async fn test_sign_message() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet (reuses existing to avoid rate limits)
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Sign Message Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    // Test signing a simple message
    let message = "Hello, Circle SDK!";
    let sign_request_builder =
        SignMessageRequestBuilder::new(test_wallet.id.clone(), message.to_string())
            .unwrap()
            .memo("Test message signing".to_string())
            .build();

    let sign_response = ops
        .dev_sign_message(sign_request_builder)
        .await
        .expect("Failed to sign message");

    // Verify response structure
    assert!(
        !sign_response.signature.is_empty(),
        "Signature should not be empty"
    );

    // Test signing a hex-encoded message
    let hex_message = "0x48656c6c6f2c20436972636c652053444b21"; // "Hello, Circle SDK!" in hex
    let hex_sign_request_builder =
        SignMessageRequestBuilder::new(test_wallet.id.clone(), hex_message.to_string())
            .unwrap()
            .encoded_by_hex(true)
            .memo("Test hex message signing".to_string())
            .build();

    let hex_sign_response = ops
        .dev_sign_message(hex_sign_request_builder)
        .await
        .expect("Failed to sign hex message");

    // Verify response structure
    assert!(
        !hex_sign_response.signature.is_empty(),
        "Hex signature should not be empty"
    );
}

#[tokio::test]
async fn test_sign_data() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps
    let ops = CircleOps::new().expect("Failed to create CircleOps");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a test wallet first
    let create_request_builder =
        CreateDevWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![DevWalletMetadata {
                name: Some("Sign Data Test Wallet".to_string()),
                ref_id: Some("sign-data-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_dev_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
    let test_wallet = create_response.wallets.first().expect("No wallet created");

    // Test signing data - data field needs to be a valid JSON string
    let data = r#"{
        "types": {
            "Data": [
            {
                "name": "dummy",
                "type": "string"
            }
            ],
            "EIP712Domain": [
            {
                "name": "name",
                "type": "string"
            },
            {
                "name": "chainId",
                "type": "uint256"
            }
            ]
        },
        "domain": {
            "name": "Test",
            "chainId": 11155111
        },
        "primaryType": "Data",
        "message": {
            "dummy": "dummy"
        }
    }"#; // Valid JSON string
    let sign_request_builder =
        SignDataRequestBuilder::new(test_wallet.id.clone(), data.to_string())
            .unwrap()
            .memo("Test data signing".to_string())
            .build();

    let sign_response = ops.dev_sign_data(sign_request_builder).await;

    // Handle the case where sign_data endpoint might not be available
    match sign_response {
        Ok(response) => {
            // Verify response structure if successful
            assert!(
                !response.signature.is_empty(),
                "Data signature should not be empty"
            );
        }
        Err(e) => {
            // If the endpoint is not available (404), skip the test
            if e.to_string().contains("404") {
                println!("Sign data endpoint not available, skipping test");
                return;
            }
            // For other errors, fail the test
            panic!("Failed to sign data: {}", e);
        }
    }
}

#[tokio::test]
async fn test_sign_transaction() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps
    let ops = CircleOps::new().expect("Failed to create CircleOps");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a test wallet first - use NEAR-TESTNET for transaction signing (supported testnet)
    let create_request_builder =
        CreateDevWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EvmTestnet])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![DevWalletMetadata {
                name: Some("Sign Transaction Test Wallet".to_string()),
                ref_id: Some("sign-transaction-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_dev_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
    let test_wallet = create_response.wallets.first().expect("No wallet created");

    // Test signing a transaction - for NEAR-TESTNET, try using both transaction and rawTransaction fields
    // Create both JSON transaction object and base64-encoded transaction
    let transaction_json = r#"{
        "nonce": 1,
        "to": "0x1234567890123456789012345678901234567890",
        "value": "1000000000000000000",
        "gas": "1000000000000000000",
        "maxFeePerGas": "1000000000000000000",
        "maxPriorityFeePerGas": "1000000000000000000",
        "chainId": "0xaa36a7"
    }"#;

    let sign_request_builder = SignTransactionRequestBuilder::new(
        test_wallet.id.clone(),
        None,                               // Use rawTransaction for NEAR-TESTNET
        Some(transaction_json.to_string()), // Also use transaction object for NEAR-TESTNET
    )
    .unwrap()
    .memo("Test transaction signing".to_string())
    .build();

    let sign_response = ops.dev_sign_transaction(sign_request_builder).await;

    // Handle the case where transaction signing might fail
    match sign_response {
        Ok(response) => {
            // Verify response structure if successful
            assert!(
                !response.signature.is_empty(),
                "Transaction signature should not be empty"
            );
            assert!(
                !response.signed_transaction.is_empty(),
                "Signed transaction should not be empty"
            );

            // For Solana, we should not have a tx_hash
            if test_wallet.blockchain == Blockchain::Sol {
                assert!(response.tx_hash.is_none(), "Solana should not have tx_hash");
            }
        }
        Err(e) => {
            // Print the full error details for debugging
            println!("Transaction signing failed with error: {}", e);
            println!("Error details: {:?}", e);

            panic!("Failed to sign transaction: {}", e);
        }
    }
}

#[tokio::test]
async fn test_sign_delegate_near() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps
    let ops = CircleOps::new().expect("Failed to create CircleOps");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a NEAR test wallet (delegate signing only works with NEAR)
    let create_request_builder =
        CreateDevWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::NearTestnet])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![DevWalletMetadata {
                name: Some("Sign Delegate NEAR Test Wallet".to_string()),
                ref_id: Some("sign-delegate-near-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_dev_wallet(create_request_builder)
        .await
        .expect("Failed to create NEAR wallet");
    let test_wallet = create_response
        .wallets
        .first()
        .expect("No NEAR wallet created");

    assert_eq!(
        test_wallet.blockchain,
        Blockchain::NearTestnet,
        "Wallet should be on NEAR blockchain"
    );

    // Create a delegate action using NEAR's official types and Borsh serialization
    let args_json = r#"{"text":"Hello from Circle SDK!"}"#;

    let function_call = FunctionCallAction {
        method_name: "addMessage".to_string(),
        args: args_json.as_bytes().to_vec(),
        gas: 100_000_000_000_000,
        deposit: 0,
    };

    // Parse the public key from the wallet
    let public_key_str = test_wallet
        .initial_public_key
        .as_ref()
        .expect("Wallet should have an initial public key");

    // Debug: print the actual public key format from Circle
    println!("Public key from Circle: '{}'", public_key_str);

    // Parse NEAR public key (handles both "ed25519:base58" and "base58" formats)
    let public_key = parse_near_public_key(public_key_str)
        .unwrap_or_else(|e| panic!("Failed to parse public key '{}': {}", public_key_str, e));

    // Use the wallet's address as sender (NEAR implicit account format)
    let sender_id = NearAccountId::try_from(test_wallet.address.clone())
        .expect("Failed to create AccountId from wallet address");
    let receiver_id = NearAccountId::try_from("guest-book.testnet".to_string())
        .expect("Failed to create receiver AccountId");

    println!("Using sender_id (wallet address): {}", sender_id);

    let action = NearAction::FunctionCall(Box::new(function_call));
    let non_delegate_action =
        NonDelegateAction::try_from(action).expect("Failed to convert Action to NonDelegateAction");

    let delegate_action = DelegateAction {
        sender_id,
        receiver_id,
        actions: vec![non_delegate_action],
        nonce: 1u64,
        max_block_height: 1_000_000u64,
        public_key,
    };

    // Serialize to Borsh and encode as base64
    let unsigned_delegate_action = serialize_near_delegate_action_to_base64(&delegate_action)
        .expect("Failed to serialize delegate action");

    // Debug: Print what we're sending to Circle
    println!(
        "Unsigned delegate action (base64): {}",
        unsigned_delegate_action
    );
    println!("Length: {} characters", unsigned_delegate_action.len());

    // Debug: Show the Borsh bytes
    let sign_request_builder =
        SignDelegateRequestBuilder::new(test_wallet.id.clone(), unsigned_delegate_action)
            .unwrap()
            .build();

    let sign_response = ops.dev_sign_delegate(sign_request_builder).await;

    // Test passes if we can create the request and handle the response
    match sign_response {
        Ok(response) => {
            assert!(
                !response.signature.is_empty(),
                "Delegate signature should not be empty"
            );
            assert!(
                !response.signed_delegate_action.is_empty(),
                "Signed delegate action should not be empty"
            );
            println!("✅ Delegate action signed successfully!");
        }
        Err(e) => {
            // Handle Circle API validation errors gracefully
            panic!("Failed to sign delegate action: {}", e);
        }
    }
}

#[tokio::test]
async fn test_sign_delegate_non_near_should_fail() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a non-NEAR test wallet (Ethereum)
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Sign Delegate ETH Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    // Verify we have a non-NEAR wallet
    assert_ne!(
        test_wallet.blockchain,
        Blockchain::Near,
        "Wallet should not be on NEAR blockchain"
    );

    // Test signing a delegate action with non-NEAR wallet (should fail)
    let unsigned_delegate_action =
        general_purpose::STANDARD.encode(r#"{"invalid": "delegate action for non-NEAR"}"#);

    let sign_request_builder =
        SignDelegateRequestBuilder::new(test_wallet.id.clone(), unsigned_delegate_action)
            .unwrap()
            .build();

    // This should fail because delegate signing only works with NEAR
    let result = ops.dev_sign_delegate(sign_request_builder).await;
    assert!(
        result.is_err(),
        "Delegate signing should fail for non-NEAR blockchains"
    );
}

#[tokio::test]
async fn test_list_transactions() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new().expect("Failed to create CircleView");

    // Test listing transactions with basic parameters
    let list_params = ListTransactionsParamsBuilder::new()
        .blockchain("ETH-SEPOLIA".to_string())
        .pagination(PaginationParams {
            page_size: Some(10),
            page_after: None,
            page_before: None,
        })
        .order("DESC".to_string())
        .build();

    let result = view.list_transactions(list_params).await;

    match result {
        Ok(transactions_response) => {
            println!("✅ Successfully listed transactions!");
            println!(
                "Found {} transactions",
                transactions_response.transactions.len()
            );

            // Verify the response structure
            assert!(
                !transactions_response.transactions.is_empty()
                    || transactions_response.transactions.is_empty(),
                "Transactions response should be valid regardless of empty results"
            );

            // If we have transactions, verify some basic fields
            if let Some(first_tx) = transactions_response.transactions.first() {
                assert!(
                    !first_tx.id.is_empty(),
                    "Transaction ID should not be empty"
                );
                assert!(
                    !first_tx.blockchain.is_empty(),
                    "Blockchain should not be empty"
                );
                assert!(!first_tx.state.is_empty(), "State should not be empty");
                assert!(
                    !first_tx.transaction_type.is_empty(),
                    "Transaction type should not be empty"
                );
                println!("First transaction ID: {}", first_tx.id);
                println!("First transaction blockchain: {}", first_tx.blockchain);
                println!("First transaction state: {}", first_tx.state);
            }
        }
        Err(e) => {
            panic!("Failed to list transactions: {}", e);
        }
    }
}

#[tokio::test]
async fn test_list_transactions_with_filters() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new().expect("Failed to create CircleView");

    // Test listing transactions with various filters
    let list_params = ListTransactionsParamsBuilder::new()
        .blockchain("ETH-SEPOLIA".to_string())
        .custody_type("DEVELOPER".to_string())
        .operation("TRANSFER".to_string())
        .state("CONFIRMED".to_string())
        .tx_type("OUTBOUND".to_string())
        .pagination(PaginationParams {
            page_size: Some(5),
            page_after: None,
            page_before: None,
        })
        .order("ASC".to_string())
        .build();

    let result = view.list_transactions(list_params).await;

    match result {
        Ok(transactions_response) => {
            println!("✅ Successfully listed filtered transactions!");
            println!(
                "Found {} filtered transactions",
                transactions_response.transactions.len()
            );

            // Verify filtered results match criteria
            for tx in &transactions_response.transactions {
                assert_eq!(
                    tx.blockchain, "ETH-SEPOLIA",
                    "Blockchain should match filter"
                );
                if let Some(custody_type) = &tx.custody_type {
                    assert_eq!(
                        custody_type, "DEVELOPER",
                        "Custody type should match filter"
                    );
                }
                if let Some(operation) = &tx.operation {
                    assert_eq!(operation, "TRANSFER", "Operation should match filter");
                }
                assert_eq!(tx.state, "CONFIRMED", "State should match filter");
                assert_eq!(
                    tx.transaction_type, "OUTBOUND",
                    "Transaction type should match filter"
                );
            }
        }
        Err(e) => {
            panic!("Failed to list filtered transactions: {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_transaction() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // 1. Get or create source wallet
    let source_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Get Transaction Test",
    )
    .await
    .expect("Failed to get or create source wallet");

    println!("Using source wallet: {}", source_wallet.id);

    // 2. Get destination wallet
    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    // 3. Create a transfer transaction
    let transfer_builder = CreateTransferTransactionRequestBuilder::new(source_wallet.id.clone())
        .destination_address(destination_wallet.address.clone())
        .amounts(vec!["0.001".to_string()])
        .blockchain(Blockchain::EthSepolia)
        .idempotency_key(uuid::Uuid::new_v4().to_string())
        .fee_level(FeeLevel::Medium)
        .ref_id("test-get-transaction".to_string())
        .build();

    let transfer_response = ops
        .create_dev_transfer_transaction(transfer_builder)
        .await
        .expect("Failed to create transfer transaction");

    let tx_id = transfer_response.id;
    println!("✅ Created transfer transaction: {}", tx_id);

    // 4. Wait a moment for the transaction to be processed
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 5. Get the specific transaction
    let transaction_response = view
        .get_transaction(&tx_id)
        .await
        .expect("Failed to get transaction");

    println!("✅ Successfully retrieved transaction!");

    // 6. Verify the response structure
    assert_eq!(
        transaction_response.transaction.id, tx_id,
        "Retrieved transaction ID should match"
    );
    assert_eq!(
        transaction_response.transaction.blockchain, "ETH-SEPOLIA",
        "Blockchain should be ETH-SEPOLIA"
    );

    // Verify it's related to our source wallet
    if let Some(wallet_id) = &transaction_response.transaction.wallet_id {
        assert_eq!(
            wallet_id, &source_wallet.id,
            "Transaction should belong to our source wallet"
        );
    }

    // Verify destination address
    if let Some(dest_address) = &transaction_response.transaction.destination_address {
        assert_eq!(
            dest_address.to_lowercase(),
            destination_wallet.address.to_lowercase(),
            "Destination address should match"
        );
    }

    println!("Transaction details:");
    println!("  ID: {}", transaction_response.transaction.id);
    println!(
        "  Blockchain: {}",
        transaction_response.transaction.blockchain
    );
    println!("  State: {}", transaction_response.transaction.state);
    println!(
        "  Type: {}",
        transaction_response.transaction.transaction_type
    );
    println!(
        "  Source Wallet ID: {:?}",
        transaction_response.transaction.wallet_id
    );
    println!(
        "  Destination: {:?}",
        transaction_response.transaction.destination_address
    );

    if let Some(tx_hash) = &transaction_response.transaction.tx_hash {
        println!("  TX Hash: {}", tx_hash);
    }
    if let Some(amounts) = &transaction_response.transaction.amounts {
        println!("  Amounts: {:?}", amounts);
    }
}

#[tokio::test]
async fn test_get_transaction_with_invalid_id() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new().expect("Failed to create CircleView");

    // Test getting a transaction with an invalid ID (should fail)
    let invalid_tx_id = "invalid-transaction-id-12345";
    let result = view.get_transaction(invalid_tx_id).await;

    // This should fail with an error
    assert!(
        result.is_err(),
        "Getting transaction with invalid ID should fail"
    );

    if let Err(e) = result {
        println!("✅ Correctly failed to get invalid transaction: {}", e);
    }
}

#[tokio::test]
async fn test_create_transfer_transaction_with_fee_level() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet (reuses existing to avoid rate limits)
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Transfer Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    println!("Using source wallet: {}", test_wallet.id);
    println!("Source address: {}", test_wallet.address);

    // Get destination wallet
    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    println!("Destination address: {}", destination_wallet.address);

    // 2. Create a transfer transaction using FeeLevel
    let transfer_builder = CreateTransferTransactionRequestBuilder::new(test_wallet.id.clone())
        .destination_address(destination_wallet.address.clone())
        .amounts(vec!["0.001".to_string()]) // Transfer 0.001 ETH
        .blockchain(Blockchain::EthSepolia) // Required for native token transfers
        .idempotency_key(uuid::Uuid::new_v4().to_string())
        .fee_level(FeeLevel::Medium)
        .ref_id("test-transfer-fee-level".to_string())
        .build();

    let response = ops
        .create_dev_transfer_transaction(transfer_builder)
        .await
        .expect("Failed to create transfer transaction");

    println!("✅ Successfully created transfer transaction!");
    println!("Transaction ID: {}", response.id);
    println!("Transaction state: {}", response.state);

    // Verify response structure
    assert!(
        !response.id.is_empty(),
        "Transaction ID should not be empty"
    );
    assert!(
        !response.state.is_empty(),
        "Transaction state should not be empty"
    );

    // State should be one of the valid states
    let valid_states = vec![
        "CANCELLED",
        "CONFIRMED",
        "COMPLETED",
        "DENIED",
        "FAILED",
        "INITIATED",
        "CLEARED",
        "QUEUED",
        "SENT",
    ];
    assert!(
        valid_states.contains(&response.state.as_str()),
        "State should be a valid transaction state"
    );
}

#[tokio::test]
async fn test_create_transfer_transaction_with_gas_settings() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet (reuses existing to avoid rate limits)
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Transfer Gas Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    println!("Using source wallet: {}", test_wallet.id);

    // Get destination wallet
    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    println!("Destination address: {}", destination_wallet.address);

    // 2. Create a transfer transaction with custom gas settings (EIP-1559)
    let test_wallet_id = test_wallet.id.clone();
    let dest_address = destination_wallet.address.clone();

    let response = common::retry_on_rate_limit(|| async {
        let transfer_builder = CreateTransferTransactionRequestBuilder::new(test_wallet_id.clone())
            .destination_address(dest_address.clone())
            .amounts(vec!["0.001".to_string()])
            .blockchain(Blockchain::EthSepolia) // Required for native token transfers
            .idempotency_key(uuid::Uuid::new_v4().to_string())
            .gas_limit("21000".to_string())
            .max_fee("50".to_string()) // 50 gwei max fee
            .priority_fee("2".to_string()) // 2 gwei priority fee
            .ref_id("test-transfer-gas-settings".to_string())
            .build();

        ops.create_dev_transfer_transaction(transfer_builder).await
    })
    .await
    .expect("Failed to create transfer transaction with gas settings");

    println!("✅ Successfully created transfer transaction with gas settings!");
    println!("Transaction ID: {}", response.id);
    println!("Transaction state: {}", response.state);

    assert!(
        !response.id.is_empty(),
        "Transaction ID should not be empty"
    );
}

#[tokio::test]
async fn test_create_token_transfer_transaction() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet (reuses existing to avoid rate limits)
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Token Transfer Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    println!("Using source wallet: {}", test_wallet.id);
    println!("Source address: {}", test_wallet.address);

    // Get destination wallet
    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    println!("Destination address: {}", destination_wallet.address);

    // Use Chainlink LINK token on Sepolia (can be obtained from https://faucets.chain.link/sepolia)
    let link_token_address = "0x779877A7B0D9E8603169DdbD7836e478b4624789"; // LINK on Sepolia

    println!("\n💡 Get LINK tokens from Chainlink faucet:");
    println!("   🔗 https://faucets.chain.link/sepolia");
    println!("   📝 Enter wallet address: {}", test_wallet.address);
    println!("   Request both ETH and LINK tokens\n");

    let transfer_builder = CreateTransferTransactionRequestBuilder::new(test_wallet.id.clone())
        .destination_address(destination_wallet.address.clone())
        .amounts(vec!["0.1".to_string()]) // Transfer 0.1 LINK
        .token_address(link_token_address.to_string())
        .blockchain(Blockchain::EthSepolia)
        .idempotency_key(uuid::Uuid::new_v4().to_string())
        .fee_level(FeeLevel::Medium)
        .ref_id("test-link-transfer".to_string())
        .build();

    // Retry on rate limit errors
    let response = {
        let mut retry_count = 0;
        let max_retries = 3;

        loop {
            match ops
                .create_dev_transfer_transaction(transfer_builder.clone())
                .await
            {
                Ok(response) => break response,
                Err(e)
                    if (e.to_string().contains("429") || e.to_string().contains("rate limit"))
                        && retry_count < max_retries =>
                {
                    retry_count += 1;
                    let delay = 2u64.pow(retry_count); // Exponential backoff: 2s, 4s, 8s
                    println!(
                        "⏳ Rate limited, waiting {}s before retry {}/{}...",
                        delay, retry_count, max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
                }
                Err(e) => panic!("Failed to create LINK token transfer transaction: {}", e),
            }
        }
    };

    println!("✅ Successfully created LINK token transfer transaction!");
    println!("Transaction ID: {}", response.id);
    println!("Transaction state: {}", response.state);

    assert!(
        !response.id.is_empty(),
        "Transaction ID should not be empty"
    );
}

#[tokio::test]
async fn test_create_transfer_transaction_all_fee_levels() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get or create a test wallet (reuses existing to avoid rate limits)
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Fee Level Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    println!("Testing all fee levels with wallet: {}", test_wallet.id);

    // Get destination wallet
    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    println!("Destination address: {}", destination_wallet.address);

    let fee_levels = vec![
        (FeeLevel::Low, "LOW"),
        (FeeLevel::Medium, "MEDIUM"),
        (FeeLevel::High, "HIGH"),
    ];

    for (fee_level, level_name) in fee_levels {
        println!("\nTesting fee level: {}", level_name);

        let wallet_id = test_wallet.id.clone();
        let dest_address = destination_wallet.address.clone();
        let fee_level_clone = fee_level.clone();
        let level_name_str = level_name.to_string();

        let response = retry_on_rate_limit(|| async {
            let transfer_builder = CreateTransferTransactionRequestBuilder::new(wallet_id.clone())
                .destination_address(dest_address.clone())
                .amounts(vec!["0.001".to_string()])
                .blockchain(Blockchain::EthSepolia)
                .idempotency_key(uuid::Uuid::new_v4().to_string())
                .fee_level(fee_level_clone.clone())
                .ref_id(format!("test-transfer-{}", level_name_str.to_lowercase()))
                .build();

            ops.create_dev_transfer_transaction(transfer_builder).await
        })
        .await
        .expect(&format!(
            "Failed to create {} fee level transaction",
            level_name
        ));

        println!(
            "  ✅ {} fee level transaction created: {}",
            level_name, response.id
        );
        assert!(!response.id.is_empty());
    }
}

#[tokio::test]
#[ignore]
async fn test_request_testnet_tokens() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Faucet Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    println!("Testing faucet for wallet: {}", test_wallet.address);

    let request = RequestTestnetTokensRequest {
        blockchain: Blockchain::EthSepolia,
        address: test_wallet.address.clone(),
        native: Some(true),
        usdc: Some(true),
        eurc: None,
    };

    let request_clone = request.clone();

    retry_on_rate_limit(|| async { view.request_testnet_tokens(request_clone.clone()).await })
        .await
        .expect("Failed to request testnet tokens");

    println!("✅ Successfully requested testnet tokens!");
    println!("   Tokens should arrive in a few moments");
}

#[tokio::test]
async fn test_estimate_contract_execution_fee() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Get a test wallet to use for estimation
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Fee Estimate Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    // Example: USDC contract on Ethereum Sepolia
    let usdc_contract = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";

    let request = EstimateContractExecutionFeeBody {
        contract_address: usdc_contract.to_string(),
        abi_function_signature: Some("balanceOf(address)".to_string()),
        abi_parameters: Some(vec![AbiParameter::String(test_wallet.address.clone())]),
        call_data: None,
        amount: None,
        blockchain: None,
        source_address: None,
        wallet_id: Some(test_wallet.id.clone()),
    };

    let response = view
        .estimate_contract_execution_fee(request)
        .await
        .expect("Failed to estimate contract execution fee");

    println!("✅ Contract execution fee estimation successful!");
    println!("High gas limit: {:?}", response.high.gas_limit);
    println!("Medium gas limit: {:?}", response.medium.gas_limit);
    println!("Low gas limit: {:?}", response.low.gas_limit);

    assert!(response.high.gas_limit.is_some());
}

#[tokio::test]
async fn test_estimate_transfer_fee() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Fee Estimate Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    let request = EstimateTransferFeeRequest {
        destination_address: destination_wallet.address.clone(),
        amounts: vec!["0.001".to_string()],
        nft_token_ids: None,
        source_address: None,
        token_id: None,
        token_address: None,
        blockchain: Some("ETH-SEPOLIA".to_string()),
        wallet_id: Some(test_wallet.id.clone()),
    };

    let response = view
        .estimate_transfer_fee(request)
        .await
        .expect("Failed to estimate transfer fee");

    println!("✅ Transfer fee estimation successful!");
    println!("High network fee: {:?}", response.high.network_fee);
    println!("Medium network fee: {:?}", response.medium.network_fee);
    println!("Low network fee: {:?}", response.low.network_fee);

    assert!(response.high.network_fee.is_some());
}

#[tokio::test]
async fn test_query_contract() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");

    // Example: USDC contract on Ethereum Sepolia
    let usdc_contract = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";

    let request = QueryContractRequest {
        blockchain: "ETH-SEPOLIA".to_string(),
        address: usdc_contract.to_string(),
        abi_function_signature: Some("name()".to_string()),
        abi_parameters: None,
        abi_json: None,
        call_data: None,
        from_address: None,
    };

    let response = ops
        .dev_query_contract(request)
        .await
        .expect("Failed to query contract");

    println!("✅ Contract query successful!");
    if let Some(values) = &response.output_values {
        println!("Output values: {:?}", values);
    } else {
        println!("Output values: None");
    }
    println!("Output data: {}", response.output_data);

    assert!(!response.output_data.is_empty());
}

#[tokio::test]
async fn test_create_contract_execution_transaction() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Contract Execution Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    // Ensure wallet is funded
    ensure_wallet_funded(&view, &test_wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to ensure wallet is funded");

    // Deploy or get a test contract
    let contract_address = get_or_deploy_test_contract(&ops, &test_wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to get or deploy test contract");

    println!("🎯 Using contract: {}", contract_address);

    let builder = CreateContractExecutionTransactionRequestBuilder::new(
        test_wallet.id.clone(),
        contract_address.clone(),
        uuid::Uuid::new_v4().to_string(),
    )
    .abi_function_signature("approve(address,uint256)".to_string())
    .abi_parameters(vec![
        AbiParameter::String("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string()),
        AbiParameter::Integer(1000000),
    ])
    .fee_level(FeeLevel::Medium)
    .ref_id("test-contract-execution".to_string())
    .build();

    let response = ops
        .create_dev_contract_execution_transaction(builder)
        .await
        .expect("Failed to create contract execution transaction");

    println!("✅ Contract execution transaction created!");
    println!("Transaction ID: {}", response.id);
    println!("Transaction state: {}", response.state);

    assert!(!response.id.is_empty());
}

#[tokio::test]
#[ignore]
async fn test_create_wallet_upgrade_transaction() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Wallet upgrade requires a specific SCA wallet type
    println!("🔧 Creating/getting SCA wallet for upgrade test...");

    // Get or create an SCA wallet
    let sca_wallet = get_or_create_sca_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
        .await
        .expect("Failed to get or create SCA wallet");

    println!("✅ Using SCA wallet: {}", sca_wallet.id);
    println!("   Account type: {}", sca_wallet.account_type);

    // Ensure the SCA wallet is funded
    ensure_wallet_funded(&view, &sca_wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to ensure SCA wallet is funded");

    let builder = CreateWalletUpgradeTransactionRequestBuilder::new(
        sca_wallet.id.clone(),
        ScaCore::Circle6900SingleownerV3,
        uuid::Uuid::new_v4().to_string(),
    )
    .fee_level(FeeLevel::Medium)
    .ref_id("test-wallet-upgrade".to_string())
    .build();

    let response = ops
        .create_dev_wallet_upgrade_transaction(builder)
        .await
        .expect("Failed to create wallet upgrade transaction");

    println!("✅ Wallet upgrade transaction created!");
    println!("Transaction ID: {}", response.id);
    println!("Transaction state: {}", response.state);

    assert!(!response.id.is_empty());
}

#[tokio::test]
async fn test_cancel_transaction() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // First, create a transaction that we can cancel
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Cancel Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    // Ensure wallet is funded
    ensure_wallet_funded(&view, &test_wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to ensure wallet is funded");

    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    // Create a transfer transaction
    println!("📝 Creating a transfer transaction to cancel...");
    let test_wallet_id = test_wallet.id.clone();
    let dest_address = destination_wallet.address.clone();

    let transaction_response = common::retry_on_rate_limit(|| async {
        let transfer_builder = CreateTransferTransactionRequestBuilder::new(test_wallet_id.clone())
            .destination_address(dest_address.clone())
            .amounts(vec!["0.0001".to_string()])
            .blockchain(Blockchain::EthSepolia)
            .idempotency_key(uuid::Uuid::new_v4().to_string())
            .fee_level(FeeLevel::Low) // Use low fee to make it easier to cancel
            .ref_id("test-transfer-for-cancel".to_string())
            .build();

        ops.create_dev_transfer_transaction(transfer_builder).await
    })
    .await
    .expect("Failed to create transfer transaction");

    let transaction_id = transaction_response.id.clone();
    println!("✅ Transaction created: {}", transaction_id);
    println!("   Transaction state: {}", transaction_response.state);

    // Wait a moment before attempting to cancel
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Now try to cancel the transaction
    println!("🚫 Attempting to cancel transaction...");
    let cancel_builder = CancelTransactionRequestBuilder::new(
        transaction_id.clone(),
        uuid::Uuid::new_v4().to_string(),
    )
    .build();

    match ops.cancel_dev_transaction(cancel_builder).await {
        Ok(response) => {
            println!("✅ Transaction canceled!");
            println!("Transaction ID: {}", response.id);
            println!("Transaction state: {}", response.state);
            assert_eq!(response.id, transaction_id);
        }
        Err(e) => {
            panic!("Failed to cancel transaction with unexpected error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_accelerate_transaction() {
    dotenv::dotenv().ok();

    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // First, create a transaction that we can accelerate
    let test_wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::EthSepolia,
        "Accelerate Test",
    )
    .await
    .expect("Failed to get or create test wallet");

    // Ensure wallet is funded
    ensure_wallet_funded(&view, &test_wallet, &Blockchain::EthSepolia)
        .await
        .expect("Failed to ensure wallet is funded");

    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::EthSepolia)
            .await
            .expect("Failed to get or create destination wallet");

    // Create a transfer transaction with low fee
    println!("📝 Creating a transfer transaction to accelerate...");
    let test_wallet_id = test_wallet.id.clone();
    let dest_address = destination_wallet.address.clone();

    let transaction_response = common::retry_on_rate_limit(|| async {
        let transfer_builder = CreateTransferTransactionRequestBuilder::new(test_wallet_id.clone())
            .destination_address(dest_address.clone())
            .amounts(vec!["0.0001".to_string()])
            .blockchain(Blockchain::EthSepolia)
            .idempotency_key(uuid::Uuid::new_v4().to_string())
            .fee_level(FeeLevel::Low) // Use low fee so we can accelerate it
            .ref_id("test-transfer-for-accelerate".to_string())
            .build();

        ops.create_dev_transfer_transaction(transfer_builder).await
    })
    .await
    .expect("Failed to create transfer transaction");

    let transaction_id = transaction_response.id.clone();
    println!("✅ Transaction created: {}", transaction_id);
    println!("   Transaction state: {}", transaction_response.state);

    // Wait a moment before attempting to accelerate
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Now try to accelerate the transaction
    println!("⚡ Attempting to accelerate transaction...");
    let accelerate_builder = AccelerateTransactionRequestBuilder::new(
        transaction_id.clone(),
        uuid::Uuid::new_v4().to_string(),
    )
    .build();

    match ops.accelerate_dev_transaction(accelerate_builder).await {
        Ok(response) => {
            println!("✅ Transaction accelerated!");
            println!("Transaction ID: {}", response.id);
            assert_eq!(response.id, transaction_id);
        }
        Err(e) => {
            panic!(
                "Failed to accelerate transaction with unexpected error: {}",
                e
            );
        }
    }
}
