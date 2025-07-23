use std::env;
use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem, 
    SystemConfig,
    bridge::BridgeConfig
};

/// Example demonstrating the GridTokenX API Bridge
/// This shows how to start the blockchain with public HTTP/WebSocket API access
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🌉 GridTokenX API Bridge Example");
    println!("================================");

    // Create system configuration
    let mut config = SystemConfig::default();
    config.test_mode = true; // Enable test mode for local development

    // Initialize the blockchain system
    let mut system = ThaiEnergyTradingSystem::new(config).await?;

    // Configure API bridge for public access
    let bridge_config = BridgeConfig {
        host: "0.0.0.0".to_string(),        // Listen on all interfaces
        port: 8080,                         // HTTP API port
        ws_port: 8081,                      // WebSocket port
        enable_cors: true,                  // Enable CORS for web access
        rate_limit: Some(100),              // 100 requests per minute
        api_key_required: false,            // No API key required for demo
        max_connections: 1000,              // Max WebSocket connections
        request_timeout_seconds: 30,        // 30 second timeout
        enable_metrics: true,               // Enable metrics collection
    };

    // Enable the API bridge
    system.enable_api_bridge(bridge_config).await?;

    // Start the blockchain system
    println!("🚀 Starting GridTokenX blockchain...");
    system.start().await?;

    // Start the API bridge server
    println!("🌐 Starting API Bridge server...");
    println!("   HTTP API: http://0.0.0.0:8080");
    println!("   WebSocket: ws://0.0.0.0:8081/ws");
    println!("   Health Check: http://0.0.0.0:8080/health");
    println!();
    println!("📚 Available API Endpoints:");
    println!("   GET  /health                           - Health check");
    println!("   GET  /api/v1/system/status             - System status");
    println!("   POST /api/v1/trading/orders            - Place energy order");
    println!("   GET  /api/v1/trading/orders            - List orders");
    println!("   GET  /api/v1/trading/orders/:id        - Get order details");
    println!("   GET  /api/v1/trading/market            - Market data");
    println!("   POST /api/v1/tokens/transfer           - Transfer tokens");
    println!("   GET  /api/v1/blockchain/blocks         - List blocks");
    println!("   GET  /api/v1/analytics/tps             - TPS metrics");
    println!();
    println!("🔌 WebSocket Channels:");
    println!("   orderbook     - Order book updates");
    println!("   trades        - Trade executions");
    println!("   blocks        - New blocks");
    println!("   market_data   - Market data updates");
    println!("   system_status - System status updates");
    println!();
    println!("💡 Try these examples:");
    println!("   curl http://localhost:8080/health");
    println!("   curl http://localhost:8080/api/v1/system/status");
    println!("   curl http://localhost:8080/api/v1/trading/market");
    println!();

    // This will run the server indefinitely
    system.start_api_bridge().await?;

    Ok(())
}

/// Test helper functions
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_bridge_creation() {
        let config = SystemConfig::default();
        let mut system = ThaiEnergyTradingSystem::new(config).await.unwrap();
        
        let bridge_config = BridgeConfig::default();
        system.enable_api_bridge(bridge_config).await.unwrap();
        
        assert!(system.api_bridge().is_some());
    }
}
