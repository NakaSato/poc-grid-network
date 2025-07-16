//! # Blockchain Node Implementation
//! 
//! This module implements the main blockchain node that coordinates all blockchain operations.

use crate::blockchain::BlockchainEngine;
use crate::config::BlockchainConfig;
use crate::utils::SystemResult;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main blockchain node
pub struct BlockchainNode {
    /// The blockchain engine
    engine: Arc<BlockchainEngine>,
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl BlockchainNode {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        let engine = Arc::new(BlockchainEngine::new(config).await?);
        
        Ok(Self {
            engine,
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        // Start the blockchain engine
        self.engine.start().await?;
        
        crate::utils::logging::log_info("BlockchainNode", "Started successfully");
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        // Stop the blockchain engine
        self.engine.stop().await?;
        
        crate::utils::logging::log_shutdown("BlockchainNode");
        Ok(())
    }
    
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
    
    /// Get the blockchain engine
    pub fn engine(&self) -> Arc<BlockchainEngine> {
        self.engine.clone()
    }
}

/// Node manager for blockchain operations
pub struct NodeManager {
    config: BlockchainConfig,
    running: Arc<RwLock<bool>>,
}

impl NodeManager {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        Ok(Self {
            config: config.clone(),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Node Manager");
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Node Manager");
        
        Ok(())
    }
}
