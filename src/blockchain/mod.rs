//! # Blockchain Layer
//! 
//! This module implements the core blockchain functionality including consensus,
//! transaction pool, storage, and network layer.

pub mod consensus;
pub mod transaction_pool;
pub mod storage;
pub mod network;
pub mod node;
pub mod transactions;
pub mod smart_contracts;

use crate::config::BlockchainConfig;
use crate::utils::SystemResult;
use crate::blockchain::smart_contracts::{SmartContractVM, ContractABI, ContractExecutionResult};
use crate::blockchain::transactions::{EnergyTransactionValidator, TransactionValidationResult, EnergyTransactionEnvelope};
use crate::types::AccountId;
use std::sync::Arc;

/// Blockchain engine that coordinates all blockchain components
pub struct BlockchainEngine {
    /// Consensus engine
    consensus: Arc<consensus::ConsensusEngine>,
    /// Transaction pool
    transaction_pool: Arc<transaction_pool::TransactionPool>,
    /// Storage layer
    storage: Arc<storage::BlockchainStorage>,
    /// Smart contract VM
    smart_contract_vm: Arc<SmartContractVM>,
    /// Network layer
    network: Arc<network::NetworkLayer>,
    /// Node manager
    node_manager: Arc<node::NodeManager>,
}

impl BlockchainEngine {
    /// Create new blockchain engine
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        let consensus = Arc::new(consensus::ConsensusEngine::new(config).await?);
        let transaction_pool = Arc::new(transaction_pool::TransactionPool::new(config).await?);
        let storage = Arc::new(storage::BlockchainStorage::new(config).await?);
        let smart_contract_vm = Arc::new(SmartContractVM::new(1_000_000)); // 1M gas limit
        let network = Arc::new(network::NetworkLayer::new(config).await?);
        let node_manager = Arc::new(node::NodeManager::new(config).await?);

        Ok(Self {
            consensus,
            transaction_pool,
            storage,
            smart_contract_vm,
            network,
            node_manager,
        })
    }

    /// Start the blockchain engine
    pub async fn start(&self) -> SystemResult<()> {
        // Start all components
        self.consensus.start().await?;
        self.transaction_pool.start().await?;
        self.storage.start().await?;
        self.network.start().await?;
        self.node_manager.start().await?;
        
        crate::utils::logging::log_info("BlockchainEngine", "Started successfully");
        Ok(())
    }

    /// Stop the blockchain engine
    pub async fn stop(&self) -> SystemResult<()> {
        self.consensus.stop().await?;
        self.transaction_pool.stop().await?;
        self.storage.stop().await?;
        self.network.stop().await?;
        self.node_manager.stop().await?;
        
        crate::utils::logging::log_shutdown("BlockchainEngine");
        Ok(())
    }

    /// Get consensus engine
    pub fn consensus(&self) -> Arc<consensus::ConsensusEngine> {
        self.consensus.clone()
    }

    /// Get transaction pool
    pub fn transaction_pool(&self) -> Arc<transaction_pool::TransactionPool> {
        self.transaction_pool.clone()
    }

    /// Get storage layer
    pub fn storage(&self) -> Arc<storage::BlockchainStorage> {
        self.storage.clone()
    }

    /// Get smart contract VM
    pub fn smart_contract_vm(&self) -> Arc<SmartContractVM> {
        self.smart_contract_vm.clone()
    }

    /// Get network layer
    pub fn network(&self) -> Arc<network::NetworkLayer> {
        self.network.clone()
    }

    /// Get node manager
    pub fn node_manager(&self) -> Arc<node::NodeManager> {
        self.node_manager.clone()
    }

    /// Submit transaction to blockchain
    pub async fn submit_transaction(&self, transaction: EnergyTransactionEnvelope) -> SystemResult<()> {
        // Validate transaction
        let validator = EnergyTransactionValidator::new();
        let validation_result = validator.validate_transaction(&transaction);
        
        if !matches!(validation_result, TransactionValidationResult::Valid) {
            return Err(crate::utils::error::SystemError::Validation(
                "Transaction validation failed".to_string()
            ));
        }

        // Add to transaction pool
        self.transaction_pool.add_transaction(transaction.clone()).await?;

        crate::utils::logging::log_info(
            "BlockchainEngine",
            &format!("Submitted transaction {}", hex::encode(transaction.hash))
        );

        Ok(())
    }

    /// Deploy smart contract
    pub async fn deploy_contract(
        &self,
        deployer: AccountId,
        code: Vec<u8>,
        abi: ContractABI,
        constructor_args: Vec<u8>,
    ) -> SystemResult<AccountId> {
        let contract_address = self.smart_contract_vm
            .deploy_contract(deployer, code, abi, constructor_args)
            .await?;

        crate::utils::logging::log_info(
            "BlockchainEngine",
            &format!("Deployed contract {}", contract_address)
        );

        Ok(contract_address)
    }

    /// Execute smart contract function
    pub async fn execute_contract_function(
        &self,
        contract_address: &AccountId,
        caller: &AccountId,
        function_name: &str,
        args: &[u8],
        gas_limit: u64,
    ) -> SystemResult<ContractExecutionResult> {
        let result = self.smart_contract_vm
            .execute_contract(
                caller.clone(),
                contract_address.clone(),
                function_name.to_string(),
                args.to_vec(),
                gas_limit,
            )
            .await?;

        crate::utils::logging::log_info(
            "BlockchainEngine",
            &format!("Executed contract function {} on {}", function_name, contract_address)
        );

        Ok(result)
    }
}
