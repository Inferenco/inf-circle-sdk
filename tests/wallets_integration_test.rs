use base64::{engine::general_purpose, Engine as _};
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    helper::PaginationParams,
    wallet::{
        dto::{AccountType, Blockchain, QueryParams, UpdateWalletRequest, WalletMetadata},
        wallet_ops::{
            CreateWalletRequestBuilder, SignDataRequestBuilder, SignDelegateRequestBuilder,
            SignMessageRequestBuilder, SignTransactionRequestBuilder,
        },
        wallet_view::{
            ListWalletsParamsBuilder, ListWalletsWithBalancesParamsBuilder, QueryParamsBuilder,
        },
    },
};
use std::env;

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

    // Create a simple delegate action JSON
    let delegate_action_json = serde_json::json!({
        "senderId": "test.sender.near",
        "receiverId": "guest-book.testnet",
        "actions": [
            {
                "type": "FunctionCall",
                "methodName": "addMessage",
                "args": "eyJ0ZXh0IjoiSGVsbG8gZnJvbSBDaXJjbGUgU0RLISJ9",
                "gas": "100000000000000",
                "deposit": "0"
            }
        ],
        "nonce": "1",
        "maxBlockHeight": "1000000",
        "publicKey": test_wallet.initial_public_key.as_ref().unwrap()
    })
    .to_string();

    let unsigned_delegate_action = general_purpose::STANDARD.encode(&delegate_action_json);

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
