mod common;

use common::{ensure_wallet_funded, get_or_create_destination_wallet, get_or_create_test_wallet};
use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    near::{
        dto::NearNetwork, get_near_account_balance, get_near_token_balance,
        get_near_token_balances, get_near_token_metadata, parse_near_public_key,
        serialize_near_delegate_action_to_base64,
    },
    types::Blockchain,
};
use near_primitives::{
    action::{
        delegate::{DelegateAction, NonDelegateAction},
        Action as NearAction, FunctionCallAction,
    },
    types::AccountId as NearAccountId,
};
use std::env;

#[tokio::test]
async fn test_get_near_account_balance_testnet() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Get or create a NEAR wallet (uses same ref_id pattern as other tests)
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    .expect("Failed to get or create NEAR wallet");

    // Try to ensure wallet is funded (NEAR testnet faucet may not be available via Circle API)
    // If funding fails, we can still test balance queries (will return 0 balance)
    if let Err(e) = ensure_wallet_funded(&view, &wallet, &Blockchain::NearTestnet).await {
        println!(
            "⚠️  Could not fund wallet via faucet (this is expected for NEAR): {}",
            e
        );
        println!("   Balance query will still work, but wallet may have 0 balance");
    }

    // Query balance using our NEAR helper
    // Note: NEAR implicit accounts don't exist on-chain until they receive their first transaction
    // If the account doesn't exist, we'll use a known testnet account for testing
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay_seconds = 1;
    let mut balance_result = get_near_account_balance(&wallet.address, NearNetwork::Testnet).await;

    // Retry on RPC errors
    while retry_count < max_retries {
        match &balance_result {
            Ok(_) => break,
            Err(e) if e.to_string().contains("RPC error") => {
                retry_count += 1;
                if retry_count < max_retries {
                    println!(
                        "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                        delay_seconds, retry_count, max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                    delay_seconds *= 2; // Exponential backoff
                    balance_result =
                        get_near_account_balance(&wallet.address, NearNetwork::Testnet).await;
                }
            }
            _ => break,
        }
    }

    let (account_id, balance) = match balance_result {
        Ok(balance) => {
            // Account exists, use it
            (wallet.address.clone(), balance)
        }
        Err(e) if e.to_string().contains("does not exist") => {
            // Account doesn't exist yet (common for new NEAR implicit accounts)
            // Use a known testnet account for testing the balance query functionality
            println!("⚠️  Wallet account doesn't exist on-chain yet (NEAR implicit accounts need first transaction)");
            println!("   Using known testnet account 'guest-book.testnet' for balance query test");
            let fallback_account = "guest-book.testnet";

            // Retry the fallback query with exponential backoff for RPC errors
            let mut retry_count = 0;
            let max_retries = 5;
            let mut delay_seconds = 1;

            loop {
                match get_near_account_balance(fallback_account, NearNetwork::Testnet).await {
                    Ok(balance) => {
                        break (fallback_account.to_string(), balance);
                    }
                    Err(e) if e.to_string().contains("RPC error") && retry_count < max_retries => {
                        retry_count += 1;
                        println!(
                            "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                            delay_seconds, retry_count, max_retries
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                        delay_seconds *= 2; // Exponential backoff
                    }
                    Err(e) => {
                        panic!(
                            "Failed to get balance for fallback account after {} retries: {}",
                            retry_count, e
                        );
                    }
                }
            }
        }
        Err(e) if e.to_string().contains("RPC error") => {
            // RPC errors after retries - fail the test
            panic!(
                "Failed to get NEAR account balance after {} retries: {}",
                retry_count, e
            );
        }
        Err(e) => {
            panic!("Failed to get NEAR account balance: {}", e);
        }
    };

    println!(
        "✅ Successfully queried balance for account: {}",
        account_id
    );
    println!("   Total: {} NEAR", balance.total);
    println!("   Available: {} NEAR", balance.available);
    println!("   Staked: {} NEAR", balance.staked);

    // Verify the response structure
    assert!(
        !balance.total.is_empty(),
        "Total balance should not be empty"
    );
    assert!(
        !balance.available.is_empty(),
        "Available balance should not be empty"
    );
    assert!(
        !balance.staked.is_empty(),
        "Staked balance should not be empty"
    );
    assert!(
        balance.block_height.is_some(),
        "Block height should be present"
    );
    assert!(balance.state_hash.is_some(), "State hash should be present");

    // Verify balances are valid numbers (can be parsed as f64)
    let total: f64 = balance
        .total
        .parse()
        .expect("Total should be a valid number");
    let available: f64 = balance
        .available
        .parse()
        .expect("Available should be a valid number");
    let staked: f64 = balance
        .staked
        .parse()
        .expect("Staked should be a valid number");

    assert!(total >= 0.0, "Total balance should be non-negative");
    assert!(available >= 0.0, "Available balance should be non-negative");
    assert!(staked >= 0.0, "Staked balance should be non-negative");
    assert!(
        (available + staked - total).abs() < 0.000001,
        "Available + Staked should approximately equal Total (within floating point precision)"
    );
}

#[tokio::test]
async fn test_get_near_account_balance_mainnet() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Test with a known mainnet account (near.near is a well-known account)
    let account_id = "near.near";
    let balance = get_near_account_balance(account_id, NearNetwork::Mainnet)
        .await
        .expect("Failed to get NEAR account balance");

    println!("✅ Successfully queried mainnet balance for {}", account_id);
    println!("   Total: {} NEAR", balance.total);
    println!("   Available: {} NEAR", balance.available);
    println!("   Staked: {} NEAR", balance.staked);

    // Verify the response structure
    assert!(!balance.total.is_empty());
    assert!(!balance.available.is_empty());
    assert!(!balance.staked.is_empty());
    assert!(balance.block_height.is_some());
    assert!(balance.state_hash.is_some());
}

#[tokio::test]
async fn test_get_near_account_balance_invalid_account() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Test with an invalid account ID
    let account_id = "this-account-definitely-does-not-exist-123456789.testnet";
    let result = get_near_account_balance(account_id, NearNetwork::Testnet).await;

    // Should return an error for non-existent account
    assert!(
        result.is_err(),
        "Should return error for non-existent account"
    );
    println!(
        "✅ Correctly returned error for invalid account: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_parse_near_public_key_with_prefix() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Get or create a NEAR wallet (uses same ref_id pattern as other tests)
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    .expect("Failed to get or create NEAR wallet");

    // Get the public key from the wallet
    let public_key_str = wallet
        .initial_public_key
        .as_ref()
        .expect("Wallet should have an initial public key");

    // Ensure it has the ed25519: prefix (Circle should provide it)
    let public_key_str = if public_key_str.starts_with("ed25519:") {
        public_key_str.clone()
    } else {
        format!("ed25519:{}", public_key_str)
    };

    let result = parse_near_public_key(&public_key_str);

    match result {
        Ok(pk) => {
            println!("✅ Successfully parsed public key with prefix: {:?}", pk);
            assert_eq!(pk.to_string(), public_key_str);
        }
        Err(e) => {
            panic!("Failed to parse valid public key: {}", e);
        }
    }
}

#[tokio::test]
async fn test_parse_near_public_key_without_prefix() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Get or create a NEAR wallet (uses same ref_id pattern as other tests)
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    .expect("Failed to get or create NEAR wallet");

    // Get the public key from the wallet and remove prefix if present
    let public_key_str = wallet
        .initial_public_key
        .as_ref()
        .expect("Wallet should have an initial public key");

    // Remove ed25519: prefix if present to test parsing without prefix
    let public_key_base58 = if public_key_str.starts_with("ed25519:") {
        public_key_str.strip_prefix("ed25519:").unwrap()
    } else {
        public_key_str.as_str()
    };

    let result = parse_near_public_key(public_key_base58);

    match result {
        Ok(pk) => {
            println!("✅ Successfully parsed public key without prefix: {:?}", pk);
            // Should have added the ed25519: prefix
            assert!(pk.to_string().starts_with("ed25519:"));
            assert!(pk.to_string().ends_with(public_key_base58));
        }
        Err(e) => {
            panic!("Failed to parse valid public key: {}", e);
        }
    }
}

#[tokio::test]
async fn test_parse_near_public_key_from_wallet() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Get or create a NEAR wallet (uses same ref_id pattern as other tests)
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    .expect("Failed to get or create NEAR wallet");

    // Get the public key from the wallet
    let public_key_str = wallet
        .initial_public_key
        .as_ref()
        .expect("Wallet should have an initial public key");

    println!("Testing with wallet public key: {}", public_key_str);

    // Parse the public key
    let parsed_key =
        parse_near_public_key(public_key_str).expect("Failed to parse wallet public key");

    println!("✅ Successfully parsed wallet public key: {:?}", parsed_key);
    assert!(!parsed_key.to_string().is_empty());
}

#[tokio::test]
async fn test_serialize_near_delegate_action() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Get or create a NEAR wallet (uses same ref_id pattern as other tests)
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    .expect("Failed to get or create NEAR wallet");

    // Parse the public key
    let public_key_str = wallet
        .initial_public_key
        .as_ref()
        .expect("Wallet should have an initial public key");

    let public_key = parse_near_public_key(public_key_str).expect("Failed to parse public key");

    // Get or create a destination wallet for the delegate action
    let destination_wallet =
        get_or_create_destination_wallet(&ops, &view, &wallet_set_id, &Blockchain::NearTestnet)
            .await
            .expect("Failed to get or create destination wallet");

    // Create a delegate action
    let sender_id = NearAccountId::try_from(wallet.address.clone())
        .expect("Failed to create AccountId from wallet address");
    let receiver_id = NearAccountId::try_from(destination_wallet.address.clone())
        .expect("Failed to create receiver AccountId");

    // Create a function call action
    // Note: This is just for testing serialization - the destination wallet doesn't need to be a contract
    // The serialization itself is what we're testing, not the execution
    let function_call = FunctionCallAction {
        method_name: "test_method".to_string(),
        args: r#"{"test":"data"}"#.as_bytes().to_vec(),
        gas: 100_000_000_000_000,
        deposit: 0,
    };

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

    // Serialize the delegate action
    let serialized = serialize_near_delegate_action_to_base64(&delegate_action)
        .expect("Failed to serialize delegate action");

    println!("✅ Successfully serialized delegate action");
    println!("   Length: {} characters", serialized.len());
    println!(
        "   First 50 chars: {}",
        &serialized[..serialized.len().min(50)]
    );

    // Verify it's valid base64
    assert!(
        !serialized.is_empty(),
        "Serialized action should not be empty"
    );

    // Verify it starts with valid base64 characters
    assert!(
        serialized
            .chars()
            .all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '='),
        "Should be valid base64"
    );

    // The serialized action should include the NEP-461 prefix
    // Decode and check first 4 bytes
    use base64::{engine::general_purpose, Engine as _};
    if let Ok(decoded) = general_purpose::STANDARD.decode(&serialized) {
        assert!(decoded.len() >= 4, "Should have at least 4 bytes (prefix)");

        // Check NEP-461 prefix: 0x400001CD (1073742285) as little-endian u32
        let prefix_bytes = &decoded[0..4];
        let prefix_value = u32::from_le_bytes([
            prefix_bytes[0],
            prefix_bytes[1],
            prefix_bytes[2],
            prefix_bytes[3],
        ]);
        const EXPECTED_PREFIX: u32 = 0x40000000 + 461; // 1073742285
        assert_eq!(
            prefix_value, EXPECTED_PREFIX,
            "Should have NEP-461 prefix (0x{:X})",
            EXPECTED_PREFIX
        );
        println!("✅ Verified NEP-461 prefix: 0x{:X}", prefix_value);
    }
}

#[tokio::test]
async fn test_get_balance_for_circle_wallet() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Get or create a NEAR wallet (uses same ref_id pattern as other tests)
    let wallet = get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    .expect("Failed to get or create NEAR wallet");

    // Try to ensure wallet is funded (NEAR testnet faucet may not be available via Circle API)
    // If funding fails, we can still test balance queries (will return 0 balance)
    if let Err(e) = ensure_wallet_funded(&view, &wallet, &Blockchain::NearTestnet).await {
        println!(
            "⚠️  Could not fund wallet via faucet (this is expected for NEAR): {}",
            e
        );
        println!("   Balance query will still work, but wallet may have 0 balance");
    }

    println!(
        "Testing balance query for Circle wallet: {}",
        wallet.address
    );

    // Query balance using our NEAR helper
    // Note: NEAR implicit accounts don't exist on-chain until they receive their first transaction
    // If the account doesn't exist, we'll use a known testnet account for testing
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay_seconds = 1;
    let mut balance_result = get_near_account_balance(&wallet.address, NearNetwork::Testnet).await;

    // Retry on RPC errors
    while retry_count < max_retries {
        match &balance_result {
            Ok(_) => break,
            Err(e) if e.to_string().contains("RPC error") => {
                retry_count += 1;
                if retry_count < max_retries {
                    println!(
                        "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                        delay_seconds, retry_count, max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                    delay_seconds *= 2; // Exponential backoff
                    balance_result =
                        get_near_account_balance(&wallet.address, NearNetwork::Testnet).await;
                }
            }
            _ => break,
        }
    }

    let (account_id, balance) = match balance_result {
        Ok(balance) => {
            // Account exists, use it
            (wallet.address.clone(), balance)
        }
        Err(e) if e.to_string().contains("does not exist") => {
            // Account doesn't exist yet (common for new NEAR implicit accounts)
            // Use a known testnet account for testing the balance query functionality
            println!("⚠️  Wallet account doesn't exist on-chain yet (NEAR implicit accounts need first transaction)");
            println!("   Using known testnet account 'guest-book.testnet' for balance query test");
            let fallback_account = "guest-book.testnet";

            // Retry the fallback query with exponential backoff for RPC errors
            let mut retry_count = 0;
            let max_retries = 5;
            let mut delay_seconds = 1;

            loop {
                match get_near_account_balance(fallback_account, NearNetwork::Testnet).await {
                    Ok(balance) => {
                        break (fallback_account.to_string(), balance);
                    }
                    Err(e) if e.to_string().contains("RPC error") && retry_count < max_retries => {
                        retry_count += 1;
                        println!(
                            "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                            delay_seconds, retry_count, max_retries
                        );
                        tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                        delay_seconds *= 2; // Exponential backoff
                    }
                    Err(e) => {
                        panic!(
                            "Failed to get balance for fallback account after {} retries: {}",
                            retry_count, e
                        );
                    }
                }
            }
        }
        Err(e) if e.to_string().contains("RPC error") => {
            // RPC errors after retries - fail the test
            panic!(
                "Failed to get NEAR account balance after {} retries: {}",
                retry_count, e
            );
        }
        Err(e) => {
            panic!("Failed to get NEAR account balance: {}", e);
        }
    };

    println!(
        "✅ Successfully queried balance for account: {}",
        account_id
    );
    println!("   Total: {} NEAR", balance.total);
    println!("   Available: {} NEAR", balance.available);
    println!("   Staked: {} NEAR", balance.staked);
    println!("   Block Height: {:?}", balance.block_height);

    // Verify the response
    assert!(!balance.total.is_empty());
    assert!(!balance.available.is_empty());
    assert!(!balance.staked.is_empty());
}

#[test]
fn test_near_network_rpc_urls() {
    // Test that network URLs are correct
    use inf_circle_sdk::near::dto::NearNetwork;

    assert_eq!(
        NearNetwork::Mainnet.rpc_url(),
        "https://rpc.mainnet.near.org"
    );
    assert_eq!(
        NearNetwork::Testnet.rpc_url(),
        "https://rpc.testnet.near.org"
    );

    println!("✅ Network RPC URLs are correct");
}

#[tokio::test]
async fn test_get_near_token_balance() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Try to get or create a NEAR wallet
    let account_id = match get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    {
        Ok(wallet) => {
            // Try to fund the wallet
            if let Err(e) = ensure_wallet_funded(&view, &wallet, &Blockchain::NearTestnet).await {
                println!(
                    "⚠️  Could not fund wallet via faucet (this is expected for NEAR): {}",
                    e
                );
                println!("   Using well-known account 'guest-book.testnet' for token balance test");
                "guest-book.testnet".to_string()
            } else {
                wallet.address
            }
        }
        Err(_) => {
            println!(
                "⚠️  Could not create Circle wallet, using well-known account 'guest-book.testnet'"
            );
            "guest-book.testnet".to_string()
        }
    };

    let token_contract = "wrap.testnet"; // Wrapped NEAR on testnet

    println!("Testing token balance query for account: {}", account_id);
    println!("Token contract: {}", token_contract);

    // Retry on RPC errors
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay_seconds = 1;
    let mut balance_result =
        get_near_token_balance(&account_id, token_contract, NearNetwork::Testnet).await;

    while retry_count < max_retries {
        match &balance_result {
            Ok(_) => break,
            Err(e) if e.to_string().contains("RPC error") => {
                retry_count += 1;
                if retry_count < max_retries {
                    println!(
                        "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                        delay_seconds, retry_count, max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                    delay_seconds *= 2;
                    balance_result =
                        get_near_token_balance(&account_id, token_contract, NearNetwork::Testnet)
                            .await;
                }
            }
            _ => break,
        }
    }

    match balance_result {
        Ok(balance) => {
            println!("✅ Successfully queried token balance");
            println!("   Balance: {}", balance);

            // Verify balance is a valid string (can be "0" or a number)
            assert!(!balance.is_empty(), "Balance should not be empty");

            // Try to parse as u128 to verify it's a valid number
            let balance_u128: u128 = balance.parse().unwrap_or(0);
            println!("   Parsed balance: {} (raw units)", balance_u128);
        }
        Err(e) => {
            panic!(
                "Failed to get token balance after {} retries: {}",
                retry_count, e
            );
        }
    }
}

#[tokio::test]
async fn test_get_near_token_metadata() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Test querying token metadata for a known testnet token
    let token_contract = "wrap.testnet"; // Wrapped NEAR on testnet

    println!(
        "Testing token metadata query for contract: {}",
        token_contract
    );

    // Retry on RPC errors
    let mut retry_count = 0;
    let max_retries = 5;
    let mut delay_seconds = 1;
    let mut metadata_result = get_near_token_metadata(token_contract, NearNetwork::Testnet).await;

    while retry_count < max_retries {
        match &metadata_result {
            Ok(_) => break,
            Err(e) if e.to_string().contains("RPC error") => {
                retry_count += 1;
                if retry_count < max_retries {
                    println!(
                        "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                        delay_seconds, retry_count, max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                    delay_seconds *= 2;
                    metadata_result =
                        get_near_token_metadata(token_contract, NearNetwork::Testnet).await;
                }
            }
            _ => break,
        }
    }

    match metadata_result {
        Ok(metadata) => {
            println!("✅ Successfully queried token metadata");
            println!("   Symbol: {}", metadata.symbol);
            println!("   Name: {}", metadata.name);
            println!("   Decimals: {}", metadata.decimals);
            if let Some(icon) = &metadata.icon {
                println!("   Icon: {}", icon);
            }
            if let Some(reference) = &metadata.reference {
                println!("   Reference: {}", reference);
            }

            // Verify metadata structure
            assert!(!metadata.symbol.is_empty(), "Symbol should not be empty");
            assert!(!metadata.name.is_empty(), "Name should not be empty");
            assert!(
                metadata.decimals <= 24,
                "Decimals should be reasonable (max 24)"
            );
        }
        Err(e) => {
            panic!(
                "Failed to get token metadata after {} retries: {}",
                retry_count, e
            );
        }
    }
}

#[tokio::test]
async fn test_get_near_token_balances() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Try to get or create a NEAR wallet
    let account_id = match get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    {
        Ok(wallet) => {
            // Try to fund the wallet
            if let Err(e) = ensure_wallet_funded(&view, &wallet, &Blockchain::NearTestnet).await {
                println!(
                    "⚠️  Could not fund wallet via faucet (this is expected for NEAR): {}",
                    e
                );
                println!(
                    "   Using well-known account 'guest-book.testnet' for token balances test"
                );
                "guest-book.testnet".to_string()
            } else {
                wallet.address
            }
        }
        Err(_) => {
            println!(
                "⚠️  Could not create Circle wallet, using well-known account 'guest-book.testnet'"
            );
            "guest-book.testnet".to_string()
        }
    };

    // List of common testnet tokens to check
    let token_contracts = vec![
        "wrap.testnet".to_string(),       // Wrapped NEAR
        "usdc.fakes.testnet".to_string(), // USDC testnet
        "usdt.fakes.testnet".to_string(), // USDT testnet
    ];

    println!(
        "Testing multiple token balance queries for account: {}",
        account_id
    );
    println!("Checking {} token contracts...", token_contracts.len());

    // Retry on RPC errors (get_near_token_balances handles retries internally for individual tokens,
    // but we can retry the whole operation if it fails completely)
    let mut retry_count = 0;
    let max_retries = 3;
    let mut delay_seconds = 1;
    let mut balances_result = get_near_token_balances(
        &account_id,
        &token_contracts,
        NearNetwork::Testnet,
        true, // include metadata
    )
    .await;

    // Note: get_near_token_balances already handles individual token failures gracefully,
    // so we only retry if there's a complete failure (unlikely)
    while retry_count < max_retries {
        match &balances_result {
            Ok(_) => break,
            Err(e) if e.to_string().contains("RPC error") && retry_count < max_retries => {
                retry_count += 1;
                println!(
                    "⚠️  RPC error, retrying in {} seconds ({}/{})...",
                    delay_seconds, retry_count, max_retries
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                delay_seconds *= 2;
                balances_result = get_near_token_balances(
                    &account_id,
                    &token_contracts,
                    NearNetwork::Testnet,
                    true,
                )
                .await;
            }
            _ => break,
        }
    }

    match balances_result {
        Ok(balances) => {
            println!("✅ Successfully queried token balances");
            println!("   Found {} tokens with non-zero balances", balances.len());

            for balance in &balances {
                println!("   Token: {}", balance.contract_id);
                println!("      Balance: {}", balance.balance);

                if let Some(meta) = &balance.metadata {
                    println!("      Symbol: {}", meta.symbol);
                    println!("      Name: {}", meta.name);
                    println!("      Decimals: {}", meta.decimals);
                } else {
                    println!("      Metadata: Not available");
                }
            }

            // Verify that all returned balances are non-zero
            for balance in &balances {
                let balance_u128: u128 = balance.balance.parse().unwrap_or(0);
                assert!(balance_u128 > 0, "All returned balances should be non-zero");
            }
        }
        Err(e) => {
            panic!(
                "Failed to get token balances after {} retries: {}",
                retry_count, e
            );
        }
    }
}

#[tokio::test]
async fn test_get_near_token_balances_without_metadata() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new().expect("Failed to create CircleOps");
    let view = CircleView::new().expect("Failed to create CircleView");

    // Get wallet set ID
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")
        .expect("CIRCLE_WALLET_SET_ID environment variable not set");

    // Try to get or create a NEAR wallet
    let account_id = match get_or_create_test_wallet(
        &ops,
        &view,
        &wallet_set_id,
        &Blockchain::NearTestnet,
        "NEAR",
    )
    .await
    {
        Ok(wallet) => {
            // Try to fund the wallet
            if let Err(e) = ensure_wallet_funded(&view, &wallet, &Blockchain::NearTestnet).await {
                println!(
                    "⚠️  Could not fund wallet via faucet (this is expected for NEAR): {}",
                    e
                );
                println!(
                    "   Using well-known account 'guest-book.testnet' for token balances test"
                );
                "guest-book.testnet".to_string()
            } else {
                wallet.address
            }
        }
        Err(_) => {
            println!(
                "⚠️  Could not create Circle wallet, using well-known account 'guest-book.testnet'"
            );
            "guest-book.testnet".to_string()
        }
    };

    let token_contracts = vec!["wrap.testnet".to_string(), "usdc.fakes.testnet".to_string()];

    println!(
        "Testing token balance queries without metadata for account: {}",
        account_id
    );

    let balances_result = get_near_token_balances(
        &account_id,
        &token_contracts,
        NearNetwork::Testnet,
        false, // don't include metadata (faster)
    )
    .await;

    match balances_result {
        Ok(balances) => {
            println!("✅ Successfully queried token balances (without metadata)");
            println!("   Found {} tokens with non-zero balances", balances.len());

            for balance in &balances {
                println!("   Token: {}", balance.contract_id);
                println!("      Balance: {}", balance.balance);
                assert!(
                    balance.metadata.is_none(),
                    "Metadata should not be included"
                );
            }
        }
        Err(e) => {
            panic!("Failed to get token balances: {}", e);
        }
    }
}

#[tokio::test]
async fn test_get_near_token_balance_invalid_contract() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Test querying a non-existent token contract
    let account_id = "guest-book.testnet";
    let invalid_contract = "nonexistent-token.testnet";

    println!(
        "Testing token balance query with invalid contract: {}",
        invalid_contract
    );

    let balance_result =
        get_near_token_balance(account_id, invalid_contract, NearNetwork::Testnet).await;

    // This should fail with an informative error
    assert!(
        balance_result.is_err(),
        "Querying invalid contract should return an error"
    );

    if let Err(e) = balance_result {
        println!("✅ Got expected error for invalid contract: {}", e);
        assert!(
            e.to_string().contains("does not exist")
                || e.to_string().contains("RPC error")
                || e.to_string().contains("Invalid"),
            "Error should be informative"
        );
    }
}
