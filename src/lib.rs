//! # GridTokenX POC Blockchain Library
//! 
//! A blockchain-based energy trading POC platform that enables
//! peer-to-peer energy trading with 1:1 token-energy ratio.
//! 
//! ## Architecture Overview
//! 
//! The system is built with a layered architecture focused on blockchain operations:
//! - **Blockchain Interface**: Direct blockchain interactions without HTTP/REST APIs
//! - **Application Layer**: Business logic services (Trading, Grid, Governance, Oracle)
//! - **Runtime Layer**: Core blockchain runtime (Token, Energy Trading, Compliance)
//! - **Blockchain Layer**: Consensus, transaction pool, storage, and networking
//! - **Infrastructure Layer**: Physical grid, smart meters, cloud services, security
//! 
//! ## Usage
//! 
//! ```rust
//! use thai_energy_trading_blockchain::{ThaiEnergyTradingSystem, SystemConfig};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = SystemConfig::default(); // Use default config for testing
//!     let system = ThaiEnergyTradingSystem::new(config).await?;
//!     system.start().await?;
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use log::info;
use anyhow::Result;

// Public modules that form the library API
pub mod blockchain;
pub mod runtime;
pub mod application;
pub mod infrastructure;
pub mod interface;
pub mod config;
pub mod types;
pub mod utils;

// Re-export commonly used types and functions
pub use config::SystemConfig;
pub use types::*;
pub use utils::{SystemResult, SystemError};

use crate::infrastructure::database::DatabaseManager;
use crate::infrastructure::grid::GridManager;
use crate::infrastructure::security::SecurityManager;
use crate::application::trading::TradingService;
use crate::application::grid::GridService;
use crate::application::governance::GovernanceService;
use crate::application::oracle::OracleService;
use crate::interface::api::{BlockchainInterface, BlockchainStatus};
use crate::blockchain::node::BlockchainNode;

/// Main blockchain system context that holds all system components
/// This is the primary entry point for using the Thai Energy Trading Blockchain library
pub struct ThaiEnergyTradingSystem {
    config: SystemConfig,
    blockchain_node: Arc<BlockchainNode>,
    database_manager: Arc<DatabaseManager>,
    grid_manager: Arc<GridManager>,
    security_manager: Arc<SecurityManager>,
    trading_service: Arc<TradingService>,
    grid_service: Arc<GridService>,
    governance_service: Arc<GovernanceService>,
    oracle_service: Arc<OracleService>,
    blockchain_interface: Arc<BlockchainInterface>,
}

impl ThaiEnergyTradingSystem {
    /// Create a new GridTokenX POC System instance
    pub async fn new(config: SystemConfig) -> Result<Self> {
        info!("🚀 Initializing GridTokenX POC Blockchain System");
        
        // Initialize infrastructure components
        let database_manager = Arc::new(DatabaseManager::new(&config.database, config.test_mode).await?);
        let grid_manager = Arc::new(GridManager::new(&config.grid).await?);
        let security_manager = Arc::new(SecurityManager::new(&config.security).await?);
        
        // Initialize blockchain node
        let blockchain_node = Arc::new(BlockchainNode::new(&config.blockchain).await?);
        
        // Initialize application services
        let trading_service = Arc::new(TradingService::new_placeholder().await?);
        let grid_service = Arc::new(GridService::new_placeholder().await?);
        let governance_service = Arc::new(GovernanceService::new_placeholder().await?);
        let oracle_service = Arc::new(OracleService::new_placeholder().await?);
        
        // Initialize blockchain interface
        let blockchain_interface = Arc::new(BlockchainInterface::new(
            trading_service.clone(),
            grid_service.clone(),
            governance_service.clone(),
            oracle_service.clone(),
            security_manager.clone(),
        ).await?);
        
        Ok(Self {
            config,
            blockchain_node,
            database_manager,
            grid_manager,
            security_manager,
            trading_service,
            grid_service,
            governance_service,
            oracle_service,
            blockchain_interface,
        })
    }
    
    /// Start the GridTokenX POC System
    pub async fn start(&self) -> Result<()> {
        info!("⚡ Starting GridTokenX POC Blockchain System");
        
        // Start infrastructure layer
        self.database_manager.start().await?;
        self.grid_manager.start().await?;
        self.security_manager.start().await?;
        
        // Start blockchain node
        self.blockchain_node.start().await?;
        
        // Start application services
        self.trading_service.start().await?;
        self.grid_service.start().await?;
        self.governance_service.start().await?;
        self.oracle_service.start().await?;
        
        // Start blockchain interface
        self.blockchain_interface.start().await?;
        
        info!("✅ GridTokenX POC Blockchain System started successfully");
        Ok(())
    }
    
    /// Stop the GridTokenX POC System
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping GridTokenX POC Blockchain System");
        
        // Stop in reverse order
        self.blockchain_interface.stop().await?;
        
        self.oracle_service.stop().await?;
        self.governance_service.stop().await?;
        self.grid_service.stop().await?;
        self.trading_service.stop().await?;
        
        self.blockchain_node.stop().await?;
        
        self.security_manager.stop().await?;
        self.grid_manager.stop().await?;
        self.database_manager.stop().await?;
        
        info!("✅ GridTokenX POC Blockchain System stopped successfully");
        Ok(())
    }
    
    /// Get system configuration
    pub fn config(&self) -> &SystemConfig {
        &self.config
    }
    
    /// Get blockchain node reference
    pub fn blockchain_node(&self) -> Arc<BlockchainNode> {
        self.blockchain_node.clone()
    }
    
    /// Get blockchain interface reference
    pub fn blockchain_interface(&self) -> Arc<BlockchainInterface> {
        self.blockchain_interface.clone()
    }
    
    /// Get trading service reference
    pub fn trading_service(&self) -> Arc<TradingService> {
        self.trading_service.clone()
    }
    
    /// Get grid service reference
    pub fn grid_service(&self) -> Arc<GridService> {
        self.grid_service.clone()
    }
    
    /// Get governance service reference
    pub fn governance_service(&self) -> Arc<GovernanceService> {
        self.governance_service.clone()
    }
    
    /// Get oracle service reference
    pub fn oracle_service(&self) -> Arc<OracleService> {
        self.oracle_service.clone()
    }
    
    // Additional methods for testing and operations
    
    /// Get blockchain status
    pub async fn get_blockchain_status(&self) -> Result<BlockchainStatus> {
        self.blockchain_interface.get_blockchain_status().await
            .map_err(|e| anyhow::anyhow!("Failed to get blockchain status: {}", e))
    }
    
    /// Get trading status
    pub async fn get_trading_status(&self) -> Result<String> {
        // Return a simple status for now
        Ok("Trading system operational".to_string())
    }
    
    /// Place energy order
    pub async fn place_energy_order(&self, order: &EnergyOrder) -> Result<uuid::Uuid> {
        self.trading_service.place_order(order.clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to place order: {}", e))
    }
    
    /// Cancel energy order
    pub async fn cancel_energy_order(&self, order_id: &uuid::Uuid, account_id: &AccountId) -> Result<()> {
        self.trading_service.cancel_order(*order_id, account_id.clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to cancel order: {}", e))
    }
    
    /// Get grid status
    pub async fn get_grid_status(&self, location: &GridLocation) -> Result<GridStatus> {
        self.grid_service.get_grid_status(location).await
            .map_err(|e| anyhow::anyhow!("Failed to get grid status: {}", e))
    }
    
    /// Validate grid load
    pub async fn validate_grid_load(&self, location: &GridLocation, additional_load: f64) -> Result<bool> {
        let status = self.grid_service.get_grid_status(location).await
            .map_err(|e| anyhow::anyhow!("Failed to get grid status: {}", e))?;
        Ok(status.can_handle_load(additional_load))
    }
    
    /// Create governance proposal
    pub async fn create_governance_proposal(&self, proposal: &GovernanceProposal) -> Result<uuid::Uuid> {
        self.governance_service.create_proposal(proposal.clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to create proposal: {}", e))
    }
    
    /// Vote on governance proposal
    pub async fn vote_on_proposal(&self, proposal_id: &uuid::Uuid, account_id: &AccountId, vote: VoteChoice, voting_power: Balance) -> Result<()> {
        // Convert VoteChoice to the correct type
        let gov_vote = match vote {
            VoteChoice::Yes => crate::application::governance::VoteChoice::Yes,
            VoteChoice::No => crate::application::governance::VoteChoice::No,
            VoteChoice::Abstain => crate::application::governance::VoteChoice::Abstain,
        };
        
        self.governance_service.vote_on_proposal(*proposal_id, account_id, gov_vote, voting_power).await
            .map_err(|e| anyhow::anyhow!("Failed to vote on proposal: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_system_creation() {
        let config = SystemConfig::default();
        let system = ThaiEnergyTradingSystem::new(config).await;
        assert!(system.is_ok());
    }
    
    #[tokio::test]
    async fn test_system_lifecycle() {
        let config = SystemConfig::default();
        let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
        
        // Test start
        let start_result = system.start().await;
        if let Err(e) = &start_result {
            eprintln!("Start failed with error: {:?}", e);
        }
        assert!(start_result.is_ok());
        
        // Test stop
        let stop_result = system.stop().await;
        assert!(stop_result.is_ok());
    }
}
