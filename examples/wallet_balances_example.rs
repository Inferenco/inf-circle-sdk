//! Example of querying wallet balances and NFTs

use inf_circle_sdk::{
    circle_view::circle_view::CircleView,
    dev_wallet::views::{list_wallets::ListDevWalletsParamsBuilder, query::QueryParamsBuilder},
    types::Blockchain,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView
    let view = CircleView::new()?;

    println!("💰 Circle SDK - Wallet Balances & NFTs Example");
    println!("===============================================\n");

    // Get wallet set ID from environment
    let wallet_set_id = env::var("CIRCLE_WALLET_SET_ID")?;

    // List wallets
    println!("📋 Listing wallets...");
    let params = ListDevWalletsParamsBuilder::new()
        .wallet_set_id(wallet_set_id)
        .blockchain(Blockchain::EthSepolia.as_str().to_string())
        .page_size(5)
        .build();

    let wallets = view.list_wallets(params).await?;
    println!("✅ Found {} wallets\n", wallets.wallets.len());

    if wallets.wallets.is_empty() {
        println!("No wallets found. Create wallets first using circle_ops_example.rs");
        return Ok(());
    }

    // Check balances and NFTs for each wallet
    for (i, wallet) in wallets.wallets.iter().enumerate() {
        println!("{}. Wallet: {} ({})", i + 1, wallet.id, wallet.address);
        println!("   ═══════════════════════════════════════");

        // Get token balances
        let balance_params = QueryParamsBuilder::new().build();
        match view.get_token_balances(&wallet.id, balance_params).await {
            Ok(balances) => {
                println!("   💰 Token Balances:");
                if balances.token_balances.is_empty() {
                    println!("      No tokens found");
                } else {
                    for balance in &balances.token_balances {
                        let native_tag = if balance.token.is_native {
                            " (native)"
                        } else {
                            ""
                        };
                        let symbol = balance.token.symbol.as_deref().unwrap_or("UNKNOWN");
                        println!("      {}: {}{}", symbol, balance.amount, native_tag);
                    }
                }
            }
            Err(e) => {
                println!("      ⚠️  Error fetching balances: {}", e);
            }
        }

        // Get NFTs
        let nft_params = QueryParamsBuilder::new().build();
        match view.get_nfts(&wallet.id, nft_params).await {
            Ok(nfts) => {
                println!("   🎨 NFTs:");
                if nfts.nfts.is_empty() {
                    println!("      No NFTs found");
                } else {
                    for nft in nfts.nfts.iter().take(3) {
                        let name = nft.token.name.as_deref().unwrap_or("Unknown");
                        let symbol = nft.token.symbol.as_deref().unwrap_or("NFT");
                        let token_id = nft.nft_token_id.as_deref().unwrap_or("N/A");
                        println!("      {} #{} - {}", name, token_id, symbol);
                    }
                    if nfts.nfts.len() > 3 {
                        println!("      ... and {} more NFTs", nfts.nfts.len() - 3);
                    }
                }
            }
            Err(e) => {
                println!("      ⚠️  Error fetching NFTs: {}", e);
            }
        }

        println!();
    }

    println!("💡 Tip: Use view.list_wallets_with_token_balances() to get balances in one call");

    Ok(())
}
