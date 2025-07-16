//! # Runtime Layer
//! 
//! This module implements the blockchain runtime including token system,
//! energy trading pallet, compliance pallet, and hybrid architecture.

pub mod token_system;
pub mod energy_trading;
pub mod compliance;
pub mod hybrid_arch;

use crate::config::SystemConfig;
use crate::utils::SystemResult;

/// Runtime layer manager
pub struct RuntimeLayer {
    token_system: token_system::TokenSystem,
    energy_trading: energy_trading::EnergyTradingPallet,
    compliance: compliance::CompliancePallet,
    hybrid_arch: hybrid_arch::HybridArchitecture,
}

impl RuntimeLayer {
    pub async fn new(config: &SystemConfig) -> SystemResult<Self> {
        let token_system = token_system::TokenSystem::new(config).await?;
        let energy_trading = energy_trading::EnergyTradingPallet::new(config).await?;
        let compliance = compliance::CompliancePallet::new(config).await?;
        let hybrid_arch = hybrid_arch::HybridArchitecture::new(config).await?;
        
        Ok(Self {
            token_system,
            energy_trading,
            compliance,
            hybrid_arch,
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        self.token_system.start().await?;
        self.energy_trading.start().await?;
        self.compliance.start().await?;
        self.hybrid_arch.start().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        self.hybrid_arch.stop().await?;
        self.compliance.stop().await?;
        self.energy_trading.stop().await?;
        self.token_system.stop().await?;
        
        Ok(())
    }
}
