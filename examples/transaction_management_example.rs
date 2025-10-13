//! Example of managing transactions (cancel and accelerate)

use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    dev_wallet::{
        dto::FeeLevel,
        ops::{
            accelerate_transaction::AccelerateTransactionRequestBuilder,
            cancel_transaction::CancelTransactionRequestBuilder,
            create_transfer_transaction::CreateTransferTransactionRequestBuilder,
        },
        views::{
            list_transactions::ListTransactionsParamsBuilder,
            list_wallets::ListDevWalletsParamsBuilder,
        },
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

    println!("⚡ Circle SDK - Transaction Management Example");
    println!("==============================================\n");

    // Get wallet set ID from environment
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")?;

    // Find wallets
    let params = ListDevWalletsParamsBuilder::new()
        .wallet_set_id(wallet_set_id)
        .blockchain(Blockchain::EthSepolia.as_str().to_string())
        .page_size(2)
        .build();

    let wallets = view.list_wallets(params).await?;

    if wallets.wallets.len() < 2 {
        println!("❌ Need at least 2 wallets. Run circle_ops_example.rs first.");
        return Ok(());
    }

    let source_wallet = &wallets.wallets[0];
    let dest_wallet = &wallets.wallets[1];

    println!(
        "Using source wallet: {} ({})",
        source_wallet.id, source_wallet.address
    );

    // List existing pending transactions
    println!("\n📋 Checking for pending transactions...");

    let tx_params = ListTransactionsParamsBuilder::new()
        .wallet_ids(source_wallet.id.clone())
        .build();

    let transactions = view.list_transactions(tx_params).await?;

    println!(
        "   Found {} pending/queued transactions",
        transactions.transactions.len()
    );

    if !transactions.transactions.is_empty() {
        let pending_tx = &transactions.transactions[0];
        println!("\n🔍 Example pending transaction:");
        println!("   ID: {}", pending_tx.id);
        println!("   State: {}", pending_tx.state);

        // Example 1: Accelerate a transaction
        println!("\n⚡ Example 1: Accelerating transaction...");
        println!("   This speeds up confirmation by increasing gas fees.");

        let accelerate_builder = AccelerateTransactionRequestBuilder::new(
            pending_tx.id.clone(),
            Uuid::new_v4().to_string(),
        )
        .build();

        match ops.accelerate_dev_transaction(accelerate_builder).await {
            Ok(response) => {
                println!("   ✅ Transaction accelerated!");
                println!("      New Transaction ID: {}", response.id);
            }
            Err(e) => {
                eprintln!("   ⚠️  Error: {}", e);
                println!("      Transaction may have already been confirmed.");
            }
        }

        // Example 2: Cancel a transaction
        println!("\n❌ Example 2: Canceling a transaction...");
        println!("   This attempts to cancel by submitting a higher-fee replacement.");

        let cancel_builder =
            CancelTransactionRequestBuilder::new(pending_tx.id.clone(), Uuid::new_v4().to_string())
                .build();

        match ops.cancel_dev_transaction(cancel_builder).await {
            Ok(response) => {
                println!("   ✅ Cancellation transaction created!");
                println!("      Cancellation TX ID: {}", response.id);
                println!("      State: {}", response.state);
            }
            Err(e) => {
                eprintln!("   ⚠️  Error: {}", e);
                println!("      Transaction may have already been confirmed.");
            }
        }
    } else {
        println!("\n💡 No pending transactions found.");
        println!("   Creating a new transfer transaction with LOW fee...");
        println!("   (Low fee = slower confirmation = good for testing cancel/accelerate)\n");

        // Create a slow transaction to demonstrate cancel/accelerate
        let transfer_builder =
            CreateTransferTransactionRequestBuilder::new(source_wallet.id.clone())
                .destination_address(dest_wallet.address.clone())
                .amounts(vec!["0.0001".to_string()])
                .blockchain(Blockchain::EthSepolia)
                .fee_level(FeeLevel::Low) // Low fee = slower
                .idempotency_key(Uuid::new_v4().to_string())
                .ref_id("slow-transfer-for-demo".to_string())
                .build();

        match ops.create_dev_transfer_transaction(transfer_builder).await {
            Ok(response) => {
                let tx_id = response.id.clone();
                println!("   ✅ Created slow transaction: {}", tx_id);
                println!("      State: {}", response.state);

                println!("\n   Now you can:");
                println!("   1. Wait a few seconds");
                println!("   2. Run this example again to see it in pending state");
                println!("   3. Try accelerating or canceling it");
            }
            Err(e) => {
                eprintln!("   ❌ Error creating transaction: {}", e);
            }
        }
    }

    println!("\n📚 Transaction Management Notes:");
    println!("   • Cancel: Replaces with 0-value tx with higher gas");
    println!("   • Accelerate: Replaces same tx with higher gas");
    println!("   • Both work only on PENDING transactions");
    println!("   • Confirmed transactions cannot be modified");
    println!("   • Additional gas fees apply");

    Ok(())
}
