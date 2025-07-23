#!/usr/bin/env rust-script
//! # GridTokenX Cryptographic Key Example
//!
//! This example demonstrates the key concepts of private keys, public keys, and AccountIds
//! in the GridTokenX blockchain system.

use thai_energy_trading_blockchain::{
    crypto::{GridTokenXKeyPair, derive_account_id, validate_account_id},
    utils::crypto::{validate_account_id as utils_validate_account_id, generate_test_account_id},
    SystemResult
};

#[tokio::main]
async fn main() -> SystemResult<()> {
    println!("🔐 GridTokenX Cryptographic Key System Demo");
    println!("==========================================");
    
    // 1. Private Key Concept
    println!("\n1️⃣ Private Key - The Master Secret");
    println!("----------------------------------");
    println!("💡 The private key is a 256-bit (32-byte) secret number that:");
    println!("   • Must NEVER be shared with anyone");
    println!("   • Provides complete control over the account");
    println!("   • Is used to create digital signatures");
    println!("   • Should be stored encrypted and backed up securely");
    
    // Generate a new keypair
    let alice_keypair = GridTokenXKeyPair::generate()?;
    let private_key_hex = hex::encode(alice_keypair.export_private_key_bytes());
    
    println!("\n🔑 Alice's Private Key (NEVER SHARE IN REAL WORLD!):");
    println!("   {}", private_key_hex);
    println!("   Length: {} characters (32 bytes)", private_key_hex.len());
    
    // 2. Public Key Derivation
    println!("\n2️⃣ Public Key - Mathematically Derived, Safe to Share");
    println!("----------------------------------------------------");
    println!("💡 The public key is derived from the private key and:");
    println!("   • Can be shared freely with everyone");
    println!("   • Used to verify signatures created by the private key");
    println!("   • Cannot be used to recreate the private key");
    println!("   • Forms the basis for the AccountId");
    
    let public_key_hex = hex::encode(alice_keypair.export_public_key_bytes());
    println!("\n🔓 Alice's Public Key (Safe to Share):");
    println!("   {}", public_key_hex);
    println!("   Length: {} characters (32 bytes)", public_key_hex.len());
    
    // 3. AccountId Generation
    println!("\n3️⃣ AccountId - The Blockchain Address");
    println!("-------------------------------------");
    println!("💡 The AccountId is derived from the public key and:");
    println!("   • Serves as the blockchain address for receiving funds");
    println!("   • Is human-readable with 'gx_' prefix");
    println!("   • Is deterministic (same public key = same AccountId)");
    println!("   • Is unique across all users");
    
    println!("\n🏠 Alice's AccountId (Blockchain Address):");
    println!("   {}", alice_keypair.account_id());
    println!("   Format: gx_[32 hexadecimal characters]");
    
    // 4. Demonstrate Deterministic Derivation
    println!("\n4️⃣ Deterministic Derivation Demo");
    println!("-------------------------------");
    println!("💡 The same private key always produces the same public key and AccountId:");
    
    // Import the same private key again
    let alice_keypair2 = GridTokenXKeyPair::from_private_key_bytes(
        &alice_keypair.export_private_key_bytes()
    )?;
    
    println!("\n🔄 Recreating keypair from same private key:");
    println!("   Original AccountId:  {}", alice_keypair.account_id());
    println!("   Recreated AccountId: {}", alice_keypair2.account_id());
    println!("   Are they identical? {}", 
        if alice_keypair.account_id() == alice_keypair2.account_id() { "✅ Yes" } else { "❌ No" }
    );
    
    // 5. AccountId Validation
    println!("\n5️⃣ AccountId Validation");
    println!("----------------------");
    println!("💡 AccountIds have a specific format that must be validated:");
    
    let test_account_ids = vec![
        alice_keypair.account_id().to_string(), // Valid
        "gx_a1b2c3d4e5f67890123456789012345".to_string(), // Valid format but fake
        "invalid_account".to_string(), // Invalid - no gx_ prefix
        "gx_".to_string(), // Invalid - too short
        "gx_invalid_hex_zzz123456789012345".to_string(), // Invalid - bad hex
        generate_test_account_id(), // Valid test AccountId
    ];
    
    for account_id in &test_account_ids {
        let is_valid = utils_validate_account_id(account_id).unwrap_or(false);
        println!("   {}: {}", 
            account_id, 
            if is_valid { "✅ Valid format" } else { "❌ Invalid format" }
        );
    }
    
    // 6. Multiple Users Demo
    println!("\n6️⃣ Multiple Users - Uniqueness Guarantee");
    println!("---------------------------------------");
    println!("💡 Each user gets a unique AccountId:");
    
    let bob_keypair = GridTokenXKeyPair::generate()?;
    let charlie_keypair = GridTokenXKeyPair::generate()?;
    
    println!("\n👩 Alice:   {}", alice_keypair.account_id());
    println!("👨 Bob:     {}", bob_keypair.account_id());
    println!("👤 Charlie: {}", charlie_keypair.account_id());
    
    let all_different = alice_keypair.account_id() != bob_keypair.account_id() &&
                       bob_keypair.account_id() != charlie_keypair.account_id() &&
                       alice_keypair.account_id() != charlie_keypair.account_id();
    
    println!("   All AccountIds are unique: {}", 
        if all_different { "✅ Yes" } else { "❌ No" }
    );
    
    // 7. Transaction Signing Demo
    println!("\n7️⃣ Transaction Signing - Proving Ownership");
    println!("-----------------------------------------");
    println!("💡 Private keys are used to sign transactions, proving ownership:");
    
    use serde::Serialize;
    
    #[derive(Serialize)]
    struct SimpleTransaction {
        from: String,
        to: String,
        amount: u64,
        energy_kwh: f64,
    }
    
    let transaction = SimpleTransaction {
        from: alice_keypair.account_id().to_string(),
        to: bob_keypair.account_id().to_string(),
        amount: 12500, // 125.00 THB in cents
        energy_kwh: 10.0,
    };
    
    let signature = alice_keypair.sign_transaction(&transaction)?;
    
    println!("\n📝 Alice signs a transaction to send 10 kWh to Bob:");
    println!("   From: {}", transaction.from);
    println!("   To: {}", transaction.to);
    println!("   Amount: {} THB cents ({} THB)", transaction.amount, transaction.amount as f64 / 100.0);
    println!("   Energy: {} kWh", transaction.energy_kwh);
    println!("   Signature: {}...", hex::encode(&signature.signature)[..32]);
    println!("   Signer: {}", signature.account_id);
    println!("   Timestamp: {}", signature.timestamp);
    
    // 8. Security Summary
    println!("\n8️⃣ Security Summary");
    println!("-----------------");
    println!("🔒 Security Features:");
    println!("   ✅ Private keys use cryptographically secure random generation");
    println!("   ✅ Ed25519 elliptic curve provides 128-bit security level");
    println!("   ✅ AccountIds are deterministically derived from public keys");
    println!("   ✅ Signatures prove ownership without revealing private keys");
    println!("   ✅ Each AccountId is cryptographically unique");
    println!("   ✅ Transaction integrity verified through digital signatures");
    
    println!("\n🚨 Security Warnings:");
    println!("   ⚠️  NEVER share your private key with anyone");
    println!("   ⚠️  Store private keys encrypted and backed up");
    println!("   ⚠️  Use hardware wallets for large amounts");
    println!("   ⚠️  Always verify AccountIds before sending transactions");
    
    println!("\n✅ Demo completed successfully!");
    println!("🎓 You now understand the GridTokenX cryptographic key system!");
    
    Ok(())
}
