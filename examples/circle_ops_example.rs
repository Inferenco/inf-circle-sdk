//! Example of using CircleOps to create a wallet.
use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
use inf_circle_sdk::types::Blockchain;
use inf_circle_sdk::wallet::{dto::AccountType, ops::create_wallet::CreateWalletRequestBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize CircleOps from environment variables
    let ops = CircleOps::new()?;

    // Get wallet set ID from environment variables
    let wallet_set_id =
        std::env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID must be set");

    // Build the request to create a new SCA wallet on Ethereum Sepolia
    // The entity secret will be automatically encrypted at request time using CIRCLE_ENTITY_SECRET and CIRCLE_PUBLIC_KEY
    let request_builder =
        CreateWalletRequestBuilder::new(wallet_set_id, vec![Blockchain::EthSepolia])
            .unwrap()
            .account_type(AccountType::Sca)
            .count(1)
            .name("My First SCA Wallet".to_string())
            .build();

    // Send the request and print the response
    match ops.create_wallet(request_builder).await {
        Ok(response) => {
            println!("Successfully created wallets: {:#?}", response.wallets);
        }
        Err(e) => {
            eprintln!("Error creating wallets: {}", e);
        }
    }

    // Demonstrate that the same builder can be reused for multiple requests
    // Each request will have a different encrypted entity secret and UUID
    println!("\n--- Demonstrating reusable builder pattern ---");

    let reusable_builder = CreateWalletRequestBuilder::new(
        std::env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID must be set"),
        vec![Blockchain::EthSepolia],
    )
    .unwrap()
    .account_type(AccountType::Eoa)
    .count(1);

    // First request
    println!("Making first request...");
    if let Err(e) = ops
        .create_wallet(
            reusable_builder
                .clone()
                .name("First Wallet".to_string())
                .build(),
        )
        .await
    {
        println!("First request error (expected): {}", e);
    }

    // Second request - will have different encryption and UUID even though builder is the same
    println!("Making second request...");
    if let Err(e) = ops
        .create_wallet(
            reusable_builder
                .clone()
                .name("Second Wallet".to_string())
                .build(),
        )
        .await
    {
        println!("Second request error (expected): {}", e);
    }

    println!("Each request above generated unique encryption and UUID!");

    Ok(())
}
