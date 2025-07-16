//! # Storage Layer
//! 
//! Implements blockchain storage and state management.

use crate::config::BlockchainConfig;
use crate::utils::SystemResult;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Blockchain storage layer
pub struct BlockchainStorage {
    config: BlockchainConfig,
    running: Arc<RwLock<bool>>,
}

impl BlockchainStorage {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        Ok(Self {
            config: config.clone(),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Blockchain Storage");
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Blockchain Storage");
        
        Ok(())
    }
}
