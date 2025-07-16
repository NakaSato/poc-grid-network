//! # Consensus Engine
//! 
//! Implements Proof-of-Work consensus mechanism for energy trading blockchain.

use crate::blockchain::transactions::{EnergyTransactionEnvelope, EnergyBalanceState, EnergyTransaction};
use crate::config::BlockchainConfig;
use crate::types::*;
use crate::utils::SystemResult;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// Proof-of-Work consensus engine
pub struct ConsensusEngine {
    config: BlockchainConfig,
    /// Current blockchain state
    blockchain_state: Arc<RwLock<BlockchainState>>,
    /// Mining difficulty
    difficulty: Arc<RwLock<u32>>,
    /// Block template for mining
    current_block_template: Arc<RwLock<Option<BlockTemplate>>>,
    /// Mining status
    is_mining: Arc<RwLock<bool>>,
}

/// Blockchain state
#[derive(Debug, Clone)]
pub struct BlockchainState {
    /// Current block height
    pub block_height: u32,
    /// Latest block hash
    pub latest_block_hash: Hash,
    /// Account balances
    pub balances: HashMap<AccountId, EnergyBalanceState>,
    /// Pending transactions
    pub pending_transactions: Vec<EnergyTransactionEnvelope>,
    /// Blockchain history
    pub blocks: Vec<EnergyBlock>,
}

/// Energy-focused block structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyBlock {
    /// Block header
    pub header: BlockHeader,
    /// Transactions in this block
    pub transactions: Vec<EnergyTransactionEnvelope>,
    /// Energy statistics for this block
    pub energy_stats: EnergyBlockStats,
}

/// Block header with PoW fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block number
    pub number: u32,
    /// Parent block hash
    pub parent_hash: Hash,
    /// Merkle root of transactions
    pub transaction_root: Hash,
    /// Timestamp
    pub timestamp: u64,
    /// Mining difficulty
    pub difficulty: u32,
    /// Nonce for proof-of-work
    pub nonce: u64,
    /// Block hash
    pub hash: Hash,
    /// Miner account
    pub miner: AccountId,
}

/// Energy statistics for a block
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyBlockStats {
    /// Total energy traded in this block
    pub total_energy_traded: EnergyAmount,
    /// Energy by source
    pub energy_by_source: HashMap<EnergySource, EnergyAmount>,
    /// Number of trades
    pub trade_count: u32,
    /// Average energy price
    pub average_price: TokenPrice,
    /// Total gas used
    pub gas_used: u64,
    /// Carbon credits generated
    pub carbon_credits: u32,
}

/// Block template for mining
#[derive(Debug, Clone)]
pub struct BlockTemplate {
    /// Block header template
    pub header: BlockHeader,
    /// Transactions to include
    pub transactions: Vec<EnergyTransactionEnvelope>,
    /// Target difficulty
    pub target: Hash,
}

impl ConsensusEngine {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        let genesis_block = Self::create_genesis_block();
        let initial_state = BlockchainState {
            block_height: 0,
            latest_block_hash: genesis_block.header.hash.clone(),
            balances: HashMap::new(),
            pending_transactions: Vec::new(),
            blocks: vec![genesis_block],
        };
        
        Ok(Self {
            config: config.clone(),
            blockchain_state: Arc::new(RwLock::new(initial_state)),
            difficulty: Arc::new(RwLock::new(1)), // Start with low difficulty
            current_block_template: Arc::new(RwLock::new(None)),
            is_mining: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("ConsensusEngine", "Starting Proof-of-Work consensus engine");
        
        // Start mining if configured
        if self.config.enable_mining {
            self.start_mining().await?;
        }
        
        // Start block template generation
        self.start_block_template_generation().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("ConsensusEngine", "Stopping consensus engine");
        
        // Stop mining
        *self.is_mining.write().await = false;
        
        Ok(())
    }
    
    /// Start mining process
    async fn start_mining(&self) -> SystemResult<()> {
        let state = self.blockchain_state.clone();
        let difficulty = self.difficulty.clone();
        let is_mining = self.is_mining.clone();
        let template = self.current_block_template.clone();
        
        *is_mining.write().await = true;
        
        tokio::spawn(async move {
            while *is_mining.read().await {
                if let Some(block_template) = template.read().await.as_ref() {
                    if let Some(mined_block) = Self::mine_block(block_template.clone(), *difficulty.read().await).await {
                        // Add mined block to blockchain
                        let mut blockchain_state = state.write().await;
                        blockchain_state.blocks.push(mined_block.clone());
                        blockchain_state.block_height += 1;
                        blockchain_state.latest_block_hash = mined_block.header.hash.clone();
                        
                        // Apply block transactions to state
                        Self::apply_block_transactions(&mut blockchain_state, &mined_block).await;
                        
                        crate::utils::logging::log_info(
                            "ConsensusEngine",
                            &format!("Mined block {} with {} transactions", 
                                     mined_block.header.number, 
                                     mined_block.transactions.len())
                        );
                    }
                }
                
                // Brief pause between mining attempts
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
        
        Ok(())
    }
    
    /// Start block template generation
    async fn start_block_template_generation(&self) -> SystemResult<()> {
        let state = self.blockchain_state.clone();
        let template = self.current_block_template.clone();
        let difficulty = self.difficulty.clone();
        
        tokio::spawn(async move {
            loop {
                // Generate new block template every 5 seconds
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                
                let blockchain_state = state.read().await;
                if !blockchain_state.pending_transactions.is_empty() {
                    let new_template = Self::create_block_template(
                        &blockchain_state,
                        *difficulty.read().await,
                    ).await;
                    
                    *template.write().await = Some(new_template);
                }
            }
        });
        
        Ok(())
    }
    
    /// Create genesis block
    fn create_genesis_block() -> EnergyBlock {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let header = BlockHeader {
            number: 0,
            parent_hash: Hash::default(),
            transaction_root: Hash::default(),
            timestamp,
            difficulty: 1,
            nonce: 0,
            hash: Hash::default(),
            miner: hex::encode([0u8; 32]),
        };
        
        let energy_stats = EnergyBlockStats {
            total_energy_traded: 0.0,
            energy_by_source: HashMap::new(),
            trade_count: 0,
            average_price: 0.0,
            gas_used: 0,
            carbon_credits: 0,
        };
        
        let mut block = EnergyBlock {
            header,
            transactions: Vec::new(),
            energy_stats,
        };
        
        // Calculate genesis block hash
        block.header.hash = Self::calculate_block_hash(&block.header);
        
        block
    }
    
    /// Create block template for mining
    async fn create_block_template(
        state: &BlockchainState,
        difficulty: u32,
    ) -> BlockTemplate {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Select transactions for the block (up to 1000 transactions)
        let transactions = state.pending_transactions.iter()
            .take(1000)
            .cloned()
            .collect::<Vec<_>>();
        
        let transaction_root = Self::calculate_merkle_root(&transactions);
        
        let header = BlockHeader {
            number: state.block_height + 1,
            parent_hash: state.latest_block_hash.clone(),
            transaction_root,
            timestamp,
            difficulty,
            nonce: 0, // Will be set during mining
            hash: Hash::default(), // Will be calculated during mining
            miner: hex::encode([1u8; 32]), // TODO: Use actual miner address
        };
        
        let target = Self::difficulty_to_target(difficulty);
        
        BlockTemplate {
            header,
            transactions,
            target,
        }
    }
    
    /// Mine a block using Proof-of-Work
    async fn mine_block(template: BlockTemplate, difficulty: u32) -> Option<EnergyBlock> {
        let mut header = template.header.clone();
        let target = Self::difficulty_to_target(difficulty);
        
        // Mining loop
        for nonce in 0..u64::MAX {
            header.nonce = nonce;
            let hash = Self::calculate_block_hash(&header);
            
            if Self::hash_meets_target(&hash, &target) {
                header.hash = hash;
                
                // Calculate energy statistics
                let energy_stats = Self::calculate_energy_stats(&template.transactions);
                
                return Some(EnergyBlock {
                    header,
                    transactions: template.transactions,
                    energy_stats,
                });
            }
            
            // Yield periodically to prevent blocking
            if nonce % 10000 == 0 {
                tokio::task::yield_now().await;
            }
        }
        
        None
    }
    
    /// Apply block transactions to blockchain state
    async fn apply_block_transactions(state: &mut BlockchainState, block: &EnergyBlock) {
        for tx in &block.transactions {
            // Remove from pending transactions
            state.pending_transactions.retain(|pending_tx| pending_tx.hash != tx.hash);
            
            // Apply transaction to balances
            Self::apply_transaction_to_state(state, tx).await;
        }
    }
    
    /// Apply single transaction to state
    async fn apply_transaction_to_state(state: &mut BlockchainState, tx: &EnergyTransactionEnvelope) {
        // TODO: Implement full transaction execution
        // For now, just handle basic transfers
        let sender = match &tx.transaction {
            EnergyTransaction::Transfer { from, .. } => from.clone(),
            EnergyTransaction::PlaceOrder { trader, .. } => trader.clone(),
            EnergyTransaction::CancelOrder { trader, .. } => trader.clone(),
            EnergyTransaction::ExecuteTrade { trade, .. } => trade.buyer_id.clone(),
            EnergyTransaction::ReportProduction { producer, .. } => producer.clone(),
            EnergyTransaction::ReportConsumption { consumer, .. } => consumer.clone(),
            EnergyTransaction::GovernanceProposal { proposer, .. } => proposer.clone(),
            EnergyTransaction::Vote { voter, .. } => voter.clone(),
            EnergyTransaction::DeployContract { deployer, .. } => deployer.clone(),
            EnergyTransaction::ExecuteContract { caller, .. } => caller.clone(),
            EnergyTransaction::RegisterProducer { producer, .. } => producer.clone(),
            EnergyTransaction::RegisterConsumer { consumer, .. } => consumer.clone(),
            EnergyTransaction::UpdateGridStatus { authority, .. } => authority.clone(),
            EnergyTransaction::CarbonCredit { issuer, .. } => issuer.clone(),
            EnergyTransaction::EnergyStorage { storage_operator, .. } => storage_operator.clone(),
        };
        let balance_state = state.balances.entry(sender).or_insert_with(EnergyBalanceState::new);
        balance_state.increment_nonce();
    }
    
    /// Calculate block hash
    fn calculate_block_hash(header: &BlockHeader) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&header.number.to_be_bytes());
        data.extend_from_slice(header.parent_hash.as_ref());
        data.extend_from_slice(header.transaction_root.as_ref());
        data.extend_from_slice(&header.timestamp.to_be_bytes());
        data.extend_from_slice(&header.difficulty.to_be_bytes());
        data.extend_from_slice(&header.nonce.to_be_bytes());
        data.extend_from_slice(header.miner.as_ref());
        
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash_result = hasher.finalize();
        hex::encode(hash_result)
    }
    
    /// Calculate merkle root of transactions
    fn calculate_merkle_root(transactions: &[EnergyTransactionEnvelope]) -> Hash {
        if transactions.is_empty() {
            return Hash::default();
        }
        
        let mut hashes: Vec<Hash> = transactions.iter()
            .map(|tx| hex::encode(tx.hash))
            .collect();
        
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    let mut data = Vec::new();
                    data.extend_from_slice(chunk[0].as_bytes());
                    data.extend_from_slice(chunk[1].as_bytes());
                    let mut hasher = Sha256::new();
                    hasher.update(&data);
                    let result = hasher.finalize();
                    hex::encode(&result[..32])
                } else {
                    chunk[0].clone()
                };
                next_level.push(combined);
            }
            
            hashes = next_level;
        }
        
        hashes.into_iter().next().unwrap_or_default()
    }
    
    /// Convert difficulty to target hash
    fn difficulty_to_target(difficulty: u32) -> Hash {
        let mut target = [0xFFu8; 32];
        
        // Simple difficulty adjustment: more leading zeros = higher difficulty
        let leading_zeros = difficulty.min(32);
        for i in 0..leading_zeros as usize {
            target[i] = 0;
        }
        
        hex::encode(target)
    }
    
    /// Check if hash meets target difficulty
    fn hash_meets_target(hash: &Hash, target: &Hash) -> bool {
        hash.as_bytes() <= target.as_bytes()
    }
    
    /// Calculate energy statistics for a block
    fn calculate_energy_stats(transactions: &[EnergyTransactionEnvelope]) -> EnergyBlockStats {
        let mut total_energy_traded = 0.0f64;
        let mut energy_by_source: HashMap<EnergySource, f64> = HashMap::new();
        let mut trade_count = 0;
        let mut total_price = 0u128;
        let mut gas_used = 0;
        let mut carbon_credits = 0u32;
        
        for tx in transactions {
            gas_used += tx.get_gas_cost();
            
            match &tx.transaction {
                crate::blockchain::transactions::EnergyTransaction::ExecuteTrade { trade, .. } => {
                    total_energy_traded += trade.energy_amount;
                    *energy_by_source.entry(trade.energy_source.clone()).or_insert(0.0) += trade.energy_amount;
                    trade_count += 1;
                    total_price += trade.total_price;
                    carbon_credits += trade.carbon_offset.offset_credits as u32;
                }
                crate::blockchain::transactions::                EnergyTransaction::ReportProduction { production_record, .. } => {
                    *energy_by_source.entry(production_record.energy_type.clone()).or_insert(0.0) += production_record.amount;
                }
                _ => {}
            }
        }
        
        let average_price = if trade_count > 0 {
            (total_price / trade_count as u128) as TokenPrice
        } else {
            0.0
        };
        
        EnergyBlockStats {
            total_energy_traded,
            energy_by_source,
            trade_count,
            average_price,
            gas_used,
            carbon_credits,
        }
    }
    
    /// Add transaction to pending pool
    pub async fn add_transaction(&self, tx: EnergyTransactionEnvelope) -> SystemResult<()> {
        let mut state = self.blockchain_state.write().await;
        state.pending_transactions.push(tx);
        Ok(())
    }
    
    /// Get current blockchain state
    pub async fn get_blockchain_state(&self) -> BlockchainState {
        self.blockchain_state.read().await.clone()
    }
    
    /// Get account balance
    pub async fn get_account_balance(&self, account: &AccountId) -> Option<EnergyBalanceState> {
        let state = self.blockchain_state.read().await;
        state.balances.get(account).cloned()
    }
    
    /// Adjust mining difficulty
    pub async fn adjust_difficulty(&self, new_difficulty: u32) {
        *self.difficulty.write().await = new_difficulty;
        crate::utils::logging::log_info(
            "ConsensusEngine",
            &format!("Adjusted mining difficulty to {}", new_difficulty)
        );
    }
    
    /// Start validator process (for compatibility)
    async fn start_validator(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("ConsensusEngine", "Starting validator node");
        Ok(())
    }
}
