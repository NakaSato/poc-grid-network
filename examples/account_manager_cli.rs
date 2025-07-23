#!/usr/bin/env rust-script
//! # GridTokenX Account and Key Management CLI
//!
//! Interactive command-line tool for demonstrating and working with
//! GridTokenX cryptographic keys and AccountIds.

use thai_energy_trading_blockchain::{
    crypto::{GridTokenXKeyPair, derive_account_id, validate_account_id, verify_account_id_matches_public_key},
    utils::crypto::{validate_account_id as utils_validate, generate_test_account_id, hash_sha256_hex},
    SystemResult
};
use ed25519_dalek::VerifyingKey;
use clap::{App, Arg, SubCommand};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> SystemResult<()> {
    let matches = App::new("GridTokenX Account Manager")
        .version("1.0.0")
        .author("GridTokenX Team")
        .about("Interactive tool for managing GridTokenX accounts and keys")
        .subcommand(SubCommand::with_name("interactive")
            .about("Start interactive mode"))
        .subcommand(SubCommand::with_name("generate")
            .about("Generate a new account keypair"))
        .subcommand(SubCommand::with_name("validate")
            .about("Validate an AccountId")
            .arg(Arg::with_name("account_id")
                .help("AccountId to validate")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("derive")
            .about("Derive AccountId from public key")
            .arg(Arg::with_name("public_key")
                .help("Public key in hex format")
                .required(true)
                .index(1)))
        .get_matches();

    match matches.subcommand() {
        ("interactive", Some(_)) => run_interactive_mode().await,
        ("generate", Some(_)) => generate_keypair().await,
        ("validate", Some(sub_m)) => {
            let account_id = sub_m.value_of("account_id").unwrap();
            validate_account_id_command(account_id).await
        },
        ("derive", Some(sub_m)) => {
            let public_key = sub_m.value_of("public_key").unwrap();
            derive_account_id_command(public_key).await
        },
        _ => run_interactive_mode().await,
    }
}

async fn run_interactive_mode() -> SystemResult<()> {
    println!("🔐 GridTokenX Account Manager - Interactive Mode");
    println!("==============================================");
    println!("Available commands:");
    println!("  1. generate    - Generate new keypair");
    println!("  2. validate    - Validate AccountId format");
    println!("  3. derive      - Derive AccountId from public key");
    println!("  4. demo        - Run key concepts demo");
    println!("  5. test        - Generate test AccountIds");
    println!("  6. help        - Show this help");
    println!("  7. quit        - Exit program");
    
    loop {
        print!("\n🔑 Enter command (1-7 or name): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();
        
        match command.as_str() {
            "1" | "generate" | "gen" => {
                generate_keypair().await?;
            },
            "2" | "validate" | "val" => {
                println!("Enter AccountId to validate:");
                print!("AccountId: ");
                io::stdout().flush().unwrap();
                let mut account_id = String::new();
                io::stdin().read_line(&mut account_id).unwrap();
                validate_account_id_command(account_id.trim()).await?;
            },
            "3" | "derive" | "der" => {
                println!("Enter public key (64 hex characters):");
                print!("Public Key: ");
                io::stdout().flush().unwrap();
                let mut public_key = String::new();
                io::stdin().read_line(&mut public_key).unwrap();
                derive_account_id_command(public_key.trim()).await?;
            },
            "4" | "demo" => {
                run_key_concepts_demo().await?;
            },
            "5" | "test" => {
                generate_test_accounts().await?;
            },
            "6" | "help" | "h" => {
                print_help();
            },
            "7" | "quit" | "exit" | "q" => {
                println!("👋 Goodbye!");
                break;
            },
            "" => {
                // Empty input, continue
            },
            _ => {
                println!("❌ Unknown command: '{}'. Type 'help' for available commands.", command);
            }
        }
    }
    
    Ok(())
}

async fn generate_keypair() -> SystemResult<()> {
    println!("\n🔑 Generating New GridTokenX Keypair");
    println!("===================================");
    
    let keypair = GridTokenXKeyPair::generate()?;
    let private_key_hex = hex::encode(keypair.export_private_key_bytes());
    let public_key_hex = hex::encode(keypair.export_public_key_bytes());
    
    println!("✅ New keypair generated successfully!");
    println!("\n📋 Account Information:");
    println!("   AccountId:   {}", keypair.account_id());
    let validation_result = utils_validate(keypair.account_id()).unwrap_or(false);
    println!("   Validation:  {}", if validation_result { "Valid format" } else { "Invalid format" });
    println!("   Public Key:  {}", public_key_hex);
    println!("   Private Key: {}", private_key_hex);
    
    println!("\n🔒 Security Information:");
    println!("   • AccountId and Public Key can be shared safely");
    println!("   • Private Key must be kept SECRET - never share it!");
    println!("   • Store Private Key encrypted and backed up securely");
    
    // Show derivation validation
    let validation_result = utils_validate(keypair.account_id()).unwrap_or(false);
    println!("\n✅ AccountId validation: {}", 
        if validation_result { "Valid ✓" } else { "Invalid ✗" });
    
    Ok(())
}

async fn validate_account_id_command(account_id: &str) -> SystemResult<()> {
    println!("\n🔍 Validating AccountId");
    println!("======================");
    println!("AccountId: {}", account_id);
    
    match utils_validate(account_id) {
        Ok(true) => {
            println!("✅ Valid AccountId format!");
            println!("\n📋 Format Details:");
            println!("   • Length: {} characters", account_id.len());
            println!("   • Prefix: 'gx_' ✓");
            println!("   • Hex part: '{}' ✓", &account_id[3..]);
            println!("   • Hex length: {} characters (16 bytes) ✓", account_id[3..].len());
        },
        Ok(false) => {
            println!("❌ Invalid AccountId format!");
            println!("\n🔍 Checking format requirements:");
            
            // Detailed validation feedback
            if account_id.len() != 35 {
                println!("   • Length: {} characters (should be 35) ✗", account_id.len());
            } else {
                println!("   • Length: {} characters ✓", account_id.len());
            }
            
            if !account_id.starts_with("gx_") {
                println!("   • Prefix: '{}' (should be 'gx_') ✗", &account_id[..3.min(account_id.len())]);
            } else {
                println!("   • Prefix: 'gx_' ✓");
            }
            
            if account_id.len() > 3 {
                let hex_part = &account_id[3..];
                if hex_part.len() != 32 {
                    println!("   • Hex part length: {} characters (should be 32) ✗", hex_part.len());
                } else if hex::decode(hex_part).is_err() {
                    println!("   • Hex encoding: Invalid characters ✗");
                } else {
                    println!("   • Hex part: Valid ✓");
                }
            }
        },
        Err(e) => {
            println!("❌ Error validating AccountId: {}", e);
        }
    }
    
    Ok(())
}

async fn derive_account_id_command(public_key_hex: &str) -> SystemResult<()> {
    println!("\n🔄 Deriving AccountId from Public Key");
    println!("====================================");
    println!("Public Key: {}", public_key_hex);
    
    // Validate hex format
    if public_key_hex.len() != 64 {
        println!("❌ Invalid public key length: {} characters (expected 64)", public_key_hex.len());
        return Ok(());
    }
    
    let public_key_bytes = match hex::decode(public_key_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("❌ Invalid hex format: {}", e);
            return Ok(());
        }
    };
    
    if public_key_bytes.len() != 32 {
        println!("❌ Invalid public key length: {} bytes (expected 32)", public_key_bytes.len());
        return Ok(());
    }
    
    let public_key = match VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap()) {
        Ok(pk) => pk,
        Err(e) => {
            println!("❌ Invalid public key: {}", e);
            return Ok(());
        }
    };
    
    let account_id = derive_account_id(&public_key)?;
    
    println!("✅ AccountId derived successfully!");
    println!("\n📋 Results:");
    println!("   Input Public Key: {}", public_key_hex);
    println!("   Derived AccountId: {}", account_id);
    println!("   AccountId Hash: {}", hash_sha256_hex(public_key_hex.as_bytes()));
    
    // Validate the derived AccountId
    let is_valid = utils_validate(&account_id).unwrap_or(false);
    println!("   Validation: {}", if is_valid { "Valid ✓" } else { "Invalid ✗" });
    
    Ok(())
}

async fn run_key_concepts_demo() -> SystemResult<()> {
    println!("\n🎓 Key Concepts Demo");
    println!("===================");
    
    // Generate example keypair
    let keypair = GridTokenXKeyPair::generate()?;
    
    println!("1. Private Key (Secret, 32 bytes):");
    println!("   {}", hex::encode(keypair.export_private_key_bytes()));
    
    println!("\n2. Public Key (Derived from private, 32 bytes):");
    println!("   {}", hex::encode(keypair.export_public_key_bytes()));
    
    println!("\n3. AccountId (Derived from public key):");
    println!("   {}", keypair.account_id());
    
    println!("\n🔍 Relationship:");
    println!("   Private Key → Public Key → AccountId");
    println!("   (secret)    → (shareable) → (blockchain address)");
    
    // Show deterministic property
    let keypair2 = GridTokenXKeyPair::from_private_key_bytes(
        &keypair.export_private_key_bytes()
    )?;
    
    println!("\n🔄 Deterministic Property:");
    println!("   Same private key always produces same AccountId");
    println!("   Original:  {}", keypair.account_id());
    println!("   Recreated: {}", keypair2.account_id());
    println!("   Match: {}", if keypair.account_id() == keypair2.account_id() { "✅" } else { "❌" });
    
    Ok(())
}

async fn generate_test_accounts() -> SystemResult<()> {
    println!("\n🧪 Generating Test Accounts");
    println!("==========================");
    
    for i in 1..=5 {
        let test_account_id = generate_test_account_id();
        let is_valid = utils_validate(&test_account_id).unwrap_or(false);
        println!("{}. {} - {}", 
            i, 
            test_account_id, 
            if is_valid { "✅ Valid" } else { "❌ Invalid" }
        );
    }
    
    println!("\nℹ️  These are randomly generated test AccountIds for development purposes.");
    println!("   They don't correspond to real private keys or accounts.");
    
    Ok(())
}

fn print_help() {
    println!("\n📚 GridTokenX Account Manager Help");
    println!("=================================");
    println!("Commands:");
    println!("  generate   - Create a new cryptographic keypair");
    println!("  validate   - Check if an AccountId has valid format");
    println!("  derive     - Calculate AccountId from a public key");
    println!("  demo       - Show key concepts demonstration");
    println!("  test       - Generate sample test AccountIds");
    println!("  help       - Display this help message");
    println!("  quit       - Exit the program");
    
    println!("\n🔐 Key Concepts:");
    println!("  Private Key - Secret 32-byte number, never share!");
    println!("  Public Key  - Derived from private key, safe to share");
    println!("  AccountId   - Blockchain address derived from public key");
    
    println!("\n📝 AccountId Format:");
    println!("  Format: gx_[32 hexadecimal characters]");
    println!("  Example: gx_a1b2c3d4e5f6789012345678901234ab");
    println!("  Total Length: 35 characters");
}
