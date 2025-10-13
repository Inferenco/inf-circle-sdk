//! Example of interacting with smart contracts (execute functions)

use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    contract::views::query_contract_view::QueryContractViewBodyBuilder,
    dev_wallet::{
        dto::FeeLevel,
        ops::create_contract_transaction::CreateContractExecutionTransactionRequestBuilder,
        views::list_wallets::ListDevWalletsParamsBuilder,
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

    println!("🔧 Circle SDK - Smart Contract Interaction Example");
    println!("===================================================\n");

    // Get wallet set ID from environment
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")?;

    // Find a wallet to use
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
    println!("Using Wallet: {} ({})", wallet.id, wallet.address);

    // Use USDC contract on Sepolia for demonstration
    let usdc_contract = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";

    println!("\n📋 Contract: USDC on Sepolia");
    println!("   Address: {}\n", usdc_contract);

    // Example 1: Read contract state (query)
    println!("1️⃣  Querying contract (read-only)...");
    println!("   Calling: balanceOf(address)");

    let query_builder =
        QueryContractViewBodyBuilder::new(Blockchain::EthSepolia, usdc_contract.to_string())
            .abi_function_signature("balanceOf(address)".to_string())
            .abi_parameters(vec![serde_json::json!(wallet.address)]);

    match view.query_contract(query_builder).await {
        Ok(response) => {
            println!("   ✅ Query successful!");
            println!("      Balance: {:?}", response.output_values);
            println!("      Output data: {}", response.output_data);
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
        }
    }

    // Example 2: Write to contract (execute transaction)
    println!("\n2️⃣  Executing contract function (write operation)...");
    println!("   Calling: approve(address spender, uint256 amount)");

    let spender_address = "0x0000000000000000000000000000000000000001"; // Example address
    let amount = "1000000"; // 1 USDC (6 decimals)

    use inf_circle_sdk::dev_wallet::dto::AbiParameter;

    let execute_builder = CreateContractExecutionTransactionRequestBuilder::new(
        wallet.id.clone(),
        usdc_contract.to_string(),
        Uuid::new_v4().to_string(),
    )
    .abi_function_signature("approve(address,uint256)".to_string())
    .abi_parameters(vec![
        AbiParameter::String(spender_address.to_string()),
        AbiParameter::String(amount.to_string()),
    ])
    .fee_level(FeeLevel::Medium)
    .ref_id("example-approve-tx".to_string())
    .build();

    match ops
        .create_dev_contract_execution_transaction(execute_builder)
        .await
    {
        Ok(response) => {
            println!("   ✅ Transaction created!");
            println!("      Transaction ID: {}", response.id);
            println!("      State: {}", response.state);
        }
        Err(e) => {
            eprintln!("   ❌ Error: {}", e);
            println!("      Make sure wallet has ETH for gas fees.");
        }
    }

    println!("\n💡 Contract Interaction Tips:");
    println!("   • Use query_contract() for read-only calls (free, no gas)");
    println!("   • Use create_contract_execution_transaction() for writes (costs gas)");
    println!("   • ABI function signature: functionName(type1,type2,...)");
    println!("   • Parameters must match function signature exactly");
    println!("\n📚 Common Functions:");
    println!("   • ERC-20: approve(address,uint256), transfer(address,uint256)");
    println!("   • ERC-721: safeTransferFrom(address,address,uint256)");
    println!("   • ERC-1155: safeTransferFrom(address,address,uint256,uint256,bytes)");

    Ok(())
}
