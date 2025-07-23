use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use crate::{SystemResult, ThaiEnergyTradingSystem};

/// Configuration for the API bridge server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Server port (default: 8080)
    pub port: u16,
    /// Server host (default: "0.0.0.0")
    pub host: String,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
    /// Enable debug mode
    pub debug: bool,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            cors_origins: vec!["*".to_string()],
            debug: false,
        }
    }
}

/// Main API bridge that provides HTTP access to blockchain functionality
pub struct ApiBridge {
    /// Reference to the blockchain system
    blockchain_system: Arc<ThaiEnergyTradingSystem>,
    /// Configuration for the bridge
    config: BridgeConfig,
}

impl ApiBridge {
    /// Create a new API bridge instance
    pub async fn new(
        blockchain_system: Arc<ThaiEnergyTradingSystem>,
        config: BridgeConfig,
    ) -> SystemResult<Self> {
        Ok(Self {
            blockchain_system,
            config,
        })
    }

    /// Start the API bridge server
    pub async fn start(&self) -> SystemResult<()> {
        println!("Starting API Bridge on {}:{}", self.config.host, self.config.port);

        // Create CORS filter
        let cors = warp::cors()
            .allow_any_origin()
            .allow_headers(vec!["content-type", "authorization"])
            .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);

        // Health check endpoint
        let health = warp::path("health")
            .and(warp::get())
            .map(|| {
                warp::reply::json(&serde_json::json!({
                    "status": "healthy",
                    "service": "GridTokenX API Bridge",
                    "version": "1.0.0"
                }))
            });

        // System status endpoint
        let system_status = warp::path("api")
            .and(warp::path("v1"))
            .and(warp::path("system"))
            .and(warp::path("status"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|| {
                warp::reply::json(&serde_json::json!({
                    "blockchain_status": "running",
                    "consensus": "proof_of_authority",
                    "network_id": "thai_energy_grid",
                    "connected_nodes": 1,
                    "current_block": 0,
                    "gas_price": "0.001"
                }))
            });

        // Market data endpoint
        let market_data = warp::path("api")
            .and(warp::path("v1"))
            .and(warp::path("trading"))
            .and(warp::path("market"))
            .and(warp::path::end())
            .and(warp::get())
            .map(|| {
                warp::reply::json(&serde_json::json!({
                    "markets": [
                        {
                            "pair": "ETH/THB",
                            "price": "85000.00",
                            "volume_24h": "1250.50",
                            "change_24h": "+2.5%"
                        },
                        {
                            "pair": "ENERGY/THB", 
                            "price": "12.50",
                            "volume_24h": "8750.25",
                            "change_24h": "+1.2%"
                        }
                    ],
                    "total_volume_24h": "10000.75",
                    "active_orders": 145,
                    "recent_trades": 89
                }))
            });

        // Combine all routes
        let routes = health
            .or(system_status)
            .or(market_data)
            .with(cors)
            .with(warp::log("api_bridge"));

        // Parse host and port
        let addr: std::net::SocketAddr = format!("{}:{}", self.config.host, self.config.port)
            .parse()
            .map_err(|e| crate::SystemError::Internal(format!("Invalid host:port format: {}", e)))?;

        println!("API Bridge server starting on http://{}", addr);
        
        // Start server
        warp::serve(routes)
            .run(addr)
            .await;

        Ok(())
    }

    /// Get bridge configuration
    pub fn config(&self) -> &BridgeConfig {
        &self.config
    }

    /// Get blockchain system reference
    pub fn blockchain_system(&self) -> &Arc<ThaiEnergyTradingSystem> {
        &self.blockchain_system
    }
}
