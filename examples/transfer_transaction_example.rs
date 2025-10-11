//! Example of creating transfer transactions (native tokens and ERC-20 tokens)

use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    dev_wallet::{
        dto::FeeLevel,
        ops::create_transfer_transaction::CreateTransferTransactionRequestBuilder,
        views::{list_wallets::ListDevWalletsParamsBuilder, query::QueryParamsBuilder},
    },
    types::Blockchain,
};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new()?;
    let view = CircleView::new()?;

    println!("💸 Circle SDK - Transfer Transaction Example");
    println!("=============================================\n");

    // Get wallet set ID from environment
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")?;

    // List wallets to find source and destination
    let params = ListDevWalletsParamsBuilder::new()
        .wallet_set_id(wallet_set_id.clone())
        .blockchain(Blockchain::EthSepolia.as_str().to_string())
        .page_size(10)
        .build();

    let wallets = view.list_wallets(params).await?;

    if wallets.wallets.len() < 2 {
        println!("❌ Need at least 2 wallets for transfer example.");
        println!("   Run circle_ops_example.rs to create wallets first.");
        return Ok(());
    }

    let source_wallet = &wallets.wallets[0];
    let dest_wallet = &wallets.wallets[1];

    println!("Source Wallet:");
    println!("  ID: {}", source_wallet.id);
    println!("  Address: {}", source_wallet.address);

    println!("\nDestination Wallet:");
    println!("  ID: {}", dest_wallet.id);
    println!("  Address: {}", dest_wallet.address);

    // Check source wallet balance
    let balance_params = QueryParamsBuilder::new().build();
    let balances = view
        .get_token_balances(&source_wallet.id, balance_params)
        .await?;

    println!("\n💰 Source Wallet Balances:");
    for balance in &balances.token_balances {
        let symbol = balance.token.symbol.as_deref().unwrap_or("UNKNOWN");
        let native_tag = if balance.token.is_native {
            " (native)"
        } else {
            ""
        };
        println!("  {}: {}{}", symbol, balance.amount, native_tag);
    }

    // Example 1: Transfer native tokens (ETH)
    println!("\n📤 Example 1: Transferring 0.001 ETH...");

    let transfer_builder = CreateTransferTransactionRequestBuilder::new(source_wallet.id.clone())
        .destination_address(dest_wallet.address.clone())
        .amounts(vec!["0.001".to_string()]) // 0.001 ETH
        .blockchain(Blockchain::EthSepolia)
        .fee_level(FeeLevel::Medium)
        .idempotency_key(Uuid::new_v4().to_string())
        .ref_id("example-eth-transfer".to_string())
        .build();

    match ops.create_transfer_transaction(transfer_builder).await {
        Ok(response) => {
            println!("✅ Native token transfer created!");
            println!("   Transaction ID: {}", response.id);
            println!("   State: {}", response.state);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            println!("   Make sure source wallet has sufficient ETH balance.");
        }
    }

    // Example 2: Transfer ERC-20 tokens (USDC)
    println!("\n📤 Example 2: Transferring 1.0 USDC...");

    let usdc_address = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238"; // USDC on Sepolia

    let erc20_transfer = CreateTransferTransactionRequestBuilder::new(source_wallet.id.clone())
        .destination_address(dest_wallet.address.clone())
        .amounts(vec!["1.0".to_string()]) // 1.0 USDC
        .token_address(usdc_address.to_string())
        .blockchain(Blockchain::EthSepolia)
        .fee_level(FeeLevel::High)
        .idempotency_key(Uuid::new_v4().to_string())
        .ref_id("example-usdc-transfer".to_string())
        .build();

    match ops.create_transfer_transaction(erc20_transfer).await {
        Ok(response) => {
            println!("✅ ERC-20 token transfer created!");
            println!("   Transaction ID: {}", response.id);
            println!("   State: {}", response.state);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            println!("   Make sure source wallet has:");
            println!("   1. USDC tokens");
            println!("   2. ETH for gas fees");
            println!("   Get USDC from: https://faucet.circle.com/");
        }
    }

    println!("\n💡 Tip: Use view.get_transaction(tx_id) to check transaction status");

    Ok(())
}
