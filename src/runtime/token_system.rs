//! # Token System
//! 
//! Manages energy tokens with specialized tracking for different energy sources.

use crate::config::SystemConfig;
use crate::types::*;
use crate::utils::SystemResult;
use crate::blockchain::transactions::{EnergyBalanceState, EnergyProductionRecord, EnergyConsumptionRecord};
use crate::types::AccountId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Energy token system with specialized balance tracking
pub struct TokenSystem {
    /// Account balances with energy-specific tracking
    balances: Arc<RwLock<HashMap<AccountId, EnergyBalanceState>>>,
    /// Total token supply
    total_supply: Arc<RwLock<Balance>>,
    /// Maximum token supply (10 million tokens)
    max_supply: Balance,
    /// Energy token exchange rates
    exchange_rates: Arc<RwLock<HashMap<EnergySource, f64>>>,
    /// Energy production validators
    production_validators: Arc<RwLock<HashMap<AccountId, ProductionValidator>>>,
}

/// Production validator for energy sources
#[derive(Debug, Clone)]
pub struct ProductionValidator {
    pub validator_id: AccountId,
    pub energy_types: Vec<EnergySource>,
    pub location: GridLocation,
    pub certified: bool,
    pub reputation_score: f64,
}

impl TokenSystem {
    pub async fn new(_config: &SystemConfig) -> SystemResult<Self> {
        let mut exchange_rates = HashMap::new();
        
        // Set initial exchange rates for different energy sources
        exchange_rates.insert(EnergySource::Solar, 1.0);      // 1:1 base rate
        exchange_rates.insert(EnergySource::Wind, 1.0);       // 1:1 base rate
        exchange_rates.insert(EnergySource::Hydro, 1.1);      // 10% premium for hydro
        exchange_rates.insert(EnergySource::Biomass, 0.9);    // 10% discount for biomass
        exchange_rates.insert(EnergySource::NaturalGas, 0.8); // 20% discount for natural gas
        exchange_rates.insert(EnergySource::Mixed, 0.95);     // 5% discount for mixed
        
        Ok(Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
            total_supply: Arc::new(RwLock::new(0)),
            max_supply: 10_000_000 * 1_000_000_000_000, // 10M tokens with 12 decimal places
            exchange_rates: Arc::new(RwLock::new(exchange_rates)),
            production_validators: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("TokenSystem", "Starting energy token system");
        
        // Initialize system with genesis balances
        self.initialize_genesis_balances().await?;
        
        // Start background tasks
        self.start_exchange_rate_updater().await?;
        self.start_production_validator().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("TokenSystem", "Stopping energy token system");
        Ok(())
    }
    
    /// Get comprehensive balance for an account
    pub async fn get_energy_balance(&self, account: &AccountId) -> EnergyBalanceState {
        let balances = self.balances.read().await;
        balances.get(account).cloned().unwrap_or_else(EnergyBalanceState::new)
    }
    
    /// Get balance for a specific energy type
    pub async fn get_energy_type_balance(&self, account: &AccountId, energy_type: &EnergySource) -> Balance {
        let balance_state = self.get_energy_balance(account).await;
        balance_state.get_available_balance(energy_type)
    }
    
    /// Transfer energy tokens with source tracking
    pub async fn transfer_energy(
        &self,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
        energy_type: EnergySource,
    ) -> SystemResult<()> {
        let mut balances = self.balances.write().await;
        
        // Get sender balance
        let sender_balance = balances.entry(from.clone()).or_insert_with(EnergyBalanceState::new);
        
        // Check if sender has sufficient balance
        if sender_balance.get_available_balance(&energy_type) < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Insufficient energy balance".to_string()
            ));
        }
        
        // Transfer from sender
        sender_balance.transfer(energy_type.clone(), amount)?;
        sender_balance.increment_nonce();
        
        // Transfer to recipient
        let recipient_balance = balances.entry(to.clone()).or_insert_with(EnergyBalanceState::new);
        recipient_balance.receive(energy_type.clone(), amount);
        
        crate::utils::logging::log_info(
            "TokenSystem",
            &format!("Energy transfer: {} -> {}, {} tokens of {:?}", 
                     from, to, amount, &energy_type)
        );
        
        Ok(())
    }
    
    /// Mint energy tokens from production
    pub async fn mint_from_production(
        &self,
        producer: &AccountId,
        production_record: EnergyProductionRecord,
    ) -> SystemResult<()> {
        // Validate production record
        if !production_record.verified {
            return Err(crate::utils::error::SystemError::Trading(
                "Production record not verified".to_string()
            ));
        }
        
        // Calculate token amount based on energy type and exchange rate
        let exchange_rates = self.exchange_rates.read().await;
        let rate = exchange_rates.get(&production_record.energy_type).copied().unwrap_or(1.0);
        let token_amount = (production_record.amount as f64 * rate) as Balance;
        
        // Check total supply limit
        let mut total_supply = self.total_supply.write().await;
        if *total_supply + token_amount > self.max_supply {
            return Err(crate::utils::error::SystemError::Trading(
                "Minting would exceed maximum supply".to_string()
            ));
        }
        
        // Mint tokens
        let mut balances = self.balances.write().await;
        let producer_balance = balances.entry(producer.clone()).or_insert_with(EnergyBalanceState::new);
        producer_balance.add_production(production_record.clone());
        
        *total_supply += token_amount;
        
        crate::utils::logging::log_info(
            "TokenSystem",
            &format!("Minted {} tokens from {:?} production for {}", 
                     token_amount, production_record.energy_type, producer)
        );
        
        Ok(())
    }
    
    /// Burn energy tokens from consumption
    pub async fn burn_from_consumption(
        &self,
        consumer: &AccountId,
        consumption_record: EnergyConsumptionRecord,
    ) -> SystemResult<()> {
        let token_amount = consumption_record.amount as Balance;
        
        let mut balances = self.balances.write().await;
        let consumer_balance = balances.entry(consumer.clone()).or_insert_with(EnergyBalanceState::new);
        
        // Check if consumer has sufficient balance
        if consumer_balance.total_balance < token_amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Insufficient balance for consumption".to_string()
            ));
        }
        
        // Burn tokens (reduce from mixed energy type)
        consumer_balance.transfer(EnergySource::Mixed, token_amount)?;
        consumer_balance.add_consumption(consumption_record.clone());
        
        // Reduce total supply
        let mut total_supply = self.total_supply.write().await;
        *total_supply = total_supply.saturating_sub(token_amount);
        
        crate::utils::logging::log_info(
            "TokenSystem",
            &format!("Burned {} tokens from consumption for {}", 
                     token_amount, consumer)
        );
        
        Ok(())
    }
    
    /// Lock tokens for trading orders
    pub async fn lock_tokens_for_order(
        &self,
        account: &AccountId,
        energy_type: EnergySource,
        amount: Balance,
    ) -> SystemResult<()> {
        let mut balances = self.balances.write().await;
        let balance_state = balances.entry(account.clone()).or_insert_with(EnergyBalanceState::new);
        
        balance_state.lock_balance(energy_type.clone(), amount)?;
        
        crate::utils::logging::log_info(
            "TokenSystem",
            &format!("Locked {} tokens of {:?} for {}", amount, energy_type, account)
        );
        
        Ok(())
    }
    
    /// Unlock tokens from cancelled orders
    pub async fn unlock_tokens_from_order(
        &self,
        account: &AccountId,
        energy_type: EnergySource,
        amount: Balance,
    ) -> SystemResult<()> {
        let mut balances = self.balances.write().await;
        let balance_state = balances.entry(account.clone()).or_insert_with(EnergyBalanceState::new);
        
        balance_state.unlock_balance(energy_type.clone(), amount)?;
        
        crate::utils::logging::log_info(
            "TokenSystem",
            &format!("Unlocked {} tokens of {:?} for {}", amount, energy_type, account)
        );
        
        Ok(())
    }
    
    /// Add production validator
    pub async fn add_production_validator(
        &self,
        validator: ProductionValidator,
    ) -> SystemResult<()> {
        let mut validators = self.production_validators.write().await;
        validators.insert(validator.validator_id.clone(), validator);
        
        Ok(())
    }
    
    /// Validate energy production
    pub async fn validate_production(
        &self,
        _producer: &AccountId,
        production_record: &EnergyProductionRecord,
    ) -> SystemResult<bool> {
        let validators = self.production_validators.read().await;
        
        // Find validators for this location and energy type
        let suitable_validators: Vec<_> = validators.values()
            .filter(|v| {
                v.certified && 
                v.energy_types.contains(&production_record.energy_type) &&
                v.location.region == production_record.location.region
            })
            .collect();
        
        if suitable_validators.is_empty() {
            return Ok(false); // No suitable validators
        }
        
        // For now, accept production if at least one validator is available
        // In a real system, this would involve complex validation logic
        Ok(true)
    }
    
    /// Get energy exchange rate
    pub async fn get_exchange_rate(&self, energy_type: &EnergySource) -> f64 {
        let rates = self.exchange_rates.read().await;
        rates.get(energy_type).copied().unwrap_or(1.0)
    }
    
    /// Update exchange rate
    pub async fn update_exchange_rate(&self, energy_type: EnergySource, rate: f64) -> SystemResult<()> {
        let mut rates = self.exchange_rates.write().await;
        rates.insert(energy_type.clone(), rate);
        
        crate::utils::logging::log_info(
            "TokenSystem",
            &format!("Updated exchange rate for {:?} to {}", energy_type, rate)
        );
        
        Ok(())
    }
    
    /// Get total supply
    pub async fn get_total_supply(&self) -> Balance {
        *self.total_supply.read().await
    }
    
    /// Get maximum supply
    pub fn get_max_supply(&self) -> Balance {
        self.max_supply
    }
    
    /// Get all balances (for admin/debugging)
    pub async fn get_all_balances(&self) -> HashMap<AccountId, EnergyBalanceState> {
        self.balances.read().await.clone()
    }
    
    /// Initialize genesis balances
    async fn initialize_genesis_balances(&self) -> SystemResult<()> {
        // Create some initial balances for testing
        let genesis_account = hex::encode([1u8; 32]);
        let mut balances = self.balances.write().await;
        
        let mut genesis_balance = EnergyBalanceState::new();
        genesis_balance.total_balance = 1_000_000; // 1M tokens
        genesis_balance.energy_balances.insert(EnergySource::Mixed, 1_000_000);
        
        balances.insert(genesis_account, genesis_balance);
        
        *self.total_supply.write().await = 1_000_000;
        
        Ok(())
    }
    
    /// Start exchange rate updater
    async fn start_exchange_rate_updater(&self) -> SystemResult<()> {
        let rates = self.exchange_rates.clone();
        
        tokio::spawn(async move {
            loop {
                // Update exchange rates based on market conditions
                // This is a simplified version - in reality would connect to external price feeds
                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // 5 minutes
                
                // Mock rate updates
                let mut rates_lock = rates.write().await;
                for (_energy_type, rate) in rates_lock.iter_mut() {
                    // Add small random fluctuation
                    let fluctuation = (rand::random::<f64>() - 0.5) * 0.1; // ±5%
                    *rate *= 1.0 + fluctuation;
                    *rate = rate.max(0.1).min(2.0); // Keep within reasonable bounds
                }
            }
        });
        
        Ok(())
    }
    
    /// Start production validator
    async fn start_production_validator(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("TokenSystem", "Starting production validator");
        
        // In a real system, this would start a service that validates production claims
        // For now, just log that it's starting
        
        Ok(())
    }
    
    // Legacy compatibility methods
    pub async fn get_balance(&self, account: &AccountId) -> Balance {
        let balance_state = self.get_energy_balance(account).await;
        balance_state.total_balance
    }
    
    pub async fn transfer(
        &self,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> SystemResult<()> {
        self.transfer_energy(from, to, amount, EnergySource::Mixed).await
    }
    
    pub async fn mint(&self, account: &AccountId, amount: Balance) -> SystemResult<()> {
        let production_record = EnergyProductionRecord {
            amount: amount as EnergyAmount,
            energy_type: EnergySource::Mixed,
            location: GridLocation {
                province: "default".to_string(),
                district: "default".to_string(),
                coordinates: (0.0, 0.0),
                region: "default".to_string(),
                substation: "default".to_string(),
                grid_code: "default".to_string(),
                meter_id: "default".to_string(),
            },
            timestamp: std::time::SystemTime::now(),
            verified: true,
            efficiency: 0.85,
            weather_conditions: None,
            equipment_id: "default".to_string(),
            quality_metrics: crate::blockchain::transactions::EnergyQualityMetrics {
                voltage: 230.0,
                frequency: 50.0,
                power_factor: 0.95,
                harmonic_distortion: 0.05,
            },
        };
        
        self.mint_from_production(account, production_record).await
    }
    
    pub async fn burn(&self, account: &AccountId, amount: Balance) -> SystemResult<()> {
        let consumption_record = EnergyConsumptionRecord {
            amount: amount as EnergyAmount,
            location: GridLocation {
                province: "default".to_string(),
                district: "default".to_string(),
                coordinates: (0.0, 0.0),
                region: "default".to_string(),
                substation: "default".to_string(),
                grid_code: "default".to_string(),
                meter_id: "default".to_string(),
            },
            timestamp: std::time::SystemTime::now(),
            verified: true,
            consumer_type: crate::blockchain::transactions::ConsumerType::Residential,
            appliance_breakdown: std::collections::HashMap::new(),
        };
        
        self.burn_from_consumption(account, consumption_record).await
    }
    
    pub fn create_energy_token(amount: Balance) -> EnergyToken {
        EnergyToken { 
            id: uuid::Uuid::new_v4().to_string(),
            amount,
            energy_type: EnergySource::Mixed,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::days(365),
            verified: true,
        }
    }
    
    pub async fn validate_transfer(
        &self,
        from: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> SystemResult<()> {
        // Check if accounts are valid
        if from == to {
            return Err(crate::utils::error::SystemError::Validation(
                "Cannot transfer to same account".to_string()
            ));
        }
        
        // Check if amount is valid
        if amount == 0 {
            return Err(crate::utils::error::SystemError::Validation(
                "Transfer amount must be greater than 0".to_string()
            ));
        }
        
        // Check if sender has sufficient balance
        let sender_balance = self.get_balance(from).await;
        if sender_balance < amount {
            return Err(crate::utils::error::SystemError::Trading(
                "Insufficient balance".to_string()
            ));
        }
        
        Ok(())
    }
}
