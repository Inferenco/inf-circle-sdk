use inf_circle_sdk::{
    circle_view::circle_view::CircleView,
    contract::views::estimate_contract_deployment::EstimateContractDeploymentBodyBuilder,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new()?;

    println!("🔧 Circle SDK - Estimate Contract Deployment Fee Example");
    println!("=========================================================\n");

    // Get wallet ID from environment
    let wallet_id =
        env::var("CIRCLE_WALLET_ID").expect("CIRCLE_WALLET_ID must be set (from a wallet you own)");

    // Get bytecode from environment variable
    // You can compile a Solidity contract using: solc --bin YourContract.sol
    let bytecode = env::var("CONTRACT_BYTECODE").expect(
        "CONTRACT_BYTECODE must be set. Compile a Solidity contract and provide its bytecode.",
    );

    println!("📊 Estimating contract deployment fee...");
    println!("   Wallet ID: {}", wallet_id);
    println!("   Bytecode length: {} bytes\n", bytecode.len());

    // Example 1: Simple estimate with wallet ID (recommended)
    let fee_estimate = view
        .estimate_contract_deployment_fee(
            EstimateContractDeploymentBodyBuilder::new(bytecode.to_string())
                .wallet_id(wallet_id.clone()),
        )
        .await?;

    println!("✅ Fee Estimation (No Constructor):");
    println!("   Low:");
    println!("      Gas Limit: {}", fee_estimate.low.gas_limit);
    if let Some(network_fee) = &fee_estimate.low.network_fee {
        println!("      Network Fee: {} ETH", network_fee);
    }
    if let Some(max_fee) = &fee_estimate.low.max_fee {
        println!("      Max Fee: {} gwei", max_fee);
    }
    println!();

    println!("   Medium:");
    println!("      Gas Limit: {}", fee_estimate.medium.gas_limit);
    if let Some(network_fee) = &fee_estimate.medium.network_fee {
        println!("      Network Fee: {} ETH", network_fee);
    }
    if let Some(max_fee) = &fee_estimate.medium.max_fee {
        println!("      Max Fee: {} gwei", max_fee);
    }
    println!();

    println!("   High:");
    println!("      Gas Limit: {}", fee_estimate.high.gas_limit);
    if let Some(network_fee) = &fee_estimate.high.network_fee {
        println!("      Network Fee: {} ETH", network_fee);
    }
    if let Some(max_fee) = &fee_estimate.high.max_fee {
        println!("      Max Fee: {} gwei", max_fee);
    }
    println!();

    // Example 2: Estimate with constructor parameters
    println!("📊 Estimating deployment with constructor parameters...");

    // Constructor parameters for an ERC20 token: constructor(string name, string symbol, uint256 initialSupply)
    let constructor_params = vec![
        serde_json::json!("MyToken"),
        serde_json::json!("MTK"),
        serde_json::json!("1000000000000000000000"), // 1000 tokens (18 decimals)
    ];

    let fee_estimate_with_constructor = view
        .estimate_contract_deployment_fee(
            EstimateContractDeploymentBodyBuilder::new(bytecode.to_string())
                .wallet_id(wallet_id)
                .constructor_signature("constructor(string,string,uint256)".to_string())
                .constructor_parameters(constructor_params),
        )
        .await?;

    println!("✅ Fee Estimation (With Constructor):");
    println!(
        "   Medium Network Fee: {:?}",
        fee_estimate_with_constructor.medium.network_fee
    );
    println!(
        "   Medium Gas Limit: {}",
        fee_estimate_with_constructor.medium.gas_limit
    );

    println!("\n✨ Done! You can now use this fee estimate to deploy your contract.");

    Ok(())
}
