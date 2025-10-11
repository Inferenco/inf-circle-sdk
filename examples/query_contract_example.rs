use inf_circle_sdk::{
    circle_view::circle_view::CircleView,
    contract::views::query_contract_view::QueryContractViewBodyBuilder, types::Blockchain,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new()?;

    println!("🔍 Circle SDK - Query Contract Example");
    println!("=======================================\n");

    // Example: Query an ERC20 token contract
    // You can use any deployed contract address
    let contract_address =
        env::var("CONTRACT_ADDRESS").unwrap_or_else(|_| "0xYourContractAddress".to_string());

    let blockchain = Blockchain::EthSepolia;

    println!("📋 Contract Details:");
    println!("   Address: {}", contract_address);
    println!("   Blockchain: {}\n", blockchain.as_str());

    // Example 1: Query with ABI function signature (recommended)
    println!("📊 Example 1: Query totalSupply() function");

    let query_result = view
        .query_contract(
            QueryContractViewBodyBuilder::new(blockchain.clone(), contract_address.clone())
                .abi_function_signature("totalSupply()".to_string())
                .abi_parameters(vec![]), // Empty array for functions with no parameters
        )
        .await?;

    println!("✅ Query successful!");
    if let Some(values) = &query_result.output_values {
        println!("   Output values: {:?}", values);
    } else {
        println!("   Output values: null (Circle couldn't decode, use output_data)");
    }
    println!("   Output data (hex): {}\n", query_result.output_data);

    // Example 2: Query with parameters (e.g., balanceOf)
    println!("📊 Example 2: Query balanceOf(address) function");

    let wallet_address =
        env::var("WALLET_ADDRESS").unwrap_or_else(|_| "0xYourWalletAddress".to_string());

    let balance_result = view
        .query_contract(
            QueryContractViewBodyBuilder::new(blockchain.clone(), contract_address.clone())
                .abi_function_signature("balanceOf(address)".to_string())
                .abi_parameters(vec![serde_json::json!(wallet_address)]),
        )
        .await?;

    println!("✅ Query successful!");
    if let Some(values) = &balance_result.output_values {
        println!("   Output values: {:?}", values);
    } else {
        println!("   Output values: null (use output_data for raw hex)");
    }
    println!("   Output data (hex): {}\n", balance_result.output_data);

    // Example 3: Query with ABI JSON (provides type information)
    println!("📊 Example 3: Query with full ABI");

    let abi_json = r#"[{"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}]"#;

    let query_with_abi = view
        .query_contract(
            QueryContractViewBodyBuilder::new(blockchain.clone(), contract_address.clone())
                .abi_function_signature("totalSupply()".to_string())
                .abi_json(abi_json.to_string()),
        )
        .await?;

    println!("✅ Query successful!");
    if let Some(values) = &query_with_abi.output_values {
        println!("   Output values: {:?}", values);
    } else {
        println!("   Output values: null (use output_data for raw hex)");
    }
    println!("   Output data (hex): {}\n", query_with_abi.output_data);

    // Example 4: Query with call data (pre-encoded)
    println!("📊 Example 4: Query with raw call data");

    // Call data for totalSupply() = 0x18160ddd
    let call_data = "0x18160ddd";

    let query_with_calldata = view
        .query_contract(
            QueryContractViewBodyBuilder::new(blockchain, contract_address.clone())
                .call_data(call_data.to_string()),
        )
        .await?;

    println!("✅ Query successful!");
    if let Some(values) = &query_with_calldata.output_values {
        println!("   Output values: {:?}", values);
    } else {
        println!("   Output values: null (use output_data for raw hex)");
    }
    println!(
        "   Output data (hex): {}\n",
        query_with_calldata.output_data
    );

    println!("✨ Done! You can use query_contract to read any view/pure function from contracts.");

    Ok(())
}
