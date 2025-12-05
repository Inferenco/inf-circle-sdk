use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    contract::ops::deploy_contract::DeployContractRequestBuilder, types::Blockchain,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps
    let ops = CircleOps::new(None)?;

    println!("🚀 Circle SDK - Deploy Contract from Bytecode Example");
    println!("=====================================================\n");

    // Get wallet ID from environment
    let wallet_id =
        env::var("CIRCLE_WALLET_ID").expect("CIRCLE_WALLET_ID must be set (from a funded wallet)");

    // Example: Simple Storage contract
    // Source:
    // contract SimpleStorage {
    //     uint256 public storedData;
    //     function set(uint256 x) public { storedData = x; }
    // }
    let bytecode = env::var("CONTRACT_BYTECODE")
        .expect("CONTRACT_BYTECODE must be set. Compile your contract and provide its bytecode.");

    let abi_json = env::var("CONTRACT_ABI")
        .expect("CONTRACT_ABI must be set. Provide your contract's ABI in JSON string format.");

    println!("📝 Contract Details:");
    println!("   Wallet ID: {}", wallet_id);
    println!("   Blockchain: ETH-SEPOLIA");
    println!("   Bytecode length: {} bytes", bytecode.len());
    println!("   ABI length: {} bytes\n", abi_json.len());

    // Create deployment request
    let contract_name = format!(
        "MyContract{}",
        uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string()
    );

    let deployment = ops
        .deploy_contract(
            DeployContractRequestBuilder::new(
                bytecode,
                abi_json,
                wallet_id,
                contract_name,
                Blockchain::EthSepolia,
            )
            .description("Deployed via Circle Rust SDK".to_string())
            .fee_level("MEDIUM".to_string())
            .ref_id(format!("deploy-{}", uuid::Uuid::new_v4())),
        )
        .await?;

    println!("✅ Contract deployment initiated!");
    println!("   Contract ID: {}", deployment.contract_id);
    println!("   Transaction ID: {}", deployment.transaction_id);
    println!("\n💡 Track deployment status:");
    println!("   - Use the transaction ID to monitor deployment progress");
    println!("   - Once confirmed, use the contract ID to interact with your contract");

    Ok(())
}
