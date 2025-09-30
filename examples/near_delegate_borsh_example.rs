//! Example demonstrating NEAR delegate action Borsh serialization
//!
//! This example shows how to properly serialize a NEAR delegate action
//! using Borsh format (Binary Object Representation Serializer for Hashing)
//! instead of JSON, which is the correct format for NEAR protocol.

use inf_circle_sdk::helper::{parse_near_public_key, serialize_near_delegate_action_to_base64};
use near_primitives::{
    action::{delegate::DelegateAction, delegate::NonDelegateAction, Action, FunctionCallAction},
    types::AccountId,
};

fn main() {
    println!("🔧 Creating a NEAR Delegate Action with Borsh serialization\n");

    // Create a function call action
    let args_json = r#"{"text":"Hello from Circle SDK!"}"#;

    let function_call = FunctionCallAction {
        method_name: "addMessage".to_string(),
        args: args_json.as_bytes().to_vec(),
        gas: 100_000_000_000_000, // 100 TGas
        deposit: 0,
    };

    println!("📋 Function Call Details:");
    println!("   Method: {}", function_call.method_name);
    println!("   Args: {}", args_json);
    println!("   Gas: {} (100 TGas)", function_call.gas);
    println!("   Deposit: {}\n", function_call.deposit);

    // Create a delegate action using NEAR's official types
    let public_key_str = "ed25519:DcA2MzgpJbrUATQLLceocVckhhAqrkingax4oJ9kZ847";
    let public_key = parse_near_public_key(public_key_str).expect("Failed to parse public key");

    // Convert Action to NonDelegateAction
    let action = Action::FunctionCall(Box::new(function_call));
    let non_delegate_action = NonDelegateAction::try_from(action).expect("Failed to convert");

    let delegate_action = DelegateAction {
        sender_id: AccountId::try_from("test.sender.near".to_string()).unwrap(),
        receiver_id: AccountId::try_from("guest-book.testnet".to_string()).unwrap(),
        actions: vec![non_delegate_action],
        nonce: 1u64,
        max_block_height: 1_000_000u64,
        public_key,
    };

    println!("🎯 Delegate Action Details:");
    println!("   Sender: {}", delegate_action.sender_id);
    println!("   Receiver: {}", delegate_action.receiver_id);
    println!("   Nonce: {}", delegate_action.nonce);
    println!(
        "   Max Block Height: {}\n",
        delegate_action.max_block_height
    );

    // Serialize to Borsh and base64 encode
    match serialize_near_delegate_action_to_base64(&delegate_action) {
        Ok(base64_str) => {
            let borsh_bytes = borsh::to_vec(&delegate_action).unwrap();
            println!("✅ Borsh Serialization Successful!");
            println!("   Byte length: {} bytes\n", borsh_bytes.len());
            println!("✅ Base64 Encoding Successful!");
            println!(
                "   Base64 string (first 100 chars): {}",
                if base64_str.len() > 100 {
                    &base64_str[..100]
                } else {
                    &base64_str
                }
            );
            println!("   Full length: {} characters\n", base64_str.len());
            println!("📦 This base64 string is ready to be sent to Circle's API");
        }
        Err(e) => {
            eprintln!("❌ Serialization failed: {}", e);
        }
    }

    println!("\n💡 Key Points:");
    println!("   • NEAR delegate actions MUST be serialized with Borsh, not JSON");
    println!("   • Borsh is NEAR's binary serialization format");
    println!("   • The serialized bytes are then base64-encoded for API transmission");
    println!("   • JSON serialization will NOT work with NEAR's protocol");
}
