//! # Transaction Pool
//! 
//! Manages pending transactions before they are included in blocks.

use crate::config::BlockchainConfig;
use crate::utils::SystemResult;
use crate::blockchain::transactions::EnergyTransactionEnvelope;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::types::Hash;

/// Transaction pool for managing pending transactions
pub struct TransactionPool {
    config: BlockchainConfig,
    running: Arc<RwLock<bool>>,
    transactions: Arc<RwLock<HashMap<Hash, EnergyTransactionEnvelope>>>,
}

impl TransactionPool {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        Ok(Self {
            config: config.clone(),
            running: Arc::new(RwLock::new(false)),
            transactions: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Transaction Pool");
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Transaction Pool");
        
        Ok(())
    }
    
    /// Add transaction to the pool
    pub async fn add_transaction(&self, transaction: EnergyTransactionEnvelope) -> SystemResult<()> {
        let mut transactions = self.transactions.write().await;
        let transaction_hash = hex::encode(transaction.hash);
        
        // Check if transaction already exists
        if transactions.contains_key(&transaction_hash) {
            return Err(crate::utils::error::SystemError::Internal("Transaction already exists".to_string()));
        }
        
        // Add transaction to pool
        transactions.insert(transaction_hash, transaction);
        
        crate::utils::logging::log_info(
            "TransactionPool",
            "Added transaction to pool"
        );
        
        Ok(())
    }
    
    /// Remove transaction from pool
    pub async fn remove_transaction(&self, hash: &str) -> SystemResult<()> {
        let mut transactions = self.transactions.write().await;
        transactions.remove(hash);
        Ok(())
    }
    
    /// Get all transactions
    pub async fn get_all_transactions(&self) -> Vec<EnergyTransactionEnvelope> {
        let transactions = self.transactions.read().await;
        transactions.values().cloned().collect()
    }
}
