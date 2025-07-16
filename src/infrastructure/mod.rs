//! # Infrastructure Layer
//! 
//! This module implements the infrastructure components including database management,
//! grid integration, cloud services, and security systems.

pub mod database;
pub mod grid;
pub mod cloud;
pub mod security;

use crate::config::SystemConfig;
use crate::utils::SystemResult;

/// Infrastructure layer manager
pub struct InfrastructureLayer {
    database_manager: database::DatabaseManager,
    grid_manager: grid::GridManager,
    cloud_manager: cloud::CloudManager,
    security_manager: security::SecurityManager,
}

impl InfrastructureLayer {
    pub async fn new(config: &SystemConfig) -> SystemResult<Self> {
        let database_manager = database::DatabaseManager::new(&config.database, config.test_mode).await?;
        let grid_manager = grid::GridManager::new(&config.grid).await?;
        let cloud_manager = cloud::CloudManager::new(config).await?;
        let security_manager = security::SecurityManager::new(&config.security).await?;
        
        Ok(Self {
            database_manager,
            grid_manager,
            cloud_manager,
            security_manager,
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        self.database_manager.start().await?;
        self.grid_manager.start().await?;
        self.cloud_manager.start().await?;
        self.security_manager.start().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        self.security_manager.stop().await?;
        self.cloud_manager.stop().await?;
        self.grid_manager.stop().await?;
        self.database_manager.stop().await?;
        
        Ok(())
    }
}
