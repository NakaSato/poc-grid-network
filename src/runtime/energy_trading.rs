//! # Energy Trading Pallet
//! 
//! Implements the core energy trading functionality including order book,
//! matching engine, and settlement system.

use crate::config::SystemConfig;
use crate::types::*;
use crate::utils::SystemResult;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Energy trading pallet
pub struct EnergyTradingPallet {
    /// Active buy orders
    buy_orders: Arc<RwLock<VecDeque<EnergyOrder>>>,
    /// Active sell orders
    sell_orders: Arc<RwLock<VecDeque<EnergyOrder>>>,
    /// Order history
    order_history: Arc<RwLock<HashMap<Uuid, EnergyOrder>>>,
    /// Trade history
    trade_history: Arc<RwLock<Vec<EnergyTrade>>>,
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl EnergyTradingPallet {
    pub async fn new(_config: &SystemConfig) -> SystemResult<Self> {
        Ok(Self {
            buy_orders: Arc::new(RwLock::new(VecDeque::new())),
            sell_orders: Arc::new(RwLock::new(VecDeque::new())),
            order_history: Arc::new(RwLock::new(HashMap::new())),
            trade_history: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Energy Trading Pallet");
        
        // Start order matching engine
        self.start_matching_engine().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Energy Trading Pallet");
        
        Ok(())
    }
    
    /// Place a new energy order
    pub async fn place_order(&self, order: EnergyOrder) -> SystemResult<Uuid> {
        let order_id = order.id;
        
        // Validate order
        self.validate_order(&order).await?;
        
        // Add to order history
        {
            let mut history = self.order_history.write().await;
            history.insert(order_id, order.clone());
        }
        
        // Add to appropriate order book
        match order.order_type {
            OrderType::Buy => {
                let mut buy_orders = self.buy_orders.write().await;
                buy_orders.push_back(order.clone());
            }
            OrderType::Sell => {
                let mut sell_orders = self.sell_orders.write().await;
                sell_orders.push_back(order.clone());
            }
        }
        
        crate::utils::logging::log_info(
            "EnergyTradingPallet",
            &format!("Order placed: {} {} kWh at {} THB/kWh", 
                     order.id, 
                     order.energy_amount, 
                     order.price_per_unit)
        );
        
        // Trigger matching engine
        self.match_orders().await?;
        
        Ok(order_id)
    }
    
    /// Cancel an existing order
    pub async fn cancel_order(&self, order_id: Uuid) -> SystemResult<()> {
        // Remove from buy orders
        {
            let mut buy_orders = self.buy_orders.write().await;
            buy_orders.retain(|order| order.id != order_id);
        }
        
        // Remove from sell orders
        {
            let mut sell_orders = self.sell_orders.write().await;
            sell_orders.retain(|order| order.id != order_id);
        }
        
        // Update order status in history
        {
            let mut history = self.order_history.write().await;
            if let Some(order) = history.get_mut(&order_id) {
                order.status = OrderStatus::Cancelled;
                order.updated_at = crate::utils::now();
            }
        }
        
        crate::utils::logging::log_info(
            "EnergyTradingPallet",
            &format!("Order cancelled: {}", order_id)
        );
        
        Ok(())
    }
    
    /// Get order by ID
    pub async fn get_order(&self, order_id: Uuid) -> Option<EnergyOrder> {
        let history = self.order_history.read().await;
        history.get(&order_id).cloned()
    }
    
    /// Get all active buy orders
    pub async fn get_buy_orders(&self) -> Vec<EnergyOrder> {
        let buy_orders = self.buy_orders.read().await;
        buy_orders.iter().cloned().collect()
    }
    
    /// Get all active sell orders
    pub async fn get_sell_orders(&self) -> Vec<EnergyOrder> {
        let sell_orders = self.sell_orders.read().await;
        sell_orders.iter().cloned().collect()
    }
    
    /// Get trade history
    pub async fn get_trade_history(&self) -> Vec<EnergyTrade> {
        let history = self.trade_history.read().await;
        history.clone()
    }
    
    /// Start the order matching engine
    async fn start_matching_engine(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("EnergyTradingPallet", "Starting matching engine");
        
        // Spawn background task for continuous matching
        let self_clone = self.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = self_clone.match_orders().await {
                    crate::utils::logging::log_error("EnergyTradingPallet", &e);
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
        
        Ok(())
    }
    
    /// Match buy and sell orders
    async fn match_orders(&self) -> SystemResult<()> {
        let mut buy_orders = self.buy_orders.write().await;
        let mut sell_orders = self.sell_orders.write().await;
        
        // Sort orders by price (best first)
        let mut buy_orders_sorted: Vec<_> = buy_orders.iter().cloned().collect();
        let mut sell_orders_sorted: Vec<_> = sell_orders.iter().cloned().collect();
        
        // Sort buy orders by price (highest first)
        buy_orders_sorted.sort_by(|a, b| b.price_per_unit.cmp(&a.price_per_unit));
        
        // Sort sell orders by price (lowest first)
        sell_orders_sorted.sort_by(|a, b| a.price_per_unit.cmp(&b.price_per_unit));
        
        let mut matched_trades = Vec::new();
        
        for buy_order in &buy_orders_sorted {
            for sell_order in &sell_orders_sorted {
                if buy_order.price_per_unit >= sell_order.price_per_unit {
                    // Check if orders can be matched (location, energy source, etc.)
                    if self.can_match_orders(buy_order, sell_order).await? {
                        let trade = self.create_trade(buy_order, sell_order).await?;
                        matched_trades.push(trade);
                    }
                }
            }
        }
        
        // Execute matched trades
        for trade in matched_trades {
            self.execute_trade(trade).await?;
        }
        
        Ok(())
    }
    
    /// Check if two orders can be matched
    async fn can_match_orders(&self, buy_order: &EnergyOrder, sell_order: &EnergyOrder) -> SystemResult<bool> {
        // Check price compatibility
        if buy_order.price_per_unit < sell_order.price_per_unit {
            return Ok(false);
        }
        
        // Check energy source compatibility
        if let Some(preferred_source) = &buy_order.energy_source {
            if let Some(available_source) = &sell_order.energy_source {
                if preferred_source != available_source {
                    return Ok(false);
                }
            }
        }
        
        // Check grid location compatibility (simplified)
        if buy_order.location.province != sell_order.location.province {
            return Ok(false);
        }
        
        // Orders are compatible
        Ok(true)
    }
    
    /// Create a trade from matched orders
    async fn create_trade(&self, buy_order: &EnergyOrder, sell_order: &EnergyOrder) -> SystemResult<EnergyTrade> {
        let trade_id = Uuid::new_v4();
        let energy_amount = buy_order.energy_amount.min(sell_order.energy_amount);
        let price_per_kwh = sell_order.price_per_unit; // Use seller's price
        let total_price = energy_amount as Balance * price_per_kwh;
        let grid_fee = (total_price as f64 * 0.05) as Balance; // 5% grid fee
        
        let trade = EnergyTrade {
            trade_id: trade_id.to_string(),
            energy_amount,
            price_per_unit: price_per_kwh,
            buyer_id: buy_order.account_id.clone(),
            seller_id: sell_order.account_id.clone(),
            timestamp: crate::utils::now().timestamp() as u64,
            status: TradeStatus::Pending,
            grid_location: sell_order.location.clone(),
            // Legacy fields for compatibility
            id: trade_id.to_string(),
            buy_order_id: buy_order.id.to_string(),
            sell_order_id: sell_order.id.to_string(),
            price_per_kwh,
            total_price,
            grid_fee,
            energy_source: sell_order.energy_source.clone().unwrap_or(EnergySource::Solar),
            carbon_offset: CarbonOffset {
                offset_credits: energy_amount as f64 * 0.5, // Simplified calculation
                verified: false,
                certification_body: "Thai Energy Authority".to_string(),
                timestamp: crate::utils::now(),
            },
        };
        
        Ok(trade)
    }
    
    /// Execute a trade
    async fn execute_trade(&self, trade: EnergyTrade) -> SystemResult<()> {
        // Add to trade history
        {
            let mut history = self.trade_history.write().await;
            history.push(trade.clone());
        }
        
        // Update order statuses
        self.update_order_status_after_trade(&trade).await?;
        
        crate::utils::logging::log_info(
            "EnergyTradingPallet",
            &format!("Trade executed: {} kWh at {} THB/kWh", 
                     trade.energy_amount, 
                     trade.price_per_kwh)
        );
        
        Ok(())
    }
    
    /// Update order status after trade execution
    async fn update_order_status_after_trade(&self, trade: &EnergyTrade) -> SystemResult<()> {
        let mut history = self.order_history.write().await;
        
        // Update buy order
        if let Some(buy_order) = history.get_mut(&trade.buy_order_id.parse::<Uuid>().unwrap()) {
            if buy_order.energy_amount == trade.energy_amount {
                buy_order.status = OrderStatus::Filled;
            } else {
                buy_order.status = OrderStatus::PartiallyFilled;
                buy_order.energy_amount -= trade.energy_amount;
            }
            buy_order.timestamp = crate::utils::now();
        }
        
        // Update sell order
        if let Some(sell_order) = history.get_mut(&trade.sell_order_id.parse::<Uuid>().unwrap()) {
            if sell_order.energy_amount == trade.energy_amount {
                sell_order.status = OrderStatus::Filled;
            } else {
                sell_order.status = OrderStatus::PartiallyFilled;
                sell_order.energy_amount -= trade.energy_amount;
            }
            sell_order.timestamp = crate::utils::now();
        }
        
        Ok(())
    }
    
    /// Validate an order before placement
    async fn validate_order(&self, order: &EnergyOrder) -> SystemResult<()> {
        // Check energy amount
        if !crate::utils::is_valid_energy_amount(order.energy_amount as u64) {
            return Err(crate::utils::SystemError::Validation(
                "Invalid energy amount".to_string()
            ));
        }
        
        // Check price
        if !crate::utils::is_valid_price(order.price_per_unit) {
            return Err(crate::utils::SystemError::Validation(
                "Invalid price".to_string()
            ));
        }
        
        // Check grid location
        if !crate::utils::is_valid_grid_location(&order.location) {
            return Err(crate::utils::SystemError::Validation(
                "Invalid grid location".to_string()
            ));
        }
        
        // Order is valid
        Ok(())
    }
}

// Implement Clone for async tasks
impl Clone for EnergyTradingPallet {
    fn clone(&self) -> Self {
        Self {
            buy_orders: self.buy_orders.clone(),
            sell_orders: self.sell_orders.clone(),
            order_history: self.order_history.clone(),
            trade_history: self.trade_history.clone(),
            running: self.running.clone(),
        }
    }
}
