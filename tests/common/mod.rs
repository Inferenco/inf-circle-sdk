#![allow(dead_code)]

use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    dev_wallet::{
        dto::{AccountType, DevWallet, DevWalletMetadata, RequestTestnetTokensRequest},
        ops::create_wallet::CreateWalletRequestBuilder,
        views::{list_wallets::ListDevWalletsParamsBuilder, query::QueryParamsBuilder},
    },
    types::Blockchain,
    CircleError,
};

/// Helper function to retry operations that fail due to rate limiting
///
/// Retries with exponential backoff for 429 errors only
pub async fn retry_on_rate_limit<F, Fut, T>(
    mut operation: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, CircleError>>,
{
    let max_retries = 3;
    let mut delay_seconds = 2;

    for attempt in 0..max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if e.to_string().contains("429") || e.to_string().contains("rate limit") => {
                if attempt < max_retries - 1 {
                    println!(
                        "⏳ Rate limit hit, waiting {} seconds before retry {}/{}...",
                        delay_seconds,
                        attempt + 1,
                        max_retries
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(delay_seconds)).await;
                    delay_seconds *= 2; // Exponential backoff
                } else {
                    return Err(e.into());
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    unreachable!()
}

/// Helper function to get or create a wallet for testing, with rate limit handling
///
/// Uses a deterministic ref_id based on blockchain to ensure the same wallet is reused.
/// This allows you to manually fund the wallet once and use it for all tests.
pub async fn get_or_create_test_wallet(
    ops: &CircleOps,
    view: &CircleView,
    wallet_set_id: &str,
    blockchain: &Blockchain,
    name_prefix: &str,
) -> Result<DevWallet, Box<dyn std::error::Error>> {
    // Use a deterministic ref_id based on blockchain only (ignore name_prefix for max reuse)
    // Exception: destination wallets get their own ref_id
    let deterministic_ref_id = if name_prefix == "Destination" {
        format!(
            "test-wallet-destination-{}",
            blockchain.as_str().to_lowercase()
        )
    } else {
        format!("test-wallet-{}", blockchain.as_str().to_lowercase())
    };

    // Try to find an existing wallet by ref_id first
    let list_params = ListDevWalletsParamsBuilder::new()
        .wallet_set_id(wallet_set_id.to_string())
        .blockchain(blockchain.as_str().to_string())
        .ref_id(deterministic_ref_id.clone())
        .build();

    if let Ok(wallets_response) = view.list_wallets(list_params).await {
        if let Some(wallet) = wallets_response.wallets.into_iter().next() {
            println!(
                "♻️  Reusing existing test wallet: {} ({})",
                wallet.id, wallet.address
            );
            println!("   Ref ID: {}", deterministic_ref_id);
            return Ok(wallet);
        }
    }

    // If no wallet exists, create one with rate limit retry
    let mut retry_count = 0;
    let max_retries = 3;

    loop {
        let create_request_builder =
            CreateWalletRequestBuilder::new(wallet_set_id.to_string(), vec![blockchain.clone()])
                .unwrap()
                .account_type(AccountType::Eoa)
                .metadata(vec![DevWalletMetadata {
                    name: Some(format!("{} Wallet", name_prefix)),
                    ref_id: Some(deterministic_ref_id.clone()), // Use deterministic ref_id
                }])
                .build();

        match ops.create_wallet(create_request_builder).await {
            Ok(create_response) => {
                if let Some(wallet) = create_response.wallets.into_iter().next() {
                    println!(
                        "🆕 Created new test wallet: {} ({})",
                        wallet.id, wallet.address
                    );
                    println!("   Ref ID: {}", deterministic_ref_id);
                    println!("   📝 Fund this wallet manually at: https://sepoliafaucet.com/");
                    println!("   💡 This wallet will be reused for all future tests!");
                    return Ok(wallet);
                }
                return Err("No wallet created".into());
            }
            Err(e) if e.to_string().contains("429") && retry_count < max_retries => {
                retry_count += 1;
                let delay = 2u64.pow(retry_count); // Exponential backoff: 2s, 4s, 8s
                println!(
                    "Rate limited, waiting {}s before retry {}/{}...",
                    delay, retry_count, max_retries
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
            }
            Err(e) => return Err(e.into()),
        }
    }
}

/// Helper function to get the destination wallet for transfer tests
pub async fn get_or_create_destination_wallet(
    ops: &CircleOps,
    view: &CircleView,
    wallet_set_id: &str,
    blockchain: &Blockchain,
) -> Result<DevWallet, Box<dyn std::error::Error>> {
    get_or_create_test_wallet(ops, view, wallet_set_id, blockchain, "Destination").await
}

/// Helper function to ensure a wallet has testnet funds
///
/// Checks if the wallet has a balance, and if not (or balance is very low),
/// requests testnet tokens from the faucet.
pub async fn ensure_wallet_funded(
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
            let native_balance = balances
                .token_balances
                .iter()
                .find(|b| b.token.is_native)
                .and_then(|b| b.amount.parse::<f64>().ok())
                .unwrap_or(0.0);

            println!(
                "💰 Current wallet balance: {} native tokens",
                native_balance
            );

            // Request more tokens if balance is less than 0.01 (to cover pending transactions and new ones)
            if native_balance >= 0.01 {
                println!("✅ Wallet {} has sufficient balance", wallet.address);
                return Ok(());
            } else if native_balance > 0.0 {
                println!(
                    "⚠️  Wallet has {} tokens but less than 0.01, requesting more...",
                    native_balance
                );
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
    println!("   ⏳ Waiting 10 seconds for tokens to arrive...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Verify tokens arrived
    match view
        .get_token_balances(&wallet.id, QueryParamsBuilder::new().build())
        .await
    {
        Ok(balances) => {
            let new_balance = balances
                .token_balances
                .iter()
                .find(|b| b.token.is_native)
                .and_then(|b| b.amount.parse::<f64>().ok())
                .unwrap_or(0.0);

            println!("   💰 New balance: {} native tokens", new_balance);

            if new_balance < 0.01 {
                println!("   ⚠️  Balance still low. You may need to:");
                println!("      1. Wait longer for tokens to arrive");
                println!("      2. Fund the wallet manually at: https://sepoliafaucet.com/");
                println!("      3. Wallet address: {}", wallet.address);
            }
        }
        Err(_) => {
            println!("   ⚠️  Could not verify new balance");
        }
    }

    Ok(())
}
