//! # Basic Usage Example
//! 
//! This example demonstrates how to use the Thai Energy Trading Blockchain library
//! to create a basic energy trading system.

use thai_energy_trading_blockchain::{ThaiEnergyTradingSystem, SystemConfig};
use tokio::time::{sleep, Duration};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    info!("🚀 Starting Thai Energy Trading Blockchain Example");
    
    // Create system configuration
    let config = SystemConfig::default();
    
    // Create and start the system
    let system = ThaiEnergyTradingSystem::new(config).await?;
    system.start().await?;
    
    info!("✅ System started successfully");
    
    // Get system components
    let trading_service = system.trading_service();
    let blockchain_interface = system.blockchain_interface();
    let grid_service = system.grid_service();
    
    // Demonstrate blockchain status
    let status = blockchain_interface.get_blockchain_status().await?;
    info!("📊 Blockchain Status: {:?}", status);
    
    // Simulate some operations
    info!("🔄 Running example operations...");
    sleep(Duration::from_secs(2)).await;
    
    // Stop the system
    info!("🛑 Stopping system...");
    system.stop().await?;
    
    info!("✅ Example completed successfully");
    Ok(())
}
