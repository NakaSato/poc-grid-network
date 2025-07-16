//! # Energy-Focused Transaction Types
//! 
//! This module defines blockchain transaction types specifically for energy trading,
//! including advanced energy balance tracking and smart contract support.

use crate::types::*;
use crate::utils::SystemResult;
use crate::config::BlockchainConfig;
use serde::{Deserialize, Serialize};
use crate::types::AccountId;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::time::SystemTime;

/// Energy-specific transaction types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnergyTransaction {
    /// Transfer energy tokens between accounts
    Transfer {
        from: AccountId,
        to: AccountId,
        amount: Balance,
        energy_type: EnergySource,
        grid_location: GridLocation,
    },
    /// Place an energy order on the trading platform
    PlaceOrder {
        trader: AccountId,
        order: EnergyOrder,
        collateral: Balance,
        signature: Vec<u8>,
    },
    /// Cancel an existing energy order
    CancelOrder {
        trader: AccountId,
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
        producer: AccountId,
        production_record: EnergyProductionRecord,
        validator_signatures: Vec<ValidatorSignature>,
    },
    /// Report energy consumption
    ReportConsumption {
        consumer: AccountId,
        consumption_record: EnergyConsumptionRecord,
        meter_signature: Vec<u8>,
    },
    /// Governance proposal transaction
    GovernanceProposal {
        proposer: AccountId,
        proposal: GovernanceProposal,
        stake_amount: Balance,
        signature: Vec<u8>,
    },
    /// Vote on a governance proposal
    Vote {
        voter: AccountId,
        proposal_id: uuid::Uuid,
        choice: VoteChoice,
        voting_power: Balance,
        signature: Vec<u8>,
    },
    /// Smart contract deployment
    DeployContract {
        deployer: AccountId,
        contract_code: Vec<u8>,
        constructor_args: Vec<u8>,
        gas_limit: u64,
        signature: Vec<u8>,
    },
    /// Smart contract execution
    ExecuteContract {
        caller: AccountId,
        contract_address: AccountId,
        method: String,
        args: Vec<u8>,
        gas_limit: u64,
        signature: Vec<u8>,
    },
    /// Register as energy producer
    RegisterProducer {
        producer: AccountId,
        location: GridLocation,
        energy_types: Vec<EnergySource>,
        capacity: EnergyAmount,
        certification: ProducerCertification,
        signature: Vec<u8>,
    },
    /// Register as energy consumer
    RegisterConsumer {
        consumer: AccountId,
        location: GridLocation,
        consumption_pattern: ConsumptionPattern,
        signature: Vec<u8>,
    },
    /// Update grid status
    UpdateGridStatus {
        authority: AccountId,
        location: GridLocation,
        status: GridStatus,
        signature: Vec<u8>,
    },
    /// Carbon credit transaction
    CarbonCredit {
        issuer: AccountId,
        recipient: AccountId,
        credits: u32,
        energy_source: EnergySource,
        verification: CarbonVerification,
        signature: Vec<u8>,
    },
    /// Energy storage transaction
    EnergyStorage {
        storage_operator: AccountId,
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
    pub validator: AccountId,
    pub capacity_check: bool,
    pub congestion_level: CongestionLevel,
    pub transmission_cost: Balance,
    pub timestamp: SystemTime,
}

/// Validator signature for production records
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatorSignature {
    pub validator: AccountId,
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
    pub verifier: AccountId,
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
        if !self.locked_balances.contains_key(&energy_type) {
            self.locked_balances.insert(energy_type.clone(), 0);
        }
        let locked = self.locked_balances.get_mut(&energy_type).unwrap();
        if *locked < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Cannot unlock more than locked amount".to_string()
            ));
        }

        *locked -= amount;
        self.last_updated = SystemTime::now();
        Ok(())
    }

    /// Increment nonce
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
        self.last_updated = SystemTime::now();
    }
}

impl EnergyTransactionEnvelope {
    /// Create new transaction envelope
    pub fn new(transaction: EnergyTransaction, nonce: u64) -> Self {
        let serialized = serde_json::to_vec(&transaction).unwrap_or_default();
        let mut hasher = Sha256::new();
        hasher.update(&serialized);
        let hash_result = hasher.finalize();
        
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hash_result);

        Self {
            transaction,
            nonce,
            gas_price: 1000, // Default gas price
            gas_limit: 100000, // Default gas limit
            timestamp: SystemTime::now(),
            hash,
        }
    }

    /// Verify transaction signature
    pub fn verify_signature(&self) -> SystemResult<bool> {
        match &self.transaction {
            EnergyTransaction::Transfer { from, to, amount, .. } => {
                Ok(from != to && *amount > 0)
            }
            EnergyTransaction::PlaceOrder { order, .. } => {
                Ok(order.energy_amount > 0.0 && order.price_per_unit > 0)
            }
            _ => Ok(true),
        }
    }

    /// Calculate transaction fee based on gas usage
    pub fn calculate_fee(&self) -> Balance {
        self.gas_price * (self.gas_limit as Balance)
    }

    /// Get gas cost for this transaction
    pub fn get_gas_cost(&self) -> u64 {
        match &self.transaction {
            EnergyTransaction::Transfer { .. } => 21000,
            EnergyTransaction::PlaceOrder { .. } => 50000,
            EnergyTransaction::CancelOrder { .. } => 30000,
            EnergyTransaction::ExecuteTrade { .. } => 100000,
            EnergyTransaction::ReportProduction { .. } => 40000,
            EnergyTransaction::ReportConsumption { .. } => 35000,
            EnergyTransaction::GovernanceProposal { .. } => 60000,
            EnergyTransaction::Vote { .. } => 25000,
            EnergyTransaction::DeployContract { .. } => 200000,
            EnergyTransaction::ExecuteContract { .. } => 80000,
            EnergyTransaction::RegisterProducer { .. } => 45000,
            EnergyTransaction::RegisterConsumer { .. } => 40000,
            EnergyTransaction::UpdateGridStatus { .. } => 30000,
            EnergyTransaction::CarbonCredit { .. } => 35000,
            EnergyTransaction::EnergyStorage { .. } => 40000,
        }
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

/// Transaction validation result
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionValidationResult {
    /// Transaction is valid
    Valid,
    /// Transaction is invalid
    Invalid(String),
}

/// Energy transaction validator
pub struct EnergyTransactionValidator {
    /// Configuration
    config: BlockchainConfig,
}

impl EnergyTransactionValidator {
    /// Create new transaction validator
    pub fn new() -> Self {
        Self {
            config: BlockchainConfig::default(),
        }
    }
    
    /// Validate energy transaction
    pub fn validate_transaction(&self, transaction: &EnergyTransactionEnvelope) -> TransactionValidationResult {
        // Basic validation
        if transaction.gas_limit == 0 {
            return TransactionValidationResult::Invalid("Gas limit cannot be zero".to_string());
        }
        
        if transaction.gas_price == 0 {
            return TransactionValidationResult::Invalid("Gas price cannot be zero".to_string());
        }
        
        // Transaction-specific validation
        match &transaction.transaction {
            EnergyTransaction::ExecuteTrade { trade, .. } => {
                if trade.energy_amount == 0.0 {
                    return TransactionValidationResult::Invalid("Energy amount cannot be zero".to_string());
                }
                if trade.price_per_unit == 0 {
                    return TransactionValidationResult::Invalid("Price per unit cannot be zero".to_string());
                }
            }
            EnergyTransaction::ReportProduction { production_record, .. } => {
                if production_record.amount == 0.0 {
                    return TransactionValidationResult::Invalid("Production amount cannot be zero".to_string());
                }
            }
            _ => {}
        }
        
        TransactionValidationResult::Valid
    }
}
