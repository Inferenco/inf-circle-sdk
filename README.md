'''
# Circle Rust SDK

[![Crates.io](https://img.shields.io/crates/v/inf-circle-sdk.svg)](https://crates.io/crates/inf-circle-sdk)
[![Docs.rs](https://docs.rs/inf-circle-sdk/badge.svg)](https://docs.rs/inf-circle-sdk)

A comprehensive Rust SDK for Circle's API, designed with a clean separation between write (`CircleOps`) and read (`CircleView`) operations. This library provides a type-safe, async-first interface to interact with Circle's powerful developer APIs.

## Architecture

The SDK is split into two main components:

- **`CircleOps`**: Handles all write operations (POST, PUT, PATCH) that require entity-level authentication. It uses an entity secret to sign requests and provides a secure way to create and manage resources.
- **`CircleView`**: Handles all read operations (GET) that do not require entity-level authentication. It provides a simple interface to query data from the Circle API.

This separation ensures that read-only processes do not require access to sensitive entity secrets, enhancing security and simplifying access control.

## Features

- **Clean Separation**: `CircleOps` for writes, `CircleView` for reads.
- **Async First**: Built on `tokio` for non-blocking, asynchronous operations.
- **Type-Safe**: Strongly typed request and response structures to prevent common errors.
- **Fluent Builders**: Easy-to-use builders for constructing complex API requests.
- **Comprehensive Error Handling**: Detailed error types to simplify debugging.
- **Extensible**: Modular design makes it easy to add new API endpoints.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
inf-circle-sdk = "0.1.0"
```

## Usage

### Environment Variables

Create a `.env` file in your project root with the following variables:

```
CIRCLE_BASE_URL="https://api.circle.com"
CIRCLE_API_KEY="YOUR_API_KEY"
CIRCLE_ENTITY_SECRET="YOUR_ENTITY_SECRET_HEX"
CIRCLE_PUBLIC_KEY="-----BEGIN PUBLIC KEY-----\nYOUR_RSA_PUBLIC_KEY_IN_PEM_FORMAT\n-----END PUBLIC KEY-----"
CIRCLE_WALLET_SET_ID="YOUR_WALLET_SET_ID"
```

**Note:** 
- `CIRCLE_API_KEY` is used for API authentication in the Authorization header
- `CIRCLE_ENTITY_SECRET` should be the hex-encoded entity secret
- `CIRCLE_PUBLIC_KEY` should be the RSA public key in PEM format (PKCS#1 or PKCS#8)
- The entity secret is automatically encrypted using RSA-OAEP with SHA-256 at request time
- Each API call generates a fresh encryption and unique UUID for security

### CircleOps: Write Operations

Use `CircleOps` to create, update, or otherwise modify resources.

```rust
use inf_circle_sdk::circle_ops::circler_ops::CircleOps;
use inf_circle_sdk::wallet::{
    dto::{AccountType, Blockchain},
    wallet_ops::CreateWalletRequestBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize CircleOps from environment variables
    let ops = CircleOps::new()?;

    // Get wallet set ID from environment variables
    let wallet_set_id =
        std::env::var("CIRCLE_WALLET_SET_ID").expect("CIRCLE_WALLET_SET_ID must be set");

    // Build the request to create a new SCA wallet on Ethereum Sepolia
    // The entity secret will be automatically encrypted at request time using CIRCLE_ENTITY_SECRET and CIRCLE_PUBLIC_KEY
    let request_builder = CreateWalletRequestBuilder::new(
        wallet_set_id,
        vec![Blockchain::EthSepolia],
    )
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

    Ok(())
}
```

### CircleView: Read Operations

Use `CircleView` to fetch data from the API.

```rust
use inf_circle_sdk::circle_view::circle_view::CircleView;
use inf_circle_sdk::wallet::wallet_view::ListWalletsParamsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize CircleView from environment variables
    let view = CircleView::new()?;

    // Build the request to list the first 10 wallets
    let params = ListWalletsParamsBuilder::new().page_size(10).build();

    // Send the request and print the response
    match view.list_wallets(Some(params)).await {
        Ok(response) => {
            println!("Successfully listed wallets: {:#?}", response.wallets);
        }
        Err(e) => {
            eprintln!("Error listing wallets: {}", e);
        }
    }

    Ok(())
}
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
'''

