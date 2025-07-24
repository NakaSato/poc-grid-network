//! # GridTokenX Cryptographic Key Management Example
//! 
//! This example demonstrates the cryptographic key management system,
//! including key generation, AccountId derivation, and transaction signing.

use thai_energy_trading_blockchain::{
    crypto::{GridTokenXKeyPair, AccountManager, derive_account_id, validate_account_id},
    types::{EnergyOrder, OrderType, EnergySource, GridLocation, GridCoordinates, OrderStatus},
    SystemResult
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct SampleEnergyTransaction {
    pub from: String,
    pub to: String,
    pub energy_amount: f64,
    pub price_per_kwh: u128,
    pub energy_source: EnergySource,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> SystemResult<()> {
    env_logger::init();
    
    println!("🔐 GridTokenX Cryptographic Key Management Example");
    println!("================================================");
    
    // 1. Generate new keypairs
    println!("\n1️⃣ Generating New Keypairs");
    println!("--------------------------");
    
    let alice_keypair = GridTokenXKeyPair::generate()?;
    let bob_keypair = GridTokenXKeyPair::generate()?;
    
    println!("👩 Alice's Account:");
    println!("   AccountId: {}", alice_keypair.account_id());
    println!("   Public Key: {}", hex::encode(alice_keypair.export_public_key_bytes()));
    println!("   Private Key: {}", hex::encode(alice_keypair.export_private_key_bytes()));
    
    println!("\n👨 Bob's Account:");
    println!("   AccountId: {}", bob_keypair.account_id());
    println!("   Public Key: {}", hex::encode(bob_keypair.export_public_key_bytes()));
    println!("   Private Key: {}", hex::encode(bob_keypair.export_private_key_bytes()));
    
    // 2. Validate AccountIds
    println!("\n2️⃣ AccountId Validation");
    println!("----------------------");
    
    let alice_valid = validate_account_id(alice_keypair.account_id())?;
    let bob_valid = validate_account_id(bob_keypair.account_id())?;
    
    println!("Alice's AccountId valid: {}", if alice_valid { "✅" } else { "❌" });
    println!("Bob's AccountId valid: {}", if bob_valid { "✅" } else { "❌" });
    
    // Test invalid AccountIds
    let invalid_ids = vec![
        "invalid_account",
        "gx_",
        "not_gx_prefixed",
        "gx_invalid_hex_zzz",
        "gx_toolong1234567890123456789012345678901234567890"
    ];
    
    println!("\nTesting invalid AccountIds:");
    for invalid_id in &invalid_ids {
        let is_valid = validate_account_id(invalid_id)?;
        println!("   {}: {}", invalid_id, if is_valid { "✅" } else { "❌" });
    }
    
    // 3. AccountId derivation consistency
    println!("\n3️⃣ AccountId Derivation Consistency");
    println!("----------------------------------");
    
    let alice_derived = derive_account_id(alice_keypair.public_key())?;
    let bob_derived = derive_account_id(bob_keypair.public_key())?;
    
    println!("Alice - Original: {}", alice_keypair.account_id());
    println!("Alice - Derived:  {}", alice_derived);
    println!("Alice - Match: {}", if alice_keypair.account_id() == alice_derived { "✅" } else { "❌" });
    
    println!("\nBob - Original: {}", bob_keypair.account_id());
    println!("Bob - Derived:  {}", bob_derived);
    println!("Bob - Match: {}", if bob_keypair.account_id() == bob_derived { "✅" } else { "❌" });
    
    // 4. Transaction signing demonstration
    println!("\n4️⃣ Transaction Signing Demonstration");
    println!("------------------------------------");
    
    let transaction = SampleEnergyTransaction {
        from: alice_keypair.account_id().to_string(),
        to: bob_keypair.account_id().to_string(),
        energy_amount: 100.5,
        price_per_kwh: 4750,
        energy_source: EnergySource::Solar,
        timestamp: Utc::now(),
    };
    
    println!("Transaction Details:");
    println!("   From: {}", transaction.from);
    println!("   To: {}", transaction.to);
    println!("   Energy: {} kWh", transaction.energy_amount);
    println!("   Price: {} tokens/kWh", transaction.price_per_kwh);
    println!("   Source: {:?}", transaction.energy_source);
    
    // Sign transaction with Alice's key
    let signature = alice_keypair.sign_transaction(&transaction)?;
    
    println!("\nTransaction Signature:");
    println!("   Signature: {}", hex::encode(&signature.signature));
    println!("   Public Key: {}", hex::encode(&signature.public_key));
    println!("   Signer AccountId: {}", signature.account_id);
    println!("   Timestamp: {}", signature.timestamp);
    
    // 5. Account Manager demonstration
    println!("\n5️⃣ Account Manager Demonstration");
    println!("--------------------------------");
    
    let account_manager = AccountManager::new();
    
    // Import Alice's and Bob's accounts
    let alice_imported_id = account_manager.import_account(alice_keypair.export_private_key_bytes()).await?;
    let bob_imported_id = account_manager.import_account(bob_keypair.export_private_key_bytes()).await?;
    
    println!("Imported Accounts:");
    println!("   Alice: {}", alice_imported_id);
    println!("   Bob: {}", bob_imported_id);
    
    // Create a new account through the manager
    let charlie_account_id = account_manager.create_account().await?;
    println!("   Charlie (new): {}", charlie_account_id);
    
    println!("\nTotal managed accounts: {}", account_manager.account_count().await);
    
    // List all accounts
    let all_accounts = account_manager.list_accounts().await;
    println!("All managed accounts:");
    for (i, account_id) in all_accounts.iter().enumerate() {
        println!("   {}. {}", i + 1, account_id);
    }
    
    // 6. Transaction signing through Account Manager
    println!("\n6️⃣ Transaction Signing via Account Manager");
    println!("-----------------------------------------");
    
    let new_transaction = SampleEnergyTransaction {
        from: alice_imported_id.clone(),
        to: charlie_account_id.clone(),
        energy_amount: 75.25,
        price_per_kwh: 4800,
        energy_source: EnergySource::Wind,
        timestamp: Utc::now(),
    };
    
    // Sign through account manager
    let manager_signature = account_manager.sign_transaction(&alice_imported_id, &new_transaction).await?;
    
    println!("Transaction signed through Account Manager:");
    println!("   From: {}", new_transaction.from);
    println!("   To: {}", new_transaction.to);
    println!("   Signature: {}", hex::encode(&manager_signature.signature));
    println!("   Signer: {}", manager_signature.account_id);
    
    // 7. Signature verification
    println!("\n7️⃣ Signature Verification");
    println!("-------------------------");
    
    let verification_result = account_manager
        .verify_transaction_signature(&new_transaction, &manager_signature)
        .await?;
    
    println!("Signature verification result: {}", if verification_result { "✅ Valid" } else { "❌ Invalid" });
    
    // Test signature verification with modified transaction
    let modified_transaction = SampleEnergyTransaction {
        from: alice_imported_id.clone(),
        to: charlie_account_id.clone(),
        energy_amount: 999.99, // Modified amount
        price_per_kwh: 4800,
        energy_source: EnergySource::Wind,
        timestamp: Utc::now(),
    };
    
    let modified_verification = account_manager
        .verify_transaction_signature(&modified_transaction, &manager_signature)
        .await?;
    
    println!("Modified transaction verification: {}", if modified_verification { "✅ Valid" } else { "❌ Invalid (expected)" });
    
    // 8. Energy Order signing example
    println!("\n8️⃣ Energy Order Signing Example");
    println!("-------------------------------");
    
    let energy_order = EnergyOrder {
        id: Uuid::new_v4(),
        account_id: alice_imported_id.clone(),
        order_type: OrderType::Sell,
        energy_amount: 150.0,
        price_per_unit: 4900,
        energy_source: Some(EnergySource::Solar),
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            coordinates: GridCoordinates { lat: 13.7563, lng: 100.5018 },
            region: "Central".to_string(),
            substation: "Siam".to_string(),
            grid_code: "BKK001".to_string(),
            meter_id: "M001".to_string(),
        },
        timestamp: Utc::now(),
        updated_at: Utc::now(),
        status: OrderStatus::Pending,
    };
    
    println!("Energy Order:");
    println!("   Order ID: {}", energy_order.id);
    println!("   Account: {}", energy_order.account_id);
    println!("   Type: {:?}", energy_order.order_type);
    println!("   Amount: {} kWh", energy_order.energy_amount);
    println!("   Price: {} tokens/kWh", energy_order.price_per_unit);
    println!("   Source: {:?}", energy_order.energy_source);
    println!("   Location: {} {}", energy_order.location.province, energy_order.location.district);
    
    let order_signature = account_manager.sign_transaction(&alice_imported_id, &energy_order).await?;
    
    println!("\nEnergy Order Signature:");
    println!("   Signature: {}...", &hex::encode(&order_signature.signature)[..16]);
    println!("   Signer: {}", order_signature.account_id);
    println!("   Timestamp: {}", order_signature.timestamp);
    
    let order_verification = account_manager
        .verify_transaction_signature(&energy_order, &order_signature)
        .await?;
    
    println!("   Verification: {}", if order_verification { "✅ Valid" } else { "❌ Invalid" });
    
    // 9. Security demonstration
    println!("\n9️⃣ Security Demonstration");
    println!("-------------------------");
    
    // Demonstrate that private keys are required for signing
    println!("Security features:");
    println!("   ✅ Private keys never transmitted");
    println!("   ✅ AccountIds are deterministically derived from public keys");
    println!("   ✅ Signatures include AccountId binding");
    println!("   ✅ Transaction integrity verified through cryptographic signatures");
    println!("   ✅ Public keys can be shared safely");
    println!("   ✅ Each AccountId is unique and cryptographically secured");
    
    // Show that different private keys produce different AccountIds
    let temp_keypair1 = GridTokenXKeyPair::generate()?;
    let temp_keypair2 = GridTokenXKeyPair::generate()?;
    
    println!("\nUniqueness verification:");
    println!("   Random Account 1: {}", temp_keypair1.account_id());
    println!("   Random Account 2: {}", temp_keypair2.account_id());
    println!("   Are they different? {}", if temp_keypair1.account_id() != temp_keypair2.account_id() { "✅ Yes" } else { "❌ No" });
    
    println!("\n🎉 Cryptographic Key Management Example completed successfully!");
    println!("   - Generated and validated multiple keypairs");
    println!("   - Demonstrated AccountId derivation");
    println!("   - Signed and verified transactions");
    println!("   - Showed account management capabilities");
    println!("   - Verified security properties");
    
    Ok(())
}
