//! # Network Layer
//! 
//! Implements P2P networking for blockchain communication.

use crate::config::BlockchainConfig;
use crate::utils::SystemResult;
use std::sync::Arc;
use tokio::sync::RwLock;

/// P2P network layer
pub struct NetworkLayer {
    config: BlockchainConfig,
    running: Arc<RwLock<bool>>,
}

impl NetworkLayer {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        Ok(Self {
            config: config.clone(),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Network Layer");
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Network Layer");
        
        Ok(())
    }
}
