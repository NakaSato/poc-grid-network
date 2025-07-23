use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem, 
    SystemConfig,
    bridge::{ApiBridge, BridgeConfig}
};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting GridTokenX API Bridge Example");
    
    // Create system configuration
    let system_config = SystemConfig::default();
    
    // Create blockchain system
    let mut blockchain_system = ThaiEnergyTradingSystem::new(system_config).await?;
    
    // Create API bridge configuration
    let bridge_config = BridgeConfig {
        port: 8081, // Use different port to avoid conflict with Docker
        host: "0.0.0.0".to_string(),
        cors_origins: vec!["*".to_string()],
        debug: true,
    };
    
    println!("🔧 Configuring API Bridge on {}:{}", bridge_config.host, bridge_config.port);
    
    // Enable API bridge
    blockchain_system.enable_api_bridge(bridge_config).await?;
    
    println!("✅ API Bridge enabled successfully");
    
    // Start the API bridge server
    println!("🌐 Starting API Bridge server...");
    blockchain_system.start_api_bridge().await?;
    
    Ok(())
}
