use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    helper::PaginationParams,
    wallet::{
        dto::{AccountType, Blockchain, QueryParams, UpdateWalletRequest, WalletMetadata},
        wallet_ops::CreateWalletRequestBuilder,
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
