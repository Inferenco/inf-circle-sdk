//! Example of importing an existing smart contract into Circle

use inf_circle_sdk::{
    circle_ops::circler_ops::CircleOps,
    circle_view::circle_view::CircleView,
    contract::{dto::ListContractsParams, ops::import_contract::ImportContractRequestBuilder},
    types::Blockchain,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleOps and CircleView
    let ops = CircleOps::new(None)?;
    let view = CircleView::new()?;

    println!("📥 Circle SDK - Import Contract Example");
    println!("========================================\n");

    // Example 1: Import USDC on Sepolia
    println!("1️⃣  Importing USDC contract on Sepolia...");

    let usdc_address = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";
    println!("   Contract Address: {}", usdc_address);

    // Check if already imported
    let check_params = ListContractsParams {
        address: Some(usdc_address.to_string()),
        blockchain: Some(Blockchain::EthSepolia),
        ..Default::default()
    };

    let existing = view.list_contracts(Some(check_params)).await?;

    if !existing.contracts.is_empty() {
        println!("   ℹ️  Contract already imported!");
        println!("      Contract ID: {:?}", existing.contracts[0].id);
        println!("      Name: {:?}", existing.contracts[0].name);
    } else {
        let import_builder = ImportContractRequestBuilder::new(
            Blockchain::EthSepolia,
            usdc_address.to_string(),
            "USDCSepoliaImported".to_string(), // Alphanumeric name only
        )
        .description(Some("USD Coin on Sepolia testnet".to_string()))
        .build();

        match ops.import_contract(import_builder).await {
            Ok(response) => {
                println!("   ✅ Contract imported successfully!");
                println!("      Contract ID: {:?}", response.contract.id);
                println!("      Address: {:?}", response.contract.contract_address);
                println!("      Blockchain: {:?}", response.contract.blockchain);
            }
            Err(e) => {
                eprintln!("   ❌ Error: {}", e);
            }
        }
    }

    // Example 2: Import a custom contract
    println!("\n2️⃣  Importing a custom contract...");
    println!("   (Replace with your own contract address)\n");

    let custom_contract = "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb"; // Example address

    let import_custom = ImportContractRequestBuilder::new(
        Blockchain::EthSepolia,
        custom_contract.to_string(),
        "MyCustomContract".to_string(),
    )
    .description(Some("My deployed contract".to_string()))
    .build();

    match ops.import_contract(import_custom).await {
        Ok(response) => {
            println!("   ✅ Custom contract imported!");
            println!("      Contract ID: {:?}", response.contract.id);
        }
        Err(e) => {
            eprintln!("   ⚠️  Error: {}", e);
            println!("      This is expected if the contract doesn't exist or already imported.");
        }
    }

    // List all imported contracts
    println!("\n3️⃣  Listing all imported contracts...");

    let all_contracts = view.list_contracts(None).await?;
    println!("   ✅ Total contracts: {}", all_contracts.contracts.len());

    for (i, contract) in all_contracts.contracts.iter().take(5).enumerate() {
        println!(
            "      {}. {} - {:?}",
            i + 1,
            contract.name.as_ref().unwrap_or(&"Unnamed".to_string()),
            contract.contract_address
        );
    }

    if all_contracts.contracts.len() > 5 {
        println!("      ... and {} more", all_contracts.contracts.len() - 5);
    }

    println!("\n💡 Why Import Contracts?");
    println!("   • Create event monitors for external contracts");
    println!("   • Query contract state without deploying");
    println!("   • Execute functions on third-party contracts");
    println!("   • Track contracts you interact with");

    println!("\n📚 Next Steps:");
    println!("   • Use create_event_monitor_example.rs to monitor events");
    println!("   • Use query_contract_example.rs to read contract data");
    println!("   • Use contract_interaction_example.rs to execute functions");

    Ok(())
}
