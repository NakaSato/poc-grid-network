//! # GridTokenX POC Blockchain Binary
//! 
//! Binary executable for running the GridTokenX POC Blockchain system as a standalone application.

use thai_energy_trading_blockchain::{ThaiEnergyTradingSystem, SystemConfig};
use tokio::signal;
use log::{info, error};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    info!("🔋 GridTokenX POC Blockchain v{}", env!("CARGO_PKG_VERSION"));
    info!("⚡ Blockchain-based peer-to-peer energy trading POC platform");
    info!("💰 1 kWh = 1 Token | Sustainable • Transparent • Decentralized");
    info!("🏛️ Consensus: Proof-of-Authority (PoA) - No mining required");
    info!("🔐 Energy-focused blockchain POC with grid integration");
    
    // Load configuration
    let config = SystemConfig::load().await?;
    
    // Initialize and start the blockchain system
    let system = ThaiEnergyTradingSystem::new(config).await?;
    system.start().await?;
    
    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("🔄 Received shutdown signal, stopping GridTokenX POC system...");
            system.stop().await?;
        }
        Err(err) => {
            error!("🚨 Unable to listen for shutdown signal: {}", err);
        }
    }
    
    Ok(())
}
