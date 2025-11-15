//! NEAR Protocol Helper Functions
//!
//! This module provides utility functions for working with NEAR protocol,
//! including account balance queries, delegate action serialization, and public key parsing.

use crate::helper::{CircleError, CircleResult};
use near_crypto::PublicKey;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::{
    action::{base64, delegate::DelegateAction},
    types::{AccountId, BlockReference, Finality},
};
use serde_json::json;
use std::str::FromStr;

use super::dto::{NearAccountBalance, NearNetwork, NearTokenBalance, NearTokenMetadata};

/// Convert yoctoNEAR (1e24) to NEAR string with proper precision
///
/// This function preserves precision by using integer arithmetic and formatting
/// the result as a decimal string. It handles the full 24 decimal places of yoctoNEAR.
fn format_yocto_to_near(yocto: u128) -> String {
    const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

    let whole = yocto / YOCTO_NEAR;
    let fractional = yocto % YOCTO_NEAR;

    if fractional == 0 {
        // No fractional part, return whole number
        whole.to_string()
    } else {
        // Format fractional part with leading zeros and trim trailing zeros
        let fractional_str = format!("{:024}", fractional);
        let trimmed = fractional_str.trim_end_matches('0');

        if trimmed.is_empty() {
            whole.to_string()
        } else {
            format!("{}.{}", whole, trimmed)
        }
    }
}

/// Get NEAR account balance by querying NEAR RPC
///
/// This function uses the official NEAR JSON-RPC client to query account balance information.
/// It returns the total, available, and staked balances for a given NEAR account.
///
/// # Arguments
/// * `account_id` - The NEAR account ID (e.g., "guest-book.testnet" or "example.near")
/// * `network` - The NEAR network to query (Mainnet or Testnet)
///
/// # Returns
/// * `CircleResult<NearAccountBalance>` - Account balance information on success
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::near::{get_near_account_balance, dto::NearNetwork};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let balance = get_near_account_balance("guest-book.testnet", NearNetwork::Testnet).await?;
/// println!("Total balance: {} NEAR", balance.total);
/// println!("Available: {} NEAR", balance.available);
/// println!("Staked: {} NEAR", balance.staked);
/// # Ok(())
/// # }
/// ```
pub async fn get_near_account_balance(
    account_id: &str,
    network: NearNetwork,
) -> CircleResult<NearAccountBalance> {
    let rpc_url = network.rpc_url();

    // Create JSON-RPC client
    let client = JsonRpcClient::connect(rpc_url);

    // Parse account ID
    let account_id = AccountId::from_str(account_id)
        .map_err(|e| CircleError::Config(format!("Invalid NEAR account ID: {}", e)))?;

    // Query account view
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: near_primitives::views::QueryRequest::ViewAccount { account_id },
    };

    let response = client.call(request).await.map_err(|e| CircleError::Api {
        status: 500,
        message: format!("NEAR RPC error: {}", e),
    })?;

    // Extract account view from response - RpcQueryResponse is a wrapper
    let (account_view, block_height, block_hash) = match response {
        methods::query::RpcQueryResponse {
            block_height,
            block_hash,
            kind,
        } => match kind {
            QueryResponseKind::ViewAccount(account_view) => {
                (account_view, Some(block_height), Some(block_hash))
            }
            _ => {
                return Err(CircleError::Api {
                    status: 500,
                    message: "Unexpected response type from NEAR RPC".to_string(),
                });
            }
        },
    };

    // Extract amounts (in yoctoNEAR)
    let amount = account_view.amount;
    let locked = account_view.locked;

    // Calculate available balance using integer arithmetic to preserve precision
    let available_yocto = amount.saturating_sub(locked);

    // Convert to NEAR strings with full precision
    let total = format_yocto_to_near(amount);
    let available = format_yocto_to_near(available_yocto);
    let staked = format_yocto_to_near(locked);

    Ok(NearAccountBalance {
        total,
        available,
        staked,
        state_hash: Some(account_view.code_hash.to_string()),
        block_hash,
        block_height,
    })
}

/// Serialize a NEAR DelegateAction to base64 for Circle API
///
/// This uses NEAR's official types and Borsh serialization.
/// According to NEP-461, delegate actions must be prefixed with
/// 2^30 + 461 = 1073742285 (0x400001CD) as a 4-byte little-endian u32.
///
/// # Arguments
/// * `delegate_action` - The NEAR DelegateAction to serialize
///
/// # Returns
/// * `std::io::Result<String>` - Base64-encoded prefixed delegate action
pub fn serialize_near_delegate_action_to_base64(
    delegate_action: &DelegateAction,
) -> std::io::Result<String> {
    // Serialize the delegate action to Borsh bytes
    let borsh_bytes = borsh::to_vec(delegate_action)?;

    // NEP-461 prefix for actionable messages (on-chain): 2^30 + 461 = 1073742285
    // This is encoded as a 4-byte little-endian u32
    const NEP_461_PREFIX: u32 = 0x40000000 + 461; // 1073741824 + 461 = 1073742285

    // Prepend the prefix to the serialized bytes
    let mut prefixed_bytes = Vec::with_capacity(4 + borsh_bytes.len());
    prefixed_bytes.extend_from_slice(&NEP_461_PREFIX.to_le_bytes());
    prefixed_bytes.extend_from_slice(&borsh_bytes);

    Ok(base64(&prefixed_bytes))
}

/// Get balance of a specific NEP-141 fungible token for an account
///
/// This function queries a specific token contract to get the balance
/// of a given account. The balance is returned as a string to preserve precision.
///
/// # Arguments
/// * `account_id` - The NEAR account ID to query
/// * `token_contract_id` - The token contract account ID (e.g., "usdc.fakes.testnet")
/// * `network` - The NEAR network to query (Mainnet or Testnet)
///
/// # Returns
/// * `CircleResult<String>` - Token balance as string (raw amount, not adjusted for decimals)
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::near::{get_near_token_balance, dto::NearNetwork};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let balance = get_near_token_balance(
///     "guest-book.testnet",
///     "usdc.fakes.testnet",
///     NearNetwork::Testnet
/// ).await?;
/// println!("USDC balance: {}", balance);
/// # Ok(())
/// # }
/// ```
pub async fn get_near_token_balance(
    account_id: &str,
    token_contract_id: &str,
    network: NearNetwork,
) -> CircleResult<String> {
    let rpc_url = network.rpc_url();
    let client = JsonRpcClient::connect(rpc_url);

    let account_id = AccountId::from_str(account_id)
        .map_err(|e| CircleError::Config(format!("Invalid NEAR account ID: {}", e)))?;
    let contract_id = AccountId::from_str(token_contract_id)
        .map_err(|e| CircleError::Config(format!("Invalid token contract ID: {}", e)))?;

    // Prepare arguments for ft_balance_of
    let args = json!({
        "account_id": account_id.as_str()
    });
    let args_bytes = serde_json::to_vec(&args).map_err(|e| CircleError::Json(e))?;

    // Query the token contract
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: near_primitives::views::QueryRequest::CallFunction {
            account_id: contract_id.clone(),
            method_name: "ft_balance_of".to_string(),
            args: args_bytes.into(),
        },
    };

    let response = client.call(request).await.map_err(|e| CircleError::Api {
        status: 500,
        message: format!("NEAR RPC error querying token balance: {}", e),
    })?;

    // Parse the response
    let result = match response {
        methods::query::RpcQueryResponse { kind, .. } => match kind {
            QueryResponseKind::CallResult(result) => {
                // The result is a JSON string containing the balance
                let result_str = String::from_utf8(result.result).map_err(|e| {
                    CircleError::Config(format!("Invalid UTF-8 in token balance response: {}", e))
                })?;

                // Parse the JSON to extract the balance
                let balance: String =
                    serde_json::from_str(&result_str).map_err(|e| CircleError::Json(e))?;

                balance
            }
            _ => {
                return Err(CircleError::Api {
                    status: 500,
                    message: "Unexpected response type from NEAR RPC".to_string(),
                });
            }
        },
    };

    Ok(result)
}

/// Get metadata for a NEP-141 fungible token
///
/// This function queries the token contract's `ft_metadata` method
/// to get token information like symbol, name, decimals, etc.
///
/// # Arguments
/// * `token_contract_id` - The token contract account ID
/// * `network` - The NEAR network to query
///
/// # Returns
/// * `CircleResult<NearTokenMetadata>` - Token metadata on success
pub async fn get_near_token_metadata(
    token_contract_id: &str,
    network: NearNetwork,
) -> CircleResult<NearTokenMetadata> {
    let rpc_url = network.rpc_url();
    let client = JsonRpcClient::connect(rpc_url);

    let contract_id = AccountId::from_str(token_contract_id)
        .map_err(|e| CircleError::Config(format!("Invalid token contract ID: {}", e)))?;

    // Query ft_metadata (no arguments needed)
    let request = methods::query::RpcQueryRequest {
        block_reference: BlockReference::Finality(Finality::Final),
        request: near_primitives::views::QueryRequest::CallFunction {
            account_id: contract_id.clone(),
            method_name: "ft_metadata".to_string(),
            args: vec![].into(),
        },
    };

    let response = client.call(request).await.map_err(|e| CircleError::Api {
        status: 500,
        message: format!("NEAR RPC error querying token metadata: {}", e),
    })?;

    // Parse the response
    let metadata = match response {
        methods::query::RpcQueryResponse { kind, .. } => match kind {
            QueryResponseKind::CallResult(result) => {
                let result_str = String::from_utf8(result.result).map_err(|e| {
                    CircleError::Config(format!("Invalid UTF-8 in token metadata response: {}", e))
                })?;

                // Parse the JSON metadata
                let metadata_json: serde_json::Value =
                    serde_json::from_str(&result_str).map_err(|e| CircleError::Json(e))?;

                NearTokenMetadata {
                    symbol: metadata_json["symbol"].as_str().unwrap_or("").to_string(),
                    name: metadata_json["name"].as_str().unwrap_or("").to_string(),
                    decimals: metadata_json["decimals"].as_u64().unwrap_or(0) as u8,
                    icon: metadata_json["icon"].as_str().map(|s| s.to_string()),
                    reference: metadata_json["reference"].as_str().map(|s| s.to_string()),
                }
            }
            _ => {
                return Err(CircleError::Api {
                    status: 500,
                    message: "Unexpected response type from NEAR RPC".to_string(),
                });
            }
        },
    };

    Ok(metadata)
}

/// Get balances for multiple NEP-141 fungible tokens
///
/// This function queries multiple token contracts to get balances for a given account.
/// Only tokens with non-zero balances are returned. Optionally fetches token metadata.
///
/// # Arguments
/// * `account_id` - The NEAR account ID to query
/// * `token_contracts` - List of token contract account IDs to check
/// * `network` - The NEAR network to query
/// * `include_metadata` - Whether to fetch token metadata (slower but more informative)
///
/// # Returns
/// * `CircleResult<Vec<NearTokenBalance>>` - List of token balances (only non-zero balances)
///
/// # Example
///
/// ```rust,no_run
/// use inf_circle_sdk::near::{get_near_token_balances, dto::NearNetwork};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let tokens = vec![
///     "usdc.fakes.testnet".to_string(),
///     "usdt.fakes.testnet".to_string(),
/// ];
/// let balances = get_near_token_balances(
///     "guest-book.testnet",
///     &tokens,
///     NearNetwork::Testnet,
///     true, // include metadata
/// ).await?;
///
/// for balance in balances {
///     println!("{}: {}", balance.contract_id, balance.balance);
/// }
/// # Ok(())
/// # }
/// ```
pub async fn get_near_token_balances(
    account_id: &str,
    token_contracts: &[String],
    network: NearNetwork,
    include_metadata: bool,
) -> CircleResult<Vec<NearTokenBalance>> {
    let mut balances = Vec::new();

    for contract_id in token_contracts {
        // Query balance
        match get_near_token_balance(account_id, contract_id, network).await {
            Ok(balance_str) => {
                // Parse balance as u128 to check if it's zero
                let balance_u128: u128 = balance_str.parse().unwrap_or(0);

                // Only include non-zero balances
                if balance_u128 > 0 {
                    let metadata = if include_metadata {
                        match get_near_token_metadata(contract_id, network).await {
                            Ok(meta) => Some(meta),
                            Err(_) => None, // Continue even if metadata fetch fails
                        }
                    } else {
                        None
                    };

                    balances.push(NearTokenBalance {
                        contract_id: contract_id.clone(),
                        balance: balance_str,
                        metadata,
                    });
                }
            }
            Err(e) => {
                // Log error but continue with other tokens
                eprintln!(
                    "Warning: Failed to query balance for {}: {}",
                    contract_id, e
                );
            }
        }
    }

    Ok(balances)
}

/// Parse a NEAR public key from various formats
///
/// Supports:
/// - "ed25519:base58..." (NEAR standard)
/// - "base58..." (Circle API format, assumes ED25519)
///
/// # Arguments
/// * `s` - The public key string to parse
///
/// # Returns
/// * `Result<PublicKey, String>` - Parsed public key or error message
pub fn parse_near_public_key(s: &str) -> Result<PublicKey, String> {
    // Try with prefix first
    if let Ok(pk) = near_crypto::PublicKey::from_str(s) {
        return Ok(pk);
    }

    // Try adding ed25519: prefix (Circle format)
    let with_prefix = format!("ed25519:{}", s);
    PublicKey::from_str(&with_prefix).map_err(|e| format!("Failed to parse NEAR public key: {}", e))
}

#[cfg(test)]
mod tests {
    use super::format_yocto_to_near;

    #[test]
    fn test_format_yocto_to_near_whole_number() {
        const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
        assert_eq!(format_yocto_to_near(0), "0");
        assert_eq!(format_yocto_to_near(YOCTO_NEAR), "1");
        assert_eq!(format_yocto_to_near(5 * YOCTO_NEAR), "5");
    }

    #[test]
    fn test_format_yocto_to_near_with_fractional() {
        const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
        // 1.5 NEAR
        assert_eq!(format_yocto_to_near(YOCTO_NEAR + YOCTO_NEAR / 2), "1.5");
        // 0.000001 NEAR (1 yoctoNEAR * 1e18)
        assert_eq!(format_yocto_to_near(1_000_000_000_000_000_000), "0.000001");
        // Very small amount
        assert_eq!(format_yocto_to_near(1), "0.000000000000000000000001");
    }

    #[test]
    fn test_format_yocto_to_near_precision_preservation() {
        // Test that we preserve all significant digits
        let test_amount = 1234567890123456789012345u128; // 1.234567890123456789012345 NEAR
        let result = format_yocto_to_near(test_amount);
        assert!(result.starts_with("1.234567890123456789012345"));
    }

    #[test]
    fn test_format_yocto_to_near_trim_trailing_zeros() {
        const YOCTO_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
        // 1.1 NEAR should not have trailing zeros
        let amount = YOCTO_NEAR + YOCTO_NEAR / 10;
        let result = format_yocto_to_near(amount);
        assert_eq!(result, "1.1");
        assert!(!result.ends_with('0'));
    }
}
