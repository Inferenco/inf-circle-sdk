use inf_circle_sdk::circle_view::circle_view::CircleView;
use inf_circle_sdk::contract::dto::{ListEventLogsParams, ListEventMonitorsParams};
use inf_circle_sdk::contract::views::create_event_monitor::CreateEventMonitorBodyBuilder;
use inf_circle_sdk::contract::views::update_event_monitor::UpdateEventMonitorBodyBuilder;
use inf_circle_sdk::helper::PaginationParams;
use inf_circle_sdk::types::Blockchain;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize CircleView (expects CIRCLE_API_KEY in environment)
    let client = CircleView::new()?;

    // Example: Create an event monitor for a Transfer event on an ERC20 contract
    println!("Creating event monitor for Transfer events...");

    // Generate a unique idempotency key
    let idempotency_key = Uuid::new_v4().to_string();

    // Example contract address (replace with your actual contract address)
    let contract_address = "0x1e124d7384cd34448ea5907bd0052a79355ab5eb";

    // Example event signature for Transfer event (no spaces)
    // Standard ERC20 Transfer event: Transfer(address indexed from, address indexed to, uint256 value)
    let event_signature = "Transfer(address indexed from, address indexed to, uint256 value)";

    // Build the request using the builder
    let builder = CreateEventMonitorBodyBuilder::new(
        idempotency_key.clone(),
        event_signature.to_string(),
        contract_address.to_string(),
        Blockchain::EthSepolia, // Use appropriate blockchain
    );

    // Create the event monitor
    match client.create_event_monitor(builder).await {
        Ok(response) => {
            println!("✅ Event monitor created successfully!");
            println!("Monitor ID: {}", response.event_monitor.id);
            println!(
                "Contract Address: {}",
                response.event_monitor.contract_address
            );
            println!(
                "Event Signature: {}",
                response.event_monitor.event_signature
            );
            println!(
                "Event Signature Hash: {}",
                response.event_monitor.event_signature_hash
            );
            println!("Is Enabled: {}", response.event_monitor.is_enabled);
            println!("Blockchain: {:?}", response.event_monitor.blockchain);
        }
        Err(e) => {
            eprintln!("❌ Error creating event monitor: {}", e);
        }
    }

    // Example: Create an event monitor for a custom event
    println!("\nCreating event monitor for custom event...");

    let idempotency_key2 = Uuid::new_v4().to_string();
    let custom_event_signature =
        "Approval(address indexed owner, address indexed spender, uint256 value)";

    let builder2 = CreateEventMonitorBodyBuilder::new(
        idempotency_key2.clone(),
        custom_event_signature.to_string(),
        contract_address.to_string(),
        Blockchain::EthSepolia,
    );

    let monitor_id = match client.create_event_monitor(builder2).await {
        Ok(response) => {
            println!("✅ Custom event monitor created successfully!");
            println!("Monitor ID: {}", response.event_monitor.id);
            println!(
                "Event Signature: {}",
                response.event_monitor.event_signature
            );
            response.event_monitor.id
        }
        Err(e) => {
            eprintln!("❌ Error creating custom event monitor: {}", e);
            return Ok(());
        }
    };

    // Example: Update an event monitor (disable it)
    println!("\nUpdating event monitor (disabling it)...");

    let update_builder = UpdateEventMonitorBodyBuilder::new(monitor_id.clone(), false);

    match client.update_event_monitor(update_builder).await {
        Ok(response) => {
            println!("✅ Event monitor updated successfully!");
            println!("Monitor ID: {}", response.event_monitor.id);
            println!("Is Enabled: {}", response.event_monitor.is_enabled);
        }
        Err(e) => {
            eprintln!("❌ Error updating event monitor: {}", e);
        }
    }

    // Example: Re-enable the event monitor
    println!("\nRe-enabling event monitor...");

    let update_builder2 = UpdateEventMonitorBodyBuilder::new(monitor_id.clone(), true);

    match client.update_event_monitor(update_builder2).await {
        Ok(response) => {
            println!("✅ Event monitor re-enabled successfully!");
            println!("Monitor ID: {}", response.event_monitor.id);
            println!("Is Enabled: {}", response.event_monitor.is_enabled);
        }
        Err(e) => {
            eprintln!("❌ Error re-enabling event monitor: {}", e);
        }
    }

    // Example: List all event monitors
    println!("\nListing all event monitors...");

    match client.list_event_monitors(None).await {
        Ok(response) => {
            println!(
                "✅ Found {} event monitors total",
                response.event_monitors.len()
            );
            for (i, monitor) in response.event_monitors.iter().enumerate().take(5) {
                println!(
                    "   {}. ID: {}, Contract: {}, Event: {}",
                    i + 1,
                    monitor.id,
                    monitor.contract_address,
                    monitor.event_signature
                );
            }
            if response.event_monitors.len() > 5 {
                println!("   ... and {} more", response.event_monitors.len() - 5);
            }
        }
        Err(e) => {
            eprintln!("❌ Error listing event monitors: {}", e);
        }
    }

    // Example: List event monitors filtered by contract address
    println!("\nListing event monitors for specific contract...");

    let params = ListEventMonitorsParams {
        contract_address: Some(contract_address.to_string()),
        blockchain: None,
        event_signature: None,
        from: None,
        to: None,
        pagination: PaginationParams::default(),
    };

    match client.list_event_monitors(Some(params)).await {
        Ok(response) => {
            println!(
                "✅ Found {} event monitors for contract {}",
                response.event_monitors.len(),
                contract_address
            );
            for monitor in &response.event_monitors {
                println!(
                    "   - Event: {}, Enabled: {}",
                    monitor.event_signature, monitor.is_enabled
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Error listing filtered event monitors: {}", e);
        }
    }

    // Example: List event logs
    println!("\nListing event logs...");

    match client.list_event_logs(None).await {
        Ok(response) => {
            println!("✅ Found {} event logs total", response.event_logs.len());

            // Show first 3 event logs as samples
            for (i, log) in response.event_logs.iter().enumerate().take(3) {
                println!("\n   Event Log #{}:", i + 1);
                println!("   - ID: {}", log.id);
                println!("   - Contract: {}", log.contract_address);
                println!("   - Event: {}", log.event_signature);
                println!(
                    "   - Block: {} (Hash: {})",
                    log.block_height,
                    &log.block_hash[..10]
                );
                println!("   - Tx Hash: {}", &log.tx_hash[..20]);
                println!("   - Confirmed: {}", log.first_confirm_date);
            }

            if response.event_logs.len() > 3 {
                println!(
                    "\n   ... and {} more event logs",
                    response.event_logs.len() - 3
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Error listing event logs: {}", e);
        }
    }

    // Example: List event logs filtered by contract
    println!("\nListing event logs for specific contract...");

    let log_params = ListEventLogsParams {
        contract_address: Some(contract_address.to_string()),
        blockchain: Some(Blockchain::EthSepolia),
        from: None,
        to: None,
        pagination: PaginationParams {
            page_size: Some(5),
            ..Default::default()
        },
    };

    match client.list_event_logs(Some(log_params)).await {
        Ok(response) => {
            println!(
                "✅ Found {} event logs for contract {}",
                response.event_logs.len(),
                contract_address
            );

            for log in &response.event_logs {
                println!(
                    "   - Event: {} (Block {})",
                    log.event_signature, log.block_height
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Error listing filtered event logs: {}", e);
        }
    }

    // Example: Delete the event monitor
    println!("\nDeleting event monitor...");

    match client.delete_event_monitor(&monitor_id).await {
        Ok(_) => {
            println!("✅ Event monitor deleted successfully!");
            println!("Monitor ID: {}", monitor_id);
        }
        Err(e) => {
            eprintln!("❌ Error deleting event monitor: {}", e);
        }
    }

    Ok(())
}
