//! # Interface Layer
//! 
//! This module implements the blockchain interface components.
//! Focused on direct blockchain interactions without HTTP/REST API layers.

pub mod api;
pub mod cda_api;

use crate::application::*;
use crate::infrastructure::security::SecurityManager;
use crate::utils::SystemResult;
use std::sync::Arc;

/// Interface layer manager for blockchain-only operations
pub struct InterfaceLayer {
    blockchain_interface: api::BlockchainInterface,
}

impl InterfaceLayer {
    pub async fn new(
        trading_service: Arc<trading::TradingService>,
        grid_service: Arc<grid::GridService>,
        governance_service: Arc<governance::GovernanceService>,
        oracle_service: Arc<oracle::OracleService>,
        security_manager: Arc<SecurityManager>,
    ) -> SystemResult<Self> {
        let blockchain_interface = api::BlockchainInterface::new(
            trading_service,
            grid_service,
            governance_service,
            oracle_service,
            security_manager,
        ).await?;
        
        Ok(Self {
            blockchain_interface,
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        self.blockchain_interface.start().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        self.blockchain_interface.stop().await?;
        
        Ok(())
    }
}
