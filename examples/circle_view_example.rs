//! Example of using CircleView to list wallets.
use inf_circle_sdk::circle_view::circle_view::CircleView;
use inf_circle_sdk::dev_wallet::views::list_wallets::ListDevWalletsParamsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize CircleView from environment variables
    let view = CircleView::new()?;

    // Build the request to list the first 10 wallets
    let params = ListDevWalletsParamsBuilder::new().page_size(10).build();

    // Send the request and print the response
    match view.list_wallets(params).await {
        Ok(response) => {
            println!("Successfully listed wallets: {:#?}", response.wallets);
        }
        Err(e) => {
            eprintln!("Error listing wallets: {}", e);
        }
    }

    Ok(())
}
