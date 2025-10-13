//! Example of signing messages and typed data with Circle wallets

use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    dev_wallet::{
        ops::{sign_data::SignDataRequestBuilder, sign_message::SignMessageRequestBuilder},
        views::list_wallets::ListDevWalletsParamsBuilder,
    },
    types::Blockchain,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new()?;
    let view = CircleView::new()?;

    println!("✍️  Circle SDK - Sign Message & Typed Data Example");
    println!("===================================================\n");

    // Get wallet set ID from environment
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")?;

    // Find a wallet to use for signing
    let params = ListDevWalletsParamsBuilder::new()
        .wallet_set_id(wallet_set_id)
        .blockchain(Blockchain::EthSepolia.as_str().to_string())
        .page_size(1)
        .build();

    let wallets = view.list_wallets(params).await?;

    if wallets.wallets.is_empty() {
        println!("❌ No wallets found. Create a wallet first using circle_ops_example.rs");
        return Ok(());
    }

    let wallet = &wallets.wallets[0];
    println!("Using Wallet:");
    println!("  ID: {}", wallet.id);
    println!("  Address: {}", wallet.address);
    println!();

    // Example 1: Sign a simple text message
    println!("1️⃣  Signing a simple message...");

    let message = "Hello from Circle SDK!";
    println!("   Message: \"{}\"", message);

    let sign_builder = SignMessageRequestBuilder::new(wallet.id.clone(), message.to_string())?
        .encoded_by_hex(false)
        .memo("Example message signature".to_string())
        .build();

    match ops.dev_sign_message(sign_builder).await {
        Ok(response) => {
            println!("   ✅ Message signed successfully!");
            println!("   Signature: {}", response.signature);
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
        }
    }

    // Example 2: Sign typed data (EIP-712)
    println!("\n2️⃣  Signing EIP-712 typed data...");

    let typed_data = serde_json::json!({
        "types": {
            "EIP712Domain": [
                {"name": "name", "type": "string"},
                {"name": "version", "type": "string"},
                {"name": "chainId", "type": "uint256"}
            ],
            "Person": [
                {"name": "name", "type": "string"},
                {"name": "wallet", "type": "address"}
            ]
        },
        "primaryType": "Person",
        "domain": {
            "name": "Circle SDK Example",
            "version": "1",
            "chainId": 11155111  // Sepolia
        },
        "message": {
            "name": "Alice",
            "wallet": wallet.address
        }
    });

    println!(
        "   Typed Data: {}",
        serde_json::to_string_pretty(&typed_data)?
    );

    let typed_data_builder =
        SignDataRequestBuilder::new(wallet.id.clone(), typed_data.to_string())?
            .memo("Example EIP-712 signature".to_string())
            .build();

    match ops.dev_sign_data(typed_data_builder).await {
        Ok(response) => {
            println!("   ✅ Typed data signed successfully!");
            println!("   Signature: {}", response.signature);
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
        }
    }

    println!("\n💡 Use Cases:");
    println!("   • Sign messages for authentication");
    println!("   • Sign typed data for dApps (MetaMask-style)");
    println!("   • Prove wallet ownership");
    println!("   • Create off-chain signatures for gasless transactions");

    Ok(())
}
