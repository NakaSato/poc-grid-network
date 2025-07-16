//! # Energy-Focused Transaction Types
//! 
//! This module defines blockchain transaction types specifically for energy trading,
//! including advanced energy balance tracking and smart contract support.

use crate::types::*;
use crate::utils::SystemResult;
use serde::{Deserialize, Serialize};
use sp_core::crypto::AccountId32;
use sp_runtime::traits::{BlakeTwo256, Hash};
use std::collections::HashMap;
use std::time::SystemTime;

/// Energy-specific transaction types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnergyTransaction {
    /// Transfer energy tokens between accounts
    Transfer {
        from: AccountId32,
        to: AccountId32,
        amount: Balance,
        energy_type: EnergySource,
        grid_location: GridLocation,
    },
    /// Place an energy order on the trading platform
    PlaceOrder {
        trader: AccountId32,
        order: EnergyOrder,
        collateral: Balance,
        signature: Vec<u8>,
    },
    /// Cancel an existing energy order
    CancelOrder {
        trader: AccountId32,
        order_id: uuid::Uuid,
        signature: Vec<u8>,
    },
    /// Execute a matched energy trade
    ExecuteTrade {
        trade: EnergyTrade,
        buyer_signature: Vec<u8>,
        seller_signature: Vec<u8>,
        grid_validation: GridValidation,
    },
    /// Report energy production to the grid
    ReportProduction {
        producer: AccountId32,
        production_record: EnergyProductionRecord,
        validator_signatures: Vec<ValidatorSignature>,
    },
    /// Report energy consumption
    ReportConsumption {
        consumer: AccountId32,
        consumption_record: EnergyConsumptionRecord,
        meter_signature: Vec<u8>,
    },
    /// Governance proposal transaction
    GovernanceProposal {
        proposer: AccountId32,
        proposal: GovernanceProposal,
        stake_amount: Balance,
        signature: Vec<u8>,
    },
    /// Vote on a governance proposal
    Vote {
        voter: AccountId32,
        proposal_id: uuid::Uuid,
        choice: VoteChoice,
        voting_power: Balance,
        signature: Vec<u8>,
    },
    /// Smart contract deployment
    DeployContract {
        deployer: AccountId32,
        contract_code: Vec<u8>,
        constructor_args: Vec<u8>,
        gas_limit: u64,
        signature: Vec<u8>,
    },
    /// Smart contract execution
    ExecuteContract {
        caller: AccountId32,
        contract_address: AccountId32,
        method: String,
        args: Vec<u8>,
        gas_limit: u64,
        signature: Vec<u8>,
    },
    /// Register as energy producer
    RegisterProducer {
        producer: AccountId32,
        location: GridLocation,
        energy_types: Vec<EnergySource>,
        capacity: EnergyAmount,
        certification: ProducerCertification,
        signature: Vec<u8>,
    },
    /// Register as energy consumer
    RegisterConsumer {
        consumer: AccountId32,
        location: GridLocation,
        consumption_pattern: ConsumptionPattern,
        signature: Vec<u8>,
    },
    /// Update grid status
    UpdateGridStatus {
        authority: AccountId32,
        location: GridLocation,
        status: GridStatus,
        signature: Vec<u8>,
    },
    /// Carbon credit transaction
    CarbonCredit {
        issuer: AccountId32,
        recipient: AccountId32,
        credits: u32,
        energy_source: EnergySource,
        verification: CarbonVerification,
        signature: Vec<u8>,
    },
    /// Energy storage transaction
    EnergyStorage {
        storage_operator: AccountId32,
        action: StorageAction,
        amount: EnergyAmount,
        location: GridLocation,
        signature: Vec<u8>,
    },
}

/// Vote choice for governance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

/// Grid validation for energy trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridValidation {
    pub validator: AccountId32,
    pub capacity_check: bool,
    pub congestion_level: CongestionLevel,
    pub transmission_cost: Balance,
    pub timestamp: SystemTime,
}

/// Validator signature for production records
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorSignature {
    pub validator: AccountId32,
    pub signature: Vec<u8>,
    pub timestamp: SystemTime,
}

/// Producer certification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProducerCertification {
    pub certificate_id: String,
    pub issuing_authority: String,
    pub valid_until: SystemTime,
    pub renewable_percentage: f32,
    pub efficiency_rating: f32,
}

/// Consumer consumption pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionPattern {
    pub peak_hours: Vec<u8>, // Hours of the day (0-23)
    pub average_consumption: EnergyAmount,
    pub max_consumption: EnergyAmount,
    pub preferred_sources: Vec<EnergySource>,
}

/// Carbon verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarbonVerification {
    pub verifier: AccountId32,
    pub methodology: String,
    pub co2_reduced: f64, // in tons
    pub verification_standard: String,
    pub timestamp: SystemTime,
}

/// Energy storage actions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StorageAction {
    Store { duration: u64 }, // seconds
    Release,
    Reserve { duration: u64 },
}

/// Transaction with full metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyTransactionEnvelope {
    pub transaction: EnergyTransaction,
    pub nonce: u64,
    pub gas_price: Balance,
    pub gas_limit: u64,
    pub timestamp: SystemTime,
    pub hash: [u8; 32],
}

/// Energy production record with detailed validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyProductionRecord {
    pub amount: EnergyAmount,
    pub energy_type: EnergySource,
    pub location: GridLocation,
    pub timestamp: SystemTime,
    pub verified: bool,
    pub efficiency: f32,
    pub weather_conditions: Option<WeatherConditions>,
    pub equipment_id: String,
    pub quality_metrics: EnergyQualityMetrics,
}

/// Energy consumption record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyConsumptionRecord {
    pub amount: EnergyAmount,
    pub location: GridLocation,
    pub timestamp: SystemTime,
    pub verified: bool,
    pub consumer_type: ConsumerType,
    pub appliance_breakdown: HashMap<String, EnergyAmount>,
}

/// Weather conditions affecting energy production
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherConditions {
    pub temperature: f32,
    pub humidity: f32,
    pub wind_speed: f32,
    pub solar_irradiance: f32,
    pub cloud_cover: f32,
}

/// Energy quality metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyQualityMetrics {
    pub voltage: f32,
    pub frequency: f32,
    pub power_factor: f32,
    pub harmonic_distortion: f32,
}

/// Consumer type classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumerType {
    Residential,
    Commercial,
    Industrial,
    Agricultural,
    Municipal,
}

/// Advanced energy balance state with detailed tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyBalanceState {
    /// Total balance across all energy types
    pub total_balance: Balance,
    /// Balance by energy source
    pub energy_balances: HashMap<EnergySource, Balance>,
    /// Locked balances (for orders)
    pub locked_balances: HashMap<EnergySource, Balance>,
    /// Staked balances (for governance)
    pub staked_balance: Balance,
    /// Production history
    pub production_history: Vec<EnergyProductionRecord>,
    /// Consumption history
    pub consumption_history: Vec<EnergyConsumptionRecord>,
    /// Carbon credits
    pub carbon_credits: u32,
    /// Account nonce
    pub nonce: u64,
    /// Last update timestamp
    pub last_updated: SystemTime,
}

impl EnergyBalanceState {
    /// Create new empty balance state
    pub fn new() -> Self {
        Self {
            total_balance: 0,
            energy_balances: HashMap::new(),
            locked_balances: HashMap::new(),
            staked_balance: 0,
            production_history: Vec::new(),
            consumption_history: Vec::new(),
            carbon_credits: 0,
            nonce: 0,
            last_updated: SystemTime::now(),
        }
    }

    /// Get available balance for a specific energy type
    pub fn get_available_balance(&self, energy_type: &EnergySource) -> Balance {
        let total = self.energy_balances.get(energy_type).copied().unwrap_or(0);
        let locked = self.locked_balances.get(energy_type).copied().unwrap_or(0);
        total.saturating_sub(locked)
    }

    /// Add energy balance from production
    pub fn add_production(&mut self, production: EnergyProductionRecord) {
        let amount = production.amount as Balance;
        *self.energy_balances.entry(production.energy_type.clone()).or_insert(0) += amount;
        self.total_balance += amount;
        self.production_history.push(production);
        self.last_updated = SystemTime::now();
    }

    /// Remove energy balance from consumption
    pub fn add_consumption(&mut self, consumption: EnergyConsumptionRecord) {
        let amount = consumption.amount as Balance;
        // Consume from mixed energy first, then specific types
        let available_mixed = self.get_available_balance(&EnergySource::Mixed);
        if available_mixed >= amount {
            *self.energy_balances.entry(EnergySource::Mixed).or_insert(0) -= amount;
        } else {
            // Consume from available energy types proportionally
            let remaining = amount - available_mixed;
            if available_mixed > 0 {
                *self.energy_balances.entry(EnergySource::Mixed).or_insert(0) -= available_mixed;
            }
            
            // Distribute remaining consumption across other energy types
            let total_available: Balance = self.energy_balances.iter()
                .filter(|(k, _)| **k != EnergySource::Mixed)
                .map(|(_, v)| *v)
                .sum();
            
            if total_available > 0 {
                for (energy_type, balance) in self.energy_balances.iter_mut() {
                    if *energy_type != EnergySource::Mixed && *balance > 0 {
                        let consumption_share = ((*balance as f64 / total_available as f64) * remaining as f64) as Balance;
                        *balance = balance.saturating_sub(consumption_share);
                    }
                }
            }
        }
        
        self.total_balance = self.total_balance.saturating_sub(amount);
        self.consumption_history.push(consumption);
        self.last_updated = SystemTime::now();
    }

    /// Transfer energy to another account
    pub fn transfer(&mut self, energy_type: EnergySource, amount: Balance) -> SystemResult<()> {
        let available = self.get_available_balance(&energy_type);
        if available < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Insufficient balance for transfer".to_string()
            ));
        }

        *self.energy_balances.entry(energy_type).or_insert(0) -= amount;
        self.total_balance = self.total_balance.saturating_sub(amount);
        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Receive energy from another account
    pub fn receive(&mut self, energy_type: EnergySource, amount: Balance) {
        *self.energy_balances.entry(energy_type).or_insert(0) += amount;
        self.total_balance += amount;
        self.last_updated = SystemTime::now();
    }

    /// Lock balance for trading order
    pub fn lock_balance(&mut self, energy_type: EnergySource, amount: Balance) -> SystemResult<()> {
        let available = self.get_available_balance(&energy_type);
        if available < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Insufficient balance to lock".to_string()
            ));
        }

        *self.locked_balances.entry(energy_type).or_insert(0) += amount;
        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Unlock balance from cancelled order
    pub fn unlock_balance(&mut self, energy_type: EnergySource, amount: Balance) -> SystemResult<()> {
        let locked = self.locked_balances.get_mut(&energy_type).unwrap_or(&mut 0);
        if *locked < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Cannot unlock more than locked amount".to_string()
            ));
        }

        *locked -= amount;
        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Stake balance for governance participation
    pub fn stake(&mut self, amount: Balance) -> SystemResult<()> {
        if self.get_available_balance(&EnergySource::Mixed) < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Insufficient balance to stake".to_string()
            ));
        }

        *self.energy_balances.entry(EnergySource::Mixed).or_insert(0) -= amount;
        self.staked_balance += amount;
        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Unstake balance
    pub fn unstake(&mut self, amount: Balance) -> SystemResult<()> {
        if self.staked_balance < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Cannot unstake more than staked amount".to_string()
            ));
        }

        self.staked_balance -= amount;
        *self.energy_balances.entry(EnergySource::Mixed).or_insert(0) += amount;
        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Add carbon credits
    pub fn add_carbon_credits(&mut self, credits: u32) {
        self.carbon_credits += credits;
        self.last_updated = SystemTime::now();
    }

    /// Increment nonce
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
        self.last_updated = SystemTime::now();
    }

    /// Get total production in a time period
    pub fn get_production_in_period(&self, start: SystemTime, end: SystemTime) -> EnergyAmount {
        self.production_history
            .iter()
            .filter(|p| p.timestamp >= start && p.timestamp <= end)
            .map(|p| p.amount)
            .sum()
    }

    /// Get total consumption in a time period
    pub fn get_consumption_in_period(&self, start: SystemTime, end: SystemTime) -> EnergyAmount {
        self.consumption_history
            .iter()
            .filter(|c| c.timestamp >= start && c.timestamp <= end)
            .map(|c| c.amount)
            .sum()
    }

    /// Calculate energy efficiency ratio
    pub fn calculate_efficiency_ratio(&self) -> f64 {
        let total_production: EnergyAmount = self.production_history.iter().map(|p| p.amount).sum();
        let total_consumption: EnergyAmount = self.consumption_history.iter().map(|c| c.amount).sum();
        
        if total_consumption == 0 {
            return 0.0;
        }
        
        total_production as f64 / total_consumption as f64
    }

    /// Get renewable energy percentage
    pub fn get_renewable_percentage(&self) -> f64 {
        let total_balance = self.total_balance as f64;
        if total_balance == 0.0 {
            return 0.0;
        }

        let renewable_balance: Balance = self.energy_balances
            .iter()
            .filter(|(energy_type, _)| matches!(energy_type, 
                EnergySource::Solar | EnergySource::Wind | EnergySource::Hydro | EnergySource::Biomass))
            .map(|(_, balance)| *balance)
            .sum();

        (renewable_balance as f64 / total_balance) * 100.0
    }
}

impl EnergyTransactionEnvelope {
    /// Create new transaction envelope
    pub fn new(transaction: EnergyTransaction, nonce: u64) -> Self {
        let serialized = bincode::serialize(&transaction).unwrap_or_default();
        let hash = BlakeTwo256::hash(&serialized);

        Self {
            transaction,
            nonce,
            gas_price: 1000, // Default gas price
            gas_limit: 100000, // Default gas limit
            timestamp: SystemTime::now(),
            hash: hash.into(),
        }
    }

    /// Verify transaction signature
    pub fn verify_signature(&self) -> SystemResult<bool> {
        // In a real implementation, this would verify the cryptographic signature
        // For now, we'll just check if the transaction is well-formed
        match &self.transaction {
            EnergyTransaction::Transfer { from, to, amount, .. } => {
                Ok(from != to && *amount > 0)
            }
            EnergyTransaction::PlaceOrder { order, .. } => {
                Ok(order.energy_amount > 0 && order.price_per_kwh > 0)
            }
            _ => Ok(true),
        }
    }

    /// Calculate transaction fee based on gas usage
    pub fn calculate_fee(&self) -> Balance {
        self.gas_price * self.gas_limit
    }

    /// Get transaction weight for block inclusion priority
    pub fn get_weight(&self) -> u64 {
        match &self.transaction {
            EnergyTransaction::Transfer { .. } => 1000,
            EnergyTransaction::PlaceOrder { .. } => 2000,
            EnergyTransaction::ExecuteTrade { .. } => 3000,
            EnergyTransaction::ReportProduction { .. } => 1500,
            EnergyTransaction::ReportConsumption { .. } => 1500,
            EnergyTransaction::DeployContract { .. } => 10000,
            EnergyTransaction::ExecuteContract { .. } => 5000,
            _ => 1000,
        }
    }
}
            nonce,
            gas_limit,
            gas_price,
            timestamp,
            hash,
        }
    }
    
    /// Calculate transaction hash
    fn calculate_hash(
        transaction: &EnergyTransaction,
        nonce: u64,
        gas_limit: u64,
        gas_price: Balance,
        timestamp: std::time::SystemTime,
    ) -> Hash {
        let mut data = Vec::new();
        data.extend_from_slice(&bincode::serialize(transaction).unwrap_or_default());
        data.extend_from_slice(&nonce.to_be_bytes());
        data.extend_from_slice(&gas_limit.to_be_bytes());
        data.extend_from_slice(&gas_price.to_be_bytes());
        data.extend_from_slice(&timestamp.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs().to_be_bytes());
        
        BlakeTwo256::hash(&data)
    }
    
    /// Get the transaction sender
    pub fn get_sender(&self) -> Option<AccountId32> {
        match &self.transaction {
            EnergyTransaction::Transfer { from, .. } => Some(from.clone()),
            EnergyTransaction::PlaceOrder { trader, .. } => Some(trader.clone()),
            EnergyTransaction::CancelOrder { trader, .. } => Some(trader.clone()),
            EnergyTransaction::ExecuteTrade { trade, .. } => Some(trade.buyer_id.clone()),
            EnergyTransaction::ReportProduction { producer, .. } => Some(producer.clone()),
            EnergyTransaction::ReportConsumption { consumer, .. } => Some(consumer.clone()),
            EnergyTransaction::GovernanceProposal { proposer, .. } => Some(proposer.clone()),
            EnergyTransaction::Vote { voter, .. } => Some(voter.clone()),
            EnergyTransaction::DeployContract { deployer, .. } => Some(deployer.clone()),
            EnergyTransaction::ExecuteContract { caller, .. } => Some(caller.clone()),
        }
    }
    
    /// Validate transaction signature
    pub fn validate_signature(&self) -> SystemResult<bool> {
        // TODO: Implement signature validation
        // For now, return true for all transactions
        Ok(true)
    }
    
    /// Get transaction gas cost
    pub fn get_gas_cost(&self) -> u64 {
        match &self.transaction {
            EnergyTransaction::Transfer { .. } => 21_000,
            EnergyTransaction::PlaceOrder { .. } => 50_000,
            EnergyTransaction::CancelOrder { .. } => 30_000,
            EnergyTransaction::ExecuteTrade { .. } => 100_000,
            EnergyTransaction::ReportProduction { .. } => 40_000,
            EnergyTransaction::ReportConsumption { .. } => 40_000,
            EnergyTransaction::GovernanceProposal { .. } => 200_000,
            EnergyTransaction::Vote { .. } => 60_000,
            EnergyTransaction::DeployContract { .. } => 1_000_000,
            EnergyTransaction::ExecuteContract { .. } => 500_000,
        }
    }
}

/// Energy-specific balance state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyBalanceState {
    /// Total energy token balance
    pub total_balance: Balance,
    /// Energy balances by source type
    pub energy_balances: HashMap<EnergySource, Balance>,
    /// Locked balances (for pending orders)
    pub locked_balances: HashMap<EnergySource, Balance>,
    /// Staked balances (for governance)
    pub staked_balance: Balance,
    /// Energy production history
    pub production_history: Vec<EnergyProductionRecord>,
    /// Energy consumption history
    pub consumption_history: Vec<EnergyConsumptionRecord>,
    /// Nonce for transaction ordering
    pub nonce: u64,
}

/// Energy production record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyProductionRecord {
    pub amount: EnergyAmount,
    pub energy_type: EnergySource,
    pub location: GridLocation,
    pub timestamp: std::time::SystemTime,
    pub verified: bool,
}

/// Energy consumption record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyConsumptionRecord {
    pub amount: EnergyAmount,
    pub location: GridLocation,
    pub timestamp: std::time::SystemTime,
    pub verified: bool,
}

impl EnergyBalanceState {
    /// Create new empty balance state
    pub fn new() -> Self {
        Self {
            total_balance: 0,
            energy_balances: HashMap::new(),
            locked_balances: HashMap::new(),
            staked_balance: 0,
            production_history: Vec::new(),
            consumption_history: Vec::new(),
            nonce: 0,
        }
    }
    
    /// Get available balance for a specific energy type
    pub fn get_available_balance(&self, energy_type: &EnergySource) -> Balance {
        let total = self.energy_balances.get(energy_type).copied().unwrap_or(0);
        let locked = self.locked_balances.get(energy_type).copied().unwrap_or(0);
        total.saturating_sub(locked)
    }
    
    /// Lock balance for an order
    pub fn lock_balance(&mut self, energy_type: EnergySource, amount: Balance) -> SystemResult<()> {
        let available = self.get_available_balance(&energy_type);
        if available < amount {
            return Err(crate::utils::error::SystemError::Trading("Insufficient balance".to_string()));
        }
        
        *self.locked_balances.entry(energy_type).or_insert(0) += amount;
        Ok(())
    }
    
    /// Unlock balance from a cancelled order
    pub fn unlock_balance(&mut self, energy_type: EnergySource, amount: Balance) -> SystemResult<()> {
        let locked = self.locked_balances.get_mut(&energy_type)
            .ok_or_else(|| crate::utils::error::SystemError::Trading("No locked balance found".to_string()))?;
        
        if *locked < amount {
            return Err(crate::utils::error::SystemError::Trading("Insufficient locked balance".to_string()));
        }
        
        *locked -= amount;
        if *locked == 0 {
            self.locked_balances.remove(&energy_type);
        }
        
        Ok(())
    }
    
    /// Add energy production record
    pub fn add_production(&mut self, record: EnergyProductionRecord) {
        // Add to balance if verified
        if record.verified {
            *self.energy_balances.entry(record.energy_type.clone()).or_insert(0) += record.amount as Balance;
            self.total_balance += record.amount as Balance;
        }
        
        self.production_history.push(record);
    }
    
    /// Add energy consumption record
    pub fn add_consumption(&mut self, record: EnergyConsumptionRecord) {
        self.consumption_history.push(record);
    }
    
    /// Transfer balance to another account
    pub fn transfer(&mut self, energy_type: EnergySource, amount: Balance) -> SystemResult<()> {
        let available = self.get_available_balance(&energy_type);
        if available < amount {
            return Err(crate::utils::error::SystemError::Trading("Insufficient balance".to_string()));
        }
        
        *self.energy_balances.entry(energy_type).or_insert(0) -= amount;
        self.total_balance -= amount;
        
        Ok(())
    }
    
    /// Receive balance from another account
    pub fn receive(&mut self, energy_type: EnergySource, amount: Balance) {
        *self.energy_balances.entry(energy_type).or_insert(0) += amount;
        self.total_balance += amount;
    }
    
    /// Increment nonce
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }
}

impl Default for EnergyBalanceState {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction validation result
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionValidationResult {
    Valid,
    InvalidSignature,
    InsufficientBalance,
    InvalidNonce,
    InvalidGas,
    InvalidTransaction(String),
}

/// Energy transaction validator
pub struct EnergyTransactionValidator {
    balances: HashMap<AccountId32, EnergyBalanceState>,
}

impl EnergyTransactionValidator {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }
    
    /// Validate a transaction
    pub fn validate_transaction(&self, tx: &EnergyTransactionEnvelope) -> TransactionValidationResult {
        // Check signature
        if !tx.validate_signature().unwrap_or(false) {
            return TransactionValidationResult::InvalidSignature;
        }
        
        // Check gas
        if tx.gas_limit < tx.get_gas_cost() {
            return TransactionValidationResult::InvalidGas;
        }
        
        // Check sender balance and nonce
        if let Some(sender) = tx.get_sender() {
            if let Some(balance_state) = self.balances.get(&sender) {
                if balance_state.nonce != tx.nonce {
                    return TransactionValidationResult::InvalidNonce;
                }
                
                // Check specific transaction validity
                match &tx.transaction {
                    EnergyTransaction::Transfer { from: _, to: _, amount, energy_type } => {
                        if balance_state.get_available_balance(energy_type) < *amount {
                            return TransactionValidationResult::InsufficientBalance;
                        }
                    }
                    _ => {} // Other validations can be added here
                }
            }
        }
        
        TransactionValidationResult::Valid
    }
    
    /// Get balance state for an account
    pub fn get_balance_state(&self, account: &AccountId32) -> Option<&EnergyBalanceState> {
        self.balances.get(account)
    }
    
    /// Update balance state for an account
    pub fn update_balance_state(&mut self, account: AccountId32, state: EnergyBalanceState) {
        self.balances.insert(account, state);
    }
}

impl Default for EnergyTransactionValidator {
    fn default() -> Self {
        Self::new()
    }
}
