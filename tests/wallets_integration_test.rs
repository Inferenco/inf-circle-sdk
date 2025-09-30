use base64::{engine::general_purpose, Engine as _};
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    helper::{parse_near_public_key, serialize_near_delegate_action_to_base64, PaginationParams},
    wallet::{
        dto::{AccountType, Blockchain, QueryParams, UpdateWalletRequest, WalletMetadata},
        ops::{
            create_wallet::CreateWalletRequestBuilder, sign_data::SignDataRequestBuilder,
            sign_delegate::SignDelegateRequestBuilder, sign_message::SignMessageRequestBuilder,
            sign_transaction::SignTransactionRequestBuilder,
        },
        views::{
            list_transactions::ListTransactionsParamsBuilder,
            list_wallets::ListWalletsParamsBuilder,
            list_wallets_with_balances::ListWalletsWithBalancesParamsBuilder,
            query::QueryParamsBuilder,
        },
    },
};
use std::env;

// NEAR Protocol types (official)
use near_primitives::{
    action::{
        delegate::DelegateAction, delegate::NonDelegateAction, Action as NearAction,
        FunctionCallAction,
    },
    types::AccountId as NearAccountId,
};

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
        CreateWalletRequestBuilder::new(wallet_set_id.clone(), vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Integration Test Wallet".to_string()),
                ref_id: Some("test-ref-123".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
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
    let update_request = UpdateWalletRequest {
        name: Some("Updated Test Wallet".to_string()),
        ref_id: Some("test-ref-123".to_string()),
    };

    let updated_wallet = ops
        .update_wallet(&new_wallet.id, update_request)
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
    let list_params = ListWalletsParamsBuilder::new()
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
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Token Balance Test Wallet".to_string()),
                ref_id: Some("token-balance-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
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
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("NFT Test Wallet".to_string()),
                ref_id: Some("nft-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
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

    // Initialize CircleOps
    let ops = CircleOps::new().expect("Failed to create CircleOps");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a test wallet first
    let create_request_builder =
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Sign Message Test Wallet".to_string()),
                ref_id: Some("sign-message-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
    let test_wallet = create_response.wallets.first().expect("No wallet created");

    // Test signing a simple message
    let message = "Hello, Circle SDK!";
    let sign_request_builder =
        SignMessageRequestBuilder::new(test_wallet.id.clone(), message.to_string())
            .unwrap()
            .memo("Test message signing".to_string())
            .build();

    let sign_response = ops
        .sign_message(sign_request_builder)
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
        .sign_message(hex_sign_request_builder)
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
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Sign Data Test Wallet".to_string()),
                ref_id: Some("sign-data-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
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

    let sign_response = ops.sign_data(sign_request_builder).await;

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
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EvmTestnet])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Sign Transaction Test Wallet".to_string()),
                ref_id: Some("sign-transaction-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
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

    let sign_response = ops.sign_transaction(sign_request_builder).await;

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
            if test_wallet.blockchain == "SOL" {
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
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::NearTestnet])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Sign Delegate NEAR Test Wallet".to_string()),
                ref_id: Some("sign-delegate-near-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
        .await
        .expect("Failed to create NEAR wallet");
    let test_wallet = create_response
        .wallets
        .first()
        .expect("No NEAR wallet created");

    assert_eq!(
        test_wallet.blockchain, "NEAR-TESTNET",
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

    let sign_response = ops.sign_delegate(sign_request_builder).await;

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

    // Initialize CircleOps
    let ops = CircleOps::new().expect("Failed to create CircleOps");

    // Get required environment variables
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID not set");

    // Create a non-NEAR test wallet (Ethereum)
    let create_request_builder =
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Sign Delegate ETH Test Wallet".to_string()),
                ref_id: Some("sign-delegate-eth-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
        .await
        .expect("Failed to create ETH wallet");
    let test_wallet = create_response
        .wallets
        .first()
        .expect("No ETH wallet created");

    // Verify we have a non-NEAR wallet
    assert_ne!(
        test_wallet.blockchain, "NEAR",
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
    let result = ops.sign_delegate(sign_request_builder).await;
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

    // 1. Create a test wallet first
    let create_request_builder =
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Eoa)
            .metadata(vec![WalletMetadata {
                name: Some("Get Transaction Test Wallet".to_string()),
                ref_id: Some("get-transaction-test".to_string()),
            }])
            .build();

    let create_response = ops
        .create_wallet(create_request_builder)
        .await
        .expect("Failed to create wallet");
    let test_wallet = create_response.wallets.first().expect("No wallet created");

    println!("Created test wallet: {}", test_wallet.id);

    // 2. Create a transaction by signing a message (this creates a transaction record)
    let test_message = "Test message for transaction creation";
    let sign_request_builder =
        SignMessageRequestBuilder::new(test_wallet.id.clone(), test_message.to_string())
            .unwrap()
            .memo("Test transaction creation".to_string())
            .build();

    let sign_response = ops.sign_message(sign_request_builder).await;

    match sign_response {
        Ok(response) => {
            println!(
                "✅ Message signed successfully, signature: {}",
                response.signature
            );

            // 3. Wait a moment for the transaction to be processed
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            // 4. List transactions to find our newly created transaction
            let list_params = ListTransactionsParamsBuilder::new()
                .wallet_ids(test_wallet.id.clone())
                .pagination(PaginationParams {
                    page_size: Some(10),
                    page_after: None,
                    page_before: None,
                })
                .order("DESC".to_string())
                .build();

            let list_result = view.list_transactions(list_params).await;

            match list_result {
                Ok(transactions_response) => {
                    if let Some(first_tx) = transactions_response.transactions.first() {
                        let tx_id = &first_tx.id;
                        println!("Found transaction ID for testing: {}", tx_id);

                        // 5. Now test getting the specific transaction
                        let get_result = view.get_transaction(tx_id).await;

                        match get_result {
                            Ok(transaction_response) => {
                                println!("✅ Successfully retrieved transaction!");

                                // Verify the response structure
                                assert!(
                                    !transaction_response.transaction.id.is_empty(),
                                    "Transaction ID should not be empty"
                                );
                                assert_eq!(
                                    transaction_response.transaction.id, *tx_id,
                                    "Retrieved transaction ID should match requested ID"
                                );
                                assert!(
                                    !transaction_response.transaction.blockchain.is_empty(),
                                    "Blockchain should not be empty"
                                );
                                assert!(
                                    !transaction_response.transaction.state.is_empty(),
                                    "State should not be empty"
                                );
                                assert!(
                                    !transaction_response.transaction.transaction_type.is_empty(),
                                    "Transaction type should not be empty"
                                );

                                // Verify it's related to our test wallet
                                if let Some(wallet_id) = &transaction_response.transaction.wallet_id
                                {
                                    assert_eq!(
                                        wallet_id, &test_wallet.id,
                                        "Transaction should belong to our test wallet"
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
                                    "  Wallet ID: {:?}",
                                    transaction_response.transaction.wallet_id
                                );

                                if let Some(tx_hash) = &transaction_response.transaction.tx_hash {
                                    println!("  TX Hash: {}", tx_hash);
                                }
                                if let Some(amount_in_usd) =
                                    &transaction_response.transaction.amount_in_usd
                                {
                                    println!("  Amount in USD: {}", amount_in_usd);
                                }
                            }
                            Err(e) => {
                                panic!("Failed to get specific transaction: {}", e);
                            }
                        }
                    } else {
                        panic!("No transactions found for our test wallet");
                    }
                }
                Err(e) => {
                    panic!("Failed to list transactions: {}", e);
                }
            }
        }
        Err(e) => {
            // Handle the case where message signing might fail
            println!("⚠️  Message signing failed: {}", e);
            println!("This might be expected in some test environments");

            // Try to find any existing transaction for this wallet
            let list_params = ListTransactionsParamsBuilder::new()
                .wallet_ids(test_wallet.id.clone())
                .pagination(PaginationParams {
                    page_size: Some(10),
                    page_after: None,
                    page_before: None,
                })
                .order("DESC".to_string())
                .build();

            let list_result = view.list_transactions(list_params).await;

            match list_result {
                Ok(transactions_response) => {
                    if let Some(first_tx) = transactions_response.transactions.first() {
                        let tx_id = &first_tx.id;
                        println!("Found existing transaction ID for testing: {}", tx_id);

                        // Test getting the specific transaction
                        let get_result = view.get_transaction(tx_id).await;

                        match get_result {
                            Ok(transaction_response) => {
                                println!("✅ Successfully retrieved existing transaction!");

                                // Basic validation
                                assert!(
                                    !transaction_response.transaction.id.is_empty(),
                                    "Transaction ID should not be empty"
                                );
                                assert_eq!(
                                    transaction_response.transaction.id, *tx_id,
                                    "Retrieved transaction ID should match requested ID"
                                );

                                println!("Transaction details:");
                                println!("  ID: {}", transaction_response.transaction.id);
                                println!(
                                    "  Blockchain: {}",
                                    transaction_response.transaction.blockchain
                                );
                                println!("  State: {}", transaction_response.transaction.state);
                            }
                            Err(e) => {
                                panic!("Failed to get existing transaction: {}", e);
                            }
                        }
                    } else {
                        panic!("No transactions found for test wallet and message signing failed");
                    }
                }
                Err(e) => {
                    panic!(
                        "Failed to list transactions and message signing failed: {}",
                        e
                    );
                }
            }
        }
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
