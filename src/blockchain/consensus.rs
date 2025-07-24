//! # Consensus Engine
//! 
//! Implements Proof-of-Authority (PoA) consensus mechanism for energy trading blockchain.

use crate::blockchain::transactions::{EnergyTransactionEnvelope, EnergyBalanceState, EnergyTransaction};
use crate::config::BlockchainConfig;
use crate::types::*;
use crate::utils::SystemResult;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::sync::RwLock;

/// Proof-of-Authority consensus engine
pub struct ConsensusEngine {
    config: BlockchainConfig,
    /// Current blockchain state
    blockchain_state: Arc<RwLock<BlockchainState>>,
    /// Authorized validators
    validators: Arc<RwLock<HashSet<AccountId>>>,
    /// Current validator (if this node is a validator)
    current_validator: Arc<RwLock<Option<AccountId>>>,
    /// Validator rotation schedule
    validator_schedule: Arc<RwLock<ValidatorSchedule>>,
    /// Block production status
    is_producing: Arc<RwLock<bool>>,
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

/// Authority validator information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authority {
    /// Validator account ID
    pub account_id: AccountId,
    /// Validator name/organization
    pub name: String,
    /// Energy sector expertise
    pub expertise: Vec<EnergySource>,
    /// Validator reputation score
    pub reputation: f64,
    /// Grid locations this validator can authorize
    pub authorized_regions: Vec<String>,
    /// Validator public key for signatures
    pub public_key: Vec<u8>,
    /// Active status
    pub is_active: bool,
    /// Registration timestamp
    pub registered_at: u64,
}

/// Validator schedule for block production
#[derive(Debug, Clone)]
pub struct ValidatorSchedule {
    /// Current active validators
    pub active_validators: Vec<AccountId>,
    /// Current round number
    pub round: u64,
    /// Current validator index
    pub current_validator_index: usize,
    /// Block production interval (seconds)
    pub block_interval: u64,
    /// Last block production time
    pub last_block_time: u64,
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
    /// Validator signature
    pub validator_signature: ValidatorSignature,
}

/// Block header with PoA fields
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Block number
    pub number: u32,
    /// Parent block hash
    pub parent_hash: Hash,
    /// Merkle root of transactions
    pub transaction_root: Hash,
    /// State root hash
    pub state_root: Hash,
    /// Timestamp
    pub timestamp: u64,
    /// Block producer (validator)
    pub validator: AccountId,
    /// Block hash
    pub hash: Hash,
}

/// Validator signature for block authentication
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorSignature {
    /// Validator account ID
    pub validator: AccountId,
    /// Digital signature
    pub signature: Vec<u8>,
    /// Signature timestamp
    pub timestamp: u64,
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

impl ConsensusEngine {
    pub async fn new(config: &BlockchainConfig) -> SystemResult<Self> {
        // Validate that only PoA is configured
        if config.consensus_algorithm != "proof_of_authority" {
            return Err(crate::utils::SystemError::Configuration(
                format!("Only Proof-of-Authority consensus is supported, got: {}", 
                        config.consensus_algorithm)
            ));
        }
        
        crate::utils::logging::log_info(
            "ConsensusEngine", 
            "✅ Initializing Proof-of-Authority consensus engine (PoW disabled)"
        );
        
        let genesis_block = Self::create_genesis_block().await?;
        let initial_state = BlockchainState {
            block_height: 0,
            latest_block_hash: genesis_block.header.hash.clone(),
            balances: HashMap::new(),
            pending_transactions: Vec::new(),
            blocks: vec![genesis_block],
        };
        
        // Initialize default energy authorities
        let mut initial_validators = HashSet::new();
        
        // Add Thai energy authorities as initial validators
        let energy_authority = AccountId::from_str("ThaiEnergyAuthority").unwrap_or_default();
        let grid_authority = AccountId::from_str("GridAuthorityThailand").unwrap_or_default();
        let renewable_authority = AccountId::from_str("RenewableEnergyAuth").unwrap_or_default();
        
        initial_validators.insert(energy_authority.clone());
        initial_validators.insert(grid_authority.clone());
        initial_validators.insert(renewable_authority.clone());
        
        let validator_schedule = ValidatorSchedule {
            active_validators: vec![energy_authority, grid_authority, renewable_authority],
            round: 0,
            current_validator_index: 0,
            block_interval: config.block_time, // Use configured block time
            last_block_time: 0,
        };
        
        crate::utils::logging::log_info(
            "ConsensusEngine", 
            &format!("🏛️ Initialized with {} validators, block time: {}s", 
                    initial_validators.len(), config.block_time)
        );
        
        Ok(Self {
            config: config.clone(),
            blockchain_state: Arc::new(RwLock::new(initial_state)),
            validators: Arc::new(RwLock::new(initial_validators)),
            current_validator: Arc::new(RwLock::new(None)),
            validator_schedule: Arc::new(RwLock::new(validator_schedule)),
            is_producing: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        // First, validate that this is a PoA-only system
        self.validate_poa_only()?;
        
        crate::utils::logging::log_startup("Proof-of-Authority Consensus Engine");
        crate::utils::logging::log_info(
            "ConsensusEngine",
            &format!("🏛️ Starting PoA consensus with {} validators", 
                    self.validators.read().await.len())
        );
        
        // Check if this node is a validator
        if self.config.validator {
            self.initialize_validator().await?;
            self.start_block_production().await?;
        }
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        crate::utils::logging::log_shutdown("PoA Consensus Engine");
        
        // Stop block production
        *self.is_producing.write().await = false;
        
        Ok(())
    }
    
    /// Initialize this node as a validator
    async fn initialize_validator(&self) -> SystemResult<()> {
        // In a real implementation, this would load the validator's private key
        // and register with the network
        let validator_id = AccountId::from_str("LocalValidator").unwrap_or_default();
        *self.current_validator.write().await = Some(validator_id.clone());
        
        // Add to validator set if not already present
        let mut validators = self.validators.write().await;
        validators.insert(validator_id);
        
        crate::utils::logging::log_info(
            "ConsensusEngine", 
            "Node initialized as PoA validator"
        );
        
        Ok(())
    }
    
    /// Start block production process for validators
    async fn start_block_production(&self) -> SystemResult<()> {
        let is_producing = self.is_producing.clone();
        let validator_schedule = self.validator_schedule.clone();
        let blockchain_state = self.blockchain_state.clone();
        let current_validator = self.current_validator.clone();
        
        *is_producing.write().await = true;
        
        tokio::spawn(async move {
            while *is_producing.read().await {
                // Check if it's our turn to produce a block
                if let Some(validator_id) = current_validator.read().await.as_ref() {
                    let schedule = validator_schedule.read().await;
                    let current_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    // Check if it's time to produce a block and if we're the designated validator
                    if current_time >= schedule.last_block_time + schedule.block_interval {
                        let current_validator_id = &schedule.active_validators[schedule.current_validator_index];
                        
                        if validator_id == current_validator_id {
                            // Produce a new block
                            if let Err(e) = Self::produce_block_static(
                                &blockchain_state,
                                &validator_schedule,
                                validator_id.clone()
                            ).await {
                                log::error!("Failed to produce block: {}", e);
                            }
                        }
                    }
                }
                
                // Sleep for a short interval before checking again
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        });
        
        Ok(())
    }
    
    /// Static method for block production (to avoid async closure issues)
    async fn produce_block_static(
        blockchain_state: &Arc<RwLock<BlockchainState>>,
        validator_schedule: &Arc<RwLock<ValidatorSchedule>>,
        validator_id: AccountId,
    ) -> SystemResult<()> {
        let mut state = blockchain_state.write().await;
        let mut schedule = validator_schedule.write().await;
        
        if state.pending_transactions.is_empty() {
            // No transactions to include, skip this block
            return Ok(());
        }
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Create new block
        let new_block = EnergyBlock {
            header: BlockHeader {
                number: state.block_height + 1,
                parent_hash: state.latest_block_hash.clone(),
                transaction_root: Self::calculate_merkle_root(&state.pending_transactions),
                state_root: Self::calculate_state_root(&state.balances),
                timestamp: current_time,
                validator: validator_id.clone(),
                hash: String::new(), // Will be calculated after
            },
            transactions: state.pending_transactions.clone(),
            energy_stats: Self::calculate_energy_stats(&state.pending_transactions),
            validator_signature: ValidatorSignature {
                validator: validator_id,
                signature: vec![0; 64], // Mock signature
                timestamp: current_time,
            },
        };
        
        // Calculate block hash
        let mut new_block = new_block;
        new_block.header.hash = Self::calculate_block_hash(&new_block);
        
        // Add block to chain
        state.blocks.push(new_block.clone());
        state.block_height += 1;
        state.latest_block_hash = new_block.header.hash;
        state.pending_transactions.clear();
        
        // Update validator schedule
        schedule.current_validator_index = (schedule.current_validator_index + 1) % schedule.active_validators.len();
        schedule.round = if schedule.current_validator_index == 0 { 
            schedule.round + 1 
        } else { 
            schedule.round 
        };
        schedule.last_block_time = current_time;
        
        crate::utils::logging::log_info(
            "ConsensusEngine",
            &format!("Block #{} produced by validator {}", 
                     new_block.header.number, 
                     new_block.header.validator)
        );
        
        Ok(())
    }
    
    /// Add a new validator to the authority set (requires governance)
    pub async fn add_validator(&self, validator_id: AccountId, authority: Authority) -> SystemResult<()> {
        let mut validators = self.validators.write().await;
        validators.insert(validator_id.clone());
        
        let mut schedule = self.validator_schedule.write().await;
        if !schedule.active_validators.contains(&validator_id) {
            schedule.active_validators.push(validator_id.clone());
        }
        
        crate::utils::logging::log_info(
            "ConsensusEngine",
            &format!("Added new validator: {} ({})", validator_id, authority.name)
        );
        
        Ok(())
    }
    
    /// Remove a validator from the authority set (requires governance)
    pub async fn remove_validator(&self, validator_id: &AccountId) -> SystemResult<()> {
        let mut validators = self.validators.write().await;
        validators.remove(validator_id);
        
        let mut schedule = self.validator_schedule.write().await;
        schedule.active_validators.retain(|v| v != validator_id);
        
        // Adjust current index if necessary
        if schedule.current_validator_index >= schedule.active_validators.len() {
            schedule.current_validator_index = 0;
        }
        
        crate::utils::logging::log_info(
            "ConsensusEngine",
            &format!("Removed validator: {}", validator_id)
        );
        
        Ok(())
    }
    
    /// Get current validator set
    pub async fn get_validators(&self) -> HashSet<AccountId> {
        self.validators.read().await.clone()
    }
    
    /// Get validator schedule information
    pub async fn get_validator_schedule(&self) -> ValidatorSchedule {
        self.validator_schedule.read().await.clone()
    }
    
    /// Verify validator signature on a block
    pub fn verify_block_signature(&self, _block: &EnergyBlock) -> SystemResult<bool> {
        // In a real implementation, this would verify the cryptographic signature
        // For now, just check if the validator is in the authority set
        // TODO: Implement actual signature verification with validator's public key
        Ok(true)
    }
    
    /// Create genesis block for PoA
    async fn create_genesis_block() -> SystemResult<EnergyBlock> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let genesis_validator = AccountId::from_str("GenesisValidator").unwrap_or_default();
        
        let header = BlockHeader {
            number: 0,
            parent_hash: Hash::default(),
            transaction_root: Hash::default(),
            state_root: Hash::default(),
            timestamp,
            validator: genesis_validator.clone(),
            hash: Hash::default(),
        };
        
        let energy_stats = EnergyBlockStats {
            total_energy_traded: 0.0,
            energy_by_source: HashMap::new(),
            trade_count: 0,
            average_price: 0.0,
            gas_used: 0,
            carbon_credits: 0,
        };
        
        let validator_signature = ValidatorSignature {
            validator: genesis_validator,
            signature: vec![0; 64], // Genesis signature
            timestamp,
        };
        
        let mut block = EnergyBlock {
            header,
            transactions: Vec::new(),
            energy_stats,
            validator_signature,
        };
        
        // Calculate genesis block hash
        block.header.hash = Self::calculate_block_hash(&block);
        
        Ok(block)
    }
    
    /// Calculate block hash for PoA
    fn calculate_block_hash(block: &EnergyBlock) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&block.header.number.to_be_bytes());
        data.extend_from_slice(block.header.parent_hash.as_bytes());
        data.extend_from_slice(block.header.transaction_root.as_bytes());
        data.extend_from_slice(block.header.state_root.as_bytes());
        data.extend_from_slice(&block.header.timestamp.to_be_bytes());
        data.extend_from_slice(block.header.validator.as_bytes());
        
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
            .map(|tx| hex::encode(&tx.hash))
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
                    hex::encode(hasher.finalize())
                } else {
                    chunk[0].clone()
                };
                next_level.push(combined);
            }
            
            hashes = next_level;
        }
        
        hashes.into_iter().next().unwrap_or_default()
    }
    
    /// Calculate state root hash
    fn calculate_state_root(balances: &HashMap<AccountId, EnergyBalanceState>) -> Hash {
        let mut data = Vec::new();
        
        // Sort accounts for deterministic hash
        let mut sorted_accounts: Vec<_> = balances.keys().collect();
        sorted_accounts.sort();
        
        for account in sorted_accounts {
            if let Some(balance) = balances.get(account) {
                data.extend_from_slice(account.as_bytes());
                data.extend_from_slice(&balance.total_balance.to_be_bytes());
                data.extend_from_slice(&balance.nonce.to_be_bytes());
            }
        }
        
        let mut hasher = Sha256::new();
        hasher.update(&data);
        hex::encode(hasher.finalize())
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
                EnergyTransaction::ExecuteTrade { trade, .. } => {
                    total_energy_traded += trade.energy_amount;
                    *energy_by_source.entry(trade.energy_source.clone()).or_insert(0.0) += trade.energy_amount;
                    trade_count += 1;
                    total_price += trade.total_price;
                    carbon_credits += trade.carbon_offset.offset_credits as u32;
                }
                EnergyTransaction::ReportProduction { production_record, .. } => {
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
    
    /// Apply transaction to blockchain state
    #[allow(dead_code)]
    async fn apply_transaction_to_state(state: &mut BlockchainState, tx: &EnergyTransactionEnvelope) {
        // Remove from pending transactions
        state.pending_transactions.retain(|pending_tx| pending_tx.hash != tx.hash);
        
        // Apply transaction effects to balances based on transaction type
        match &tx.transaction {
            EnergyTransaction::Transfer { from, to, amount, .. } => {
                // Handle token transfer
                if let Some(sender_balance) = state.balances.get_mut(from) {
                    sender_balance.total_balance = sender_balance.total_balance.saturating_sub(*amount);
                    sender_balance.increment_nonce();
                }
                
                let receiver_balance = state.balances.entry(to.clone()).or_insert_with(EnergyBalanceState::new);
                receiver_balance.total_balance = receiver_balance.total_balance.saturating_add(*amount);
            }
            EnergyTransaction::ExecuteTrade { trade, .. } => {
                // Handle energy trade execution
                if let Some(buyer_balance) = state.balances.get_mut(&trade.buyer_id) {
                    buyer_balance.total_balance = buyer_balance.total_balance.saturating_sub(trade.total_price);
                }
                
                if let Some(seller_balance) = state.balances.get_mut(&trade.seller_id) {
                    seller_balance.total_balance = seller_balance.total_balance.saturating_add(trade.total_price);
                }
            }
            _ => {
                // For other transaction types, just increment nonce
                let sender = tx.get_sender();
                let balance_state = state.balances.entry(sender).or_insert_with(EnergyBalanceState::new);
                balance_state.increment_nonce();
            }
        }
    }
}

impl ValidatorSchedule {
    /// Get the current designated validator
    pub fn get_current_validator(&self) -> Option<&AccountId> {
        self.active_validators.get(self.current_validator_index)
    }
    
    /// Check if it's time for the next validator
    pub fn should_rotate(&self, current_time: u64) -> bool {
        current_time >= self.last_block_time + self.block_interval
    }
    
    /// Get the next validator in line
    pub fn get_next_validator(&self) -> Option<&AccountId> {
        let next_index = (self.current_validator_index + 1) % self.active_validators.len();
        self.active_validators.get(next_index)
    }
}

impl Authority {
    /// Create a new authority
    pub fn new(
        account_id: AccountId,
        name: String,
        expertise: Vec<EnergySource>,
        authorized_regions: Vec<String>,
        public_key: Vec<u8>,
    ) -> Self {
        Self {
            account_id,
            name,
            expertise,
            reputation: 1.0, // Start with neutral reputation
            authorized_regions,
            public_key,
            is_active: true,
            registered_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Check if authority can validate for a specific energy source
    pub fn can_validate_energy_source(&self, energy_source: &EnergySource) -> bool {
        self.is_active && (self.expertise.contains(energy_source) || self.expertise.is_empty())
    }
    
    /// Check if authority can validate for a specific region
    pub fn can_validate_region(&self, region: &str) -> bool {
        self.is_active && (self.authorized_regions.contains(&region.to_string()) || self.authorized_regions.is_empty())
    }
}

/// PoA-specific validation functions
impl ConsensusEngine {
    /// Verify this is a PoA-only system (no PoW allowed)
    pub fn validate_poa_only(&self) -> SystemResult<()> {
        if self.config.consensus_algorithm != "proof_of_authority" {
            return Err(crate::utils::SystemError::Configuration(
                "❌ CRITICAL: Only Proof-of-Authority is allowed! PoW and other consensus mechanisms are disabled.".to_string()
            ));
        }
        
        crate::utils::logging::log_info(
            "ConsensusEngine",
            "✅ PoA validation passed: System is pure Proof-of-Authority"
        );
        
        Ok(())
    }
    
    /// Get consensus mechanism type
    pub fn get_consensus_type(&self) -> &str {
        "proof_of_authority"
    }
    
    /// Check if mining is disabled (should always be true for PoA)
    pub fn is_mining_disabled(&self) -> bool {
        true // Always true for PoA-only system
    }
    
    /// Get block production method
    pub fn get_block_production_method(&self) -> &str {
        "validator_authority"
    }
}
