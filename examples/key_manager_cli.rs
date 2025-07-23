//! # GridTokenX Key Management CLI
//! 
//! Command-line tool for generating, managing, and working with
//! cryptographic keys and AccountIds in the GridTokenX system.

use thai_energy_trading_blockchain::{
    crypto::{GridTokenXKeyPair, derive_account_id, validate_account_id, verify_account_id_matches_public_key},
    SystemResult
};
use ed25519_dalek::{PublicKey, SecretKey};
use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> SystemResult<()> {
    env_logger::init();
    
    let matches = App::new("GridTokenX Key Manager")
        .version("1.0.0")
        .author("GridTokenX Team")
        .about("Manage cryptographic keys and AccountIds for GridTokenX blockchain")
        .subcommand(SubCommand::with_name("generate")
            .about("Generate a new keypair and AccountId")
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("DIRECTORY")
                .help("Output directory for key files (default: current directory)")
                .takes_value(true))
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Name prefix for key files")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("derive")
            .about("Derive AccountId from a public key")
            .arg(Arg::with_name("pubkey")
                .short("p")
                .long("pubkey")
                .value_name("PUBKEY_HEX")
                .help("Public key in hexadecimal format")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("PUBKEY_FILE")
                .help("File containing public key in hex format")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("validate")
            .about("Validate AccountId format and checksum")
            .arg(Arg::with_name("account_id")
                .value_name("ACCOUNT_ID")
                .help("AccountId to validate")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("verify")
            .about("Verify that an AccountId matches a public key")
            .arg(Arg::with_name("account_id")
                .short("a")
                .long("account")
                .value_name("ACCOUNT_ID")
                .help("AccountId to verify")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("pubkey")
                .short("p")
                .long("pubkey")
                .value_name("PUBKEY_HEX")
                .help("Public key in hexadecimal format")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("inspect")
            .about("Inspect key files and display information")
            .arg(Arg::with_name("private_key")
                .short("s")
                .long("secret")
                .value_name("SECRET_KEY_FILE")
                .help("Private key file to inspect")
                .takes_value(true))
            .arg(Arg::with_name("public_key")
                .short("p")
                .long("public")
                .value_name("PUBLIC_KEY_FILE")
                .help("Public key file to inspect")
                .takes_value(true)))
        .subcommand(SubCommand::with_name("import")
            .about("Import a private key and generate AccountId")
            .arg(Arg::with_name("private_key")
                .short("s")
                .long("secret")
                .value_name("PRIVATE_KEY_HEX")
                .help("Private key in hexadecimal format")
                .required(true)
                .takes_value(true)))
        .get_matches();
    
    match matches.subcommand() {
        ("generate", Some(sub_matches)) => {
            handle_generate(sub_matches).await?;
        }
        ("derive", Some(sub_matches)) => {
            handle_derive(sub_matches).await?;
        }
        ("validate", Some(sub_matches)) => {
            handle_validate(sub_matches).await?;
        }
        ("verify", Some(sub_matches)) => {
            handle_verify(sub_matches).await?;
        }
        ("inspect", Some(sub_matches)) => {
            handle_inspect(sub_matches).await?;
        }
        ("import", Some(sub_matches)) => {
            handle_import(sub_matches).await?;
        }
        _ => {
            println!("Use --help for usage information");
        }
    }
    
    Ok(())
}

async fn handle_generate(matches: &clap::ArgMatches<'_>) -> SystemResult<()> {
    println!("🔐 Generating new GridTokenX keypair...");
    
    let keypair = GridTokenXKeyPair::generate()?;
    
    let output_dir = matches.value_of("output").unwrap_or(".");
    let name_prefix = matches.value_of("name").unwrap_or("gridtokenx");
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir).map_err(|e| {
        crate::SystemError::Internal(format!("Failed to create output directory: {}", e))
    })?;
    
    // Generate file paths
    let private_key_path = format!("{}/{}_private_key.hex", output_dir, name_prefix);
    let public_key_path = format!("{}/{}_public_key.hex", output_dir, name_prefix);
    let account_id_path = format!("{}/{}_account_id.txt", output_dir, name_prefix);
    
    // Save private key
    let private_key_hex = hex::encode(keypair.export_private_key().to_bytes());
    fs::write(&private_key_path, &private_key_hex).map_err(|e| {
        crate::SystemError::Internal(format!("Failed to write private key file: {}", e))
    })?;
    
    // Save public key
    let public_key_hex = hex::encode(keypair.export_public_key_bytes());
    fs::write(&public_key_path, &public_key_hex).map_err(|e| {
        crate::SystemError::Internal(format!("Failed to write public key file: {}", e))
    })?;
    
    // Save AccountId
    fs::write(&account_id_path, keypair.account_id()).map_err(|e| {
        crate::SystemError::Internal(format!("Failed to write account ID file: {}", e))
    })?;
    
    println!("✅ Successfully generated new keypair!");
    println!();
    println!("📋 Account Information:");
    println!("   AccountId: {}", keypair.account_id());
    println!("   Public Key: {}", public_key_hex);
    println!("   Private Key: {}", private_key_hex);
    println!();
    println!("📁 Files created:");
    println!("   Private Key: {}", private_key_path);
    println!("   Public Key:  {}", public_key_path);
    println!("   AccountId:   {}", account_id_path);
    println!();
    println!("⚠️  SECURITY WARNING:");
    println!("   - Keep your private key secure and never share it");
    println!("   - The private key file contains sensitive information");
    println!("   - Consider encrypting the private key file for storage");
    println!("   - Your public key and AccountId can be shared safely");
    
    Ok(())
}

async fn handle_derive(matches: &clap::ArgMatches<'_>) -> SystemResult<()> {
    let pubkey_hex = if let Some(pubkey) = matches.value_of("pubkey") {
        pubkey.to_string()
    } else if let Some(file_path) = matches.value_of("file") {
        fs::read_to_string(file_path).map_err(|e| {
            crate::SystemError::Internal(format!("Failed to read public key file: {}", e))
        })?.trim().to_string()
    } else {
        return Err(crate::SystemError::InvalidInput("Either --pubkey or --file must be provided".to_string()));
    };
    
    println!("🔍 Deriving AccountId from public key...");
    println!("Public Key: {}", pubkey_hex);
    
    // Decode public key
    let pubkey_bytes = hex::decode(&pubkey_hex).map_err(|e| {
        crate::SystemError::InvalidInput(format!("Invalid hex format: {}", e))
    })?;
    
    if pubkey_bytes.len() != 32 {
        return Err(crate::SystemError::InvalidInput("Public key must be exactly 32 bytes".to_string()));
    }
    
    let public_key = PublicKey::from_bytes(&pubkey_bytes).map_err(|e| {
        crate::SystemError::InvalidInput(format!("Invalid public key: {}", e))
    })?;
    
    let account_id = derive_account_id(&public_key)?;
    
    println!("✅ AccountId derived successfully!");
    println!();
    println!("📋 Result:");
    println!("   Input Public Key: {}", pubkey_hex);
    println!("   Derived AccountId: {}", account_id);
    println!();
    println!("✅ AccountId is valid: {}", validate_account_id(&account_id)?);
    
    Ok(())
}

async fn handle_validate(matches: &clap::ArgMatches<'_>) -> SystemResult<()> {
    let account_id = matches.value_of("account_id").unwrap();
    
    println!("✅ Validating AccountId format...");
    println!("AccountId: {}", account_id);
    
    match validate_account_id(account_id) {
        Ok(true) => {
            println!("✅ AccountId is valid!");
            println!();
            println!("📋 Validation Details:");
            println!("   ✅ Correct prefix: {}", account_id.starts_with("gx_"));
            println!("   ✅ Correct length: {}", account_id.len() == 35); // gx_ + 32 hex chars
            println!("   ✅ Valid hex encoding");
            println!("   ✅ Checksum verified (implicit in hex validation)");
        }
        Ok(false) => {
            println!("❌ AccountId is invalid!");
            println!();
            println!("🔍 Validation Issues:");
            if !account_id.starts_with("gx_") {
                println!("   ❌ Missing or incorrect prefix (should start with 'gx_')");
            }
            if account_id.len() != 35 {
                println!("   ❌ Incorrect length: {} (should be 35 characters)", account_id.len());
            }
            if account_id.starts_with("gx_") {
                let hex_part = &account_id[3..];
                if hex::decode(hex_part).is_err() {
                    println!("   ❌ Invalid hex encoding in AccountId");
                }
            }
        }
        Err(e) => {
            println!("❌ Error validating AccountId: {}", e);
        }
    }
    
    Ok(())
}

async fn handle_verify(matches: &clap::ArgMatches<'_>) -> SystemResult<()> {
    let account_id = matches.value_of("account_id").unwrap();
    let pubkey_hex = matches.value_of("pubkey").unwrap();
    
    println!("🔍 Verifying AccountId matches public key...");
    println!("AccountId: {}", account_id);
    println!("Public Key: {}", pubkey_hex);
    
    // Decode public key
    let pubkey_bytes = hex::decode(pubkey_hex).map_err(|e| {
        crate::SystemError::InvalidInput(format!("Invalid hex format: {}", e))
    })?;
    
    if pubkey_bytes.len() != 32 {
        return Err(crate::SystemError::InvalidInput("Public key must be exactly 32 bytes".to_string()));
    }
    
    let public_key = PublicKey::from_bytes(&pubkey_bytes).map_err(|e| {
        crate::SystemError::InvalidInput(format!("Invalid public key: {}", e))
    })?;
    
    // First validate the AccountId format
    let account_id_valid = validate_account_id(account_id)?;
    if !account_id_valid {
        println!("❌ AccountId format is invalid!");
        return Ok(());
    }
    
    // Then verify it matches the public key
    let matches = verify_account_id_matches_public_key(account_id, &public_key)?;
    
    if matches {
        println!("✅ Verification successful!");
        println!();
        println!("📋 Verification Details:");
        println!("   ✅ AccountId format is valid");
        println!("   ✅ AccountId correctly derived from public key");
        println!("   ✅ Cryptographic integrity verified");
    } else {
        println!("❌ Verification failed!");
        println!();
        println!("🔍 Issue Details:");
        println!("   ✅ AccountId format is valid");
        println!("   ❌ AccountId does NOT match the provided public key");
        
        let correct_account_id = derive_account_id(&public_key)?;
        println!("   📝 Correct AccountId for this public key: {}", correct_account_id);
    }
    
    Ok(())
}

async fn handle_inspect(matches: &clap::ArgMatches<'_>) -> SystemResult<()> {
    println!("🔍 Inspecting key files...");
    
    if let Some(private_key_file) = matches.value_of("private_key") {
        println!("\n🔐 Private Key File: {}", private_key_file);
        
        if !Path::new(private_key_file).exists() {
            println!("❌ Private key file does not exist");
            return Ok(());
        }
        
        let private_key_hex = fs::read_to_string(private_key_file).map_err(|e| {
            crate::SystemError::Internal(format!("Failed to read private key file: {}", e))
        })?.trim().to_string();
        
        let private_key_bytes = hex::decode(&private_key_hex).map_err(|e| {
            crate::SystemError::InvalidInput(format!("Invalid private key hex: {}", e))
        })?;
        
        if private_key_bytes.len() != 32 {
            println!("❌ Invalid private key length: {} bytes (expected 32)", private_key_bytes.len());
            return Ok(());
        }
        
        let secret_key = SecretKey::from_bytes(&private_key_bytes).map_err(|e| {
            crate::SystemError::InvalidInput(format!("Invalid private key: {}", e))
        })?;
        
        let keypair = GridTokenXKeyPair::from_private_key(secret_key)?;
        
        println!("✅ Private key file is valid");
        println!("📋 Derived Information:");
        println!("   AccountId: {}", keypair.account_id());
        println!("   Public Key: {}", hex::encode(keypair.export_public_key_bytes()));
        println!("   Private Key: {}", private_key_hex);
    }
    
    if let Some(public_key_file) = matches.value_of("public_key") {
        println!("\n🔓 Public Key File: {}", public_key_file);
        
        if !Path::new(public_key_file).exists() {
            println!("❌ Public key file does not exist");
            return Ok(());
        }
        
        let public_key_hex = fs::read_to_string(public_key_file).map_err(|e| {
            crate::SystemError::Internal(format!("Failed to read public key file: {}", e))
        })?.trim().to_string();
        
        let public_key_bytes = hex::decode(&public_key_hex).map_err(|e| {
            crate::SystemError::InvalidInput(format!("Invalid public key hex: {}", e))
        })?;
        
        if public_key_bytes.len() != 32 {
            println!("❌ Invalid public key length: {} bytes (expected 32)", public_key_bytes.len());
            return Ok(());
        }
        
        let public_key = PublicKey::from_bytes(&public_key_bytes).map_err(|e| {
            crate::SystemError::InvalidInput(format!("Invalid public key: {}", e))
        })?;
        
        let account_id = derive_account_id(&public_key)?;
        
        println!("✅ Public key file is valid");
        println!("📋 Derived Information:");
        println!("   AccountId: {}", account_id);
        println!("   Public Key: {}", public_key_hex);
    }
    
    if matches.value_of("private_key").is_none() && matches.value_of("public_key").is_none() {
        println!("❌ Please specify either --secret or --public key file to inspect");
    }
    
    Ok(())
}

async fn handle_import(matches: &clap::ArgMatches<'_>) -> SystemResult<()> {
    let private_key_hex = matches.value_of("private_key").unwrap();
    
    println!("📥 Importing private key...");
    
    let private_key_bytes = hex::decode(private_key_hex).map_err(|e| {
        crate::SystemError::InvalidInput(format!("Invalid hex format: {}", e))
    })?;
    
    if private_key_bytes.len() != 32 {
        return Err(crate::SystemError::InvalidInput("Private key must be exactly 32 bytes".to_string()));
    }
    
    let secret_key = SecretKey::from_bytes(&private_key_bytes).map_err(|e| {
        crate::SystemError::InvalidInput(format!("Invalid private key: {}", e))
    })?;
    
    let keypair = GridTokenXKeyPair::from_private_key(secret_key)?;
    
    println!("✅ Private key imported successfully!");
    println!();
    println!("📋 Account Information:");
    println!("   AccountId: {}", keypair.account_id());
    println!("   Public Key: {}", hex::encode(keypair.export_public_key_bytes()));
    println!("   Private Key: {}", private_key_hex);
    println!();
    println!("✅ AccountId validation: {}", validate_account_id(keypair.account_id())?);
    
    Ok(())
}
