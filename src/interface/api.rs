//! # Blockchain Interface
//! 
//! Direct blockchain interface for programmatic blockchain interactions.
//! This is NOT an HTTP/REST API - it provides direct access to blockchain functionality.

use crate::application::trading::TradingService;
use crate::application::grid::GridService;
use crate::application::governance::GovernanceService;
use crate::application::oracle::OracleService;
use crate::infrastructure::security::SecurityManager;
use crate::utils::SystemResult;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Blockchain interface for direct blockchain interactions
/// This provides programmatic access to blockchain functionality without HTTP layer
pub struct BlockchainInterface {
    trading_service: Arc<TradingService>,
    grid_service: Arc<GridService>,
    governance_service: Arc<GovernanceService>,
    oracle_service: Arc<OracleService>,
    security_manager: Arc<SecurityManager>,
    running: Arc<RwLock<bool>>,
}

impl BlockchainInterface {
    pub async fn new(
        trading_service: Arc<TradingService>,
        grid_service: Arc<GridService>,
        governance_service: Arc<GovernanceService>,
        oracle_service: Arc<OracleService>,
        security_manager: Arc<SecurityManager>,
    ) -> SystemResult<Self> {
        Ok(Self {
            trading_service,
            grid_service,
            governance_service,
            oracle_service,
            security_manager,
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Blockchain Interface");
        
        // Start blockchain interface services
        self.start_blockchain_services().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Blockchain Interface");
        
        Ok(())
    }
    
    /// Start blockchain services
    async fn start_blockchain_services(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("BlockchainInterface", "Starting blockchain services");
        
        // All blockchain services are now running through the underlying components
        // No HTTP server needed for blockchain-only system
        
        Ok(())
    }
    
    /// Get blockchain status
    pub async fn get_blockchain_status(&self) -> SystemResult<BlockchainStatus> {
        Ok(BlockchainStatus {
            system_status: "running".to_string(),
            service: "Thai Energy Trading Blockchain".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: crate::utils::now().timestamp() as u64,
        })
    }
}

/// Blockchain status structure
#[derive(Debug, Clone)]
pub struct BlockchainStatus {
    pub system_status: String,
    pub service: String,
    pub version: String,
    pub timestamp: u64,
}
