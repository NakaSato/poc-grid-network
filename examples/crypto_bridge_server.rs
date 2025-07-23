use std::error::Error;
use thai_energy_trading_blockchain::bridge::crypto_bridge::{CryptoBridge, CryptoBridgeConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 Starting GridTokenX Crypto Bridge Server");
    println!("============================================");

    // Configure the crypto bridge
    let config = CryptoBridgeConfig {
        host: "0.0.0.0".to_string(),
        port: 8081,
        cors_origins: vec!["*".to_string()],
        max_request_size: 1024 * 1024, // 1MB
        rate_limit_per_minute: 100,
    };

    // Create and start the bridge
    let bridge = CryptoBridge::new(config);
    
    println!("🌐 Server will be available at: http://localhost:8081");
    println!("📖 API Documentation:");
    println!();
    println!("POST /crypto/generate-keypair");
    println!("  Generate a new Ed25519 keypair");
    println!("  Body: {{ \"entropy\": \"optional_hex_string\" }}");
    println!();
    println!("POST /crypto/validate-account-id");
    println!("  Validate AccountId format");
    println!("  Body: {{ \"account_id\": \"gx_...\" }}");
    println!();
    println!("POST /crypto/derive-account-id");
    println!("  Derive AccountId from public key");
    println!("  Body: {{ \"public_key\": \"hex_string\" }}");
    println!();
    println!("POST /crypto/sign-transaction");
    println!("  Sign transaction data");
    println!("  Body: {{ \"private_key\": \"hex\", \"transaction_data\": \"data\" }}");
    println!();
    println!("POST /crypto/verify-signature");
    println!("  Verify signature");
    println!("  Body: {{ \"public_key\": \"hex\", \"signature\": \"hex\", \"message\": \"data\" }}");
    println!();
    println!("POST /crypto/generate-test-accounts");
    println!("  Generate test accounts");
    println!("  Body: {{ \"count\": 5 }}");
    println!();
    println!("POST /crypto/hash-data");
    println!("  Hash data with SHA256");
    println!("  Body: {{ \"data\": \"string\", \"encoding\": \"utf8|hex|base64\" }}");
    println!();
    println!("GET /crypto/health");
    println!("  Health check endpoint");
    println!();
    println!("GET /crypto/info");
    println!("  Bridge information");
    println!();
    println!("🔧 Example curl commands:");
    println!();
    println!("# Generate keypair");
    println!("curl -X POST http://localhost:8081/crypto/generate-keypair \\");
    println!("  -H 'Content-Type: application/json' \\");
    println!("  -d '{{}}'");
    println!();
    println!("# Validate AccountId");
    println!("curl -X POST http://localhost:8081/crypto/validate-account-id \\");
    println!("  -H 'Content-Type: application/json' \\");
    println!("  -d '{{\"account_id\": \"gx_a1b2c3d4e5f6789012345678901234ab\"}}'");
    println!();
    println!("# Health check");
    println!("curl http://localhost:8081/crypto/health");
    println!();
    println!("✨ Press Ctrl+C to stop the server");
    println!();

    // Start the bridge (this will run indefinitely)
    bridge.start().await?;

    Ok(())
}
