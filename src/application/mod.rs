//! # Application Layer
//! 
//! This module implements the business logic services including Trading Service,
//! Grid Management, Governance Service, and Oracle Service.

pub mod trading;
pub mod enhanced_trading;
pub mod grid;
pub mod governance;
pub mod oracle;

use crate::config::SystemConfig;
use crate::utils::SystemResult;

/// Application layer manager
pub struct ApplicationLayer {
    trading_service: trading::TradingService,
    grid_service: grid::GridService,
    governance_service: governance::GovernanceService,
    oracle_service: oracle::OracleService,
}

impl ApplicationLayer {
    pub async fn new(config: &SystemConfig) -> SystemResult<Self> {
        // These would typically be initialized with proper dependencies
        // For now, we'll create placeholder implementations
        
        Ok(Self {
            trading_service: trading::TradingService::new_placeholder().await?,
            grid_service: grid::GridService::new_placeholder().await?,
            governance_service: governance::GovernanceService::new_placeholder().await?,
            oracle_service: oracle::OracleService::new_placeholder().await?,
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        tokio::try_join!(
            self.trading_service.start(),
            self.grid_service.start(),
            self.governance_service.start(),
            self.oracle_service.start()
        )?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        tokio::try_join!(
            self.trading_service.stop(),
            self.grid_service.stop(),
            self.governance_service.stop(),
            self.oracle_service.stop()
        )?;
        
        Ok(())
    }
}
