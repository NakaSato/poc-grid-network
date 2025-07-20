//! # Continuous Double Auction Engine - Optimized
//! 
//! Implements a sophisticated but streamlined Continuous Double Auction (CDA) system for energy trading.
//! This optimized version uses modular components for better maintainability and performance.
//! 
//! ## Key Features:
//! - Modular architecture with separated concerns
//! - Efficient order matching with price-time priority
//! - Real-time market data and event streaming
//! - Comprehensive fee calculation and management
//! - Location-aware energy trading

use crate::{
    runtime::cda::{
        fees::FeeCalculator,
        market_data::MarketDataManager,
        matching::MatchingEngine,
        orders::OrderManager,
        types::*,
    },
    types::*,
    utils::SystemResult,
};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Main Continuous Double Auction Engine - Optimized Implementation
pub struct ContinuousDoubleAuction {
    /// Buy orders organized by price (highest first) then time
    bid_book: Arc<RwLock<BTreeMap<OrderedFloat, VecDeque<CDAOrder>>>>,
    /// Sell orders organized by price (lowest first) then time
    ask_book: Arc<RwLock<BTreeMap<OrderedFloat, VecDeque<CDAOrder>>>>,
    /// Order lifecycle manager
    order_manager: Arc<RwLock<OrderManager>>,
    /// Trade execution history (limited to recent trades for memory efficiency)
    trades: Arc<RwLock<VecDeque<TradeExecution>>>,
    /// Maximum trades to keep in memory
    max_trades_in_memory: usize,
    /// Matching algorithm configuration
    matching_algorithm: Arc<RwLock<MatchingAlgorithm>>,
    /// Event broadcast channel
    event_sender: broadcast::Sender<OrderBookEvent>,
    /// Running state
    running: Arc<RwLock<bool>>,
    /// Market data manager
    market_data_manager: Arc<MarketDataManager>,
    /// Fee calculator
    fee_calculator: Arc<FeeCalculator>,
}

impl ContinuousDoubleAuction {
    /// Create a new optimized Continuous Double Auction engine
    pub async fn new() -> SystemResult<Self> {
        Self::with_config(1000, 10000).await
    }
    
    /// Create CDA with custom configuration
    pub async fn with_config(event_buffer: usize, max_trades: usize) -> SystemResult<Self> {
        let (event_sender, _) = broadcast::channel(event_buffer);
        
        Ok(Self {
            bid_book: Arc::new(RwLock::new(BTreeMap::new())),
            ask_book: Arc::new(RwLock::new(BTreeMap::new())),
            order_manager: Arc::new(RwLock::new(OrderManager::new())),
            trades: Arc::new(RwLock::new(VecDeque::with_capacity(max_trades))),
            max_trades_in_memory: max_trades,
            matching_algorithm: Arc::new(RwLock::new(MatchingAlgorithm::PriceTimeProRata)),
            event_sender,
            running: Arc::new(RwLock::new(false)),
            market_data_manager: Arc::new(MarketDataManager::new()),
            fee_calculator: Arc::new(FeeCalculator::new()),
        })
    }
    
    /// Start the CDA engine with background tasks
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Optimized Continuous Double Auction Engine");
        
        // Start lightweight background tasks
        self.start_maintenance_task().await?;
        
        Ok(())
    }
    
    /// Stop the CDA engine
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Optimized CDA Engine");
        Ok(())
    }
    
    /// Submit a new order to the auction (main public interface)
    pub async fn submit_order(&self, base_order: EnergyOrder) -> SystemResult<Vec<TradeExecution>> {
        let order = self.create_cda_order(base_order)?;
        self.process_order(order).await
    }
    
    /// Cancel an order
    pub async fn cancel_order(&self, order_id: Uuid) -> SystemResult<bool> {
        let order = {
            let mut manager = self.order_manager.write().await;
            manager.remove_order(&order_id)
        };
        
        if let Some(order) = order {
            self.remove_from_book(&order).await?;
            let _ = self.event_sender.send(OrderBookEvent::OrderCancelled(order_id));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Validate order before processing
    fn validate_order(&self, order: &CDAOrder) -> SystemResult<()> {
        // Basic validation
        if order.remaining_quantity <= 0.0 {
            return Err(crate::utils::SystemError::InvalidOrder("Order quantity must be positive".to_string()));
        }
        
        if order.price <= 0.0 {
            return Err(crate::utils::SystemError::InvalidOrder("Order price must be positive".to_string()));
        }
        
        if order.account_id.is_empty() {
            return Err(crate::utils::SystemError::InvalidOrder("Order must have a valid account ID".to_string()));
        }
        
        // Additional validations can be added here
        Ok(())
    }
    
    /// Get current market depth
    pub async fn get_market_depth(&self, levels: usize) -> SystemResult<MarketDepth> {
        let bid_book = self.bid_book.read().await;
        let ask_book = self.ask_book.read().await;
        
        self.market_data_manager.generate_market_depth(&bid_book, &ask_book, levels)
    }
    
    /// Get recent trade history (memory efficient)
    pub async fn get_recent_trades(&self, limit: Option<usize>) -> SystemResult<Vec<TradeExecution>> {
        let trades = self.trades.read().await;
        let limit = limit.unwrap_or(trades.len().min(100)); // Default limit to prevent large responses
        Ok(trades.iter().rev().take(limit).cloned().collect())
    }
    
    /// Subscribe to order book events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<OrderBookEvent> {
        self.event_sender.subscribe()
    }
    
    // Private helper methods - optimized implementations
    
    /// Create CDA order from base order
    fn create_cda_order(&self, base_order: EnergyOrder) -> SystemResult<CDAOrder> {
        let now = chrono::Utc::now();
        Ok(CDAOrder {
            base: base_order.clone(),
            original_quantity: base_order.energy_amount,
            filled_quantity: 0.0,
            remaining_quantity: base_order.energy_amount,
            is_hidden: false,
            time_in_force: TimeInForce::GTC,
            priority_timestamp: now,
            ice_berg_quantity: None,
            // Direct field access compatibility
            id: base_order.id,
            account_id: base_order.account_id,
            order_type: base_order.order_type,
            energy_amount: base_order.energy_amount,
            price: base_order.price_per_unit as f64,
            grid_location: base_order.location,
            energy_source: base_order.energy_source.unwrap_or(EnergySource::Solar),
            filled_amount: 0.0,
            minimum_fill: None,
            post_only: false,
        })
    }
    
    /// Process order through matching engine (streamlined)
    async fn process_order(&self, mut order: CDAOrder) -> SystemResult<Vec<TradeExecution>> {
        // Validate order
        self.validate_order(&order)?;
        
        // Execute matching based on order type
        let executions = match order.base.order_type {
            OrderType::Buy => self.match_buy_order_optimized(&mut order).await?,
            OrderType::Sell => self.match_sell_order_optimized(&mut order).await?,
        };
        
        // Add remaining order to book if needed
        if order.remaining_quantity > 0.0 && !matches!(order.time_in_force, TimeInForce::IOC | TimeInForce::FOK) {
            self.add_to_book(order).await?;
        }
        
        // Store executions efficiently
        if !executions.is_empty() {
            self.store_executions_efficiently(&executions).await?;
        }
        
        Ok(executions)
    }
    
    /// Optimized buy order matching - simplified algorithm
    async fn match_buy_order_optimized(&self, order: &mut CDAOrder) -> SystemResult<Vec<TradeExecution>> {
        let mut executions = Vec::new();
        let mut ask_book = self.ask_book.write().await;
        
        // Use efficient price range query
        let order_price = OrderedFloat::from(order.base.price_per_unit as f64);
        let matching_prices: Vec<_> = ask_book.range(..=order_price).map(|(p, _)| *p).collect();
        
        for price in matching_prices {
            if order.remaining_quantity <= 0.0 { break; }
            
            if let Some(ask_orders) = ask_book.get_mut(&price) {
                // Process matches at this price level
                while let Some(mut ask_order) = ask_orders.pop_front() {
                    if order.remaining_quantity <= 0.0 { break; }
                    
                    if self.market_data_manager.are_orders_compatible(&order.base, &ask_order.base) {
                        let trade_quantity = order.remaining_quantity.min(ask_order.remaining_quantity);
                        let execution = self.create_execution(order, &mut ask_order, price.into(), trade_quantity, true).await?;
                        executions.push(execution);
                        
                        // Update quantities
                        order.remaining_quantity -= trade_quantity;
                        ask_order.remaining_quantity -= trade_quantity;
                        
                        // Put back if not fully filled
                        if ask_order.remaining_quantity > 0.0 {
                            ask_orders.push_front(ask_order);
                        }
                    } else {
                        ask_orders.push_back(ask_order); // Put non-matching order back
                    }
                }
                
                if ask_orders.is_empty() {
                    ask_book.remove(&price);
                }
            }
        }
        
        Ok(executions)
    }
    
    /// Optimized sell order matching - simplified algorithm
    async fn match_sell_order_optimized(&self, order: &mut CDAOrder) -> SystemResult<Vec<TradeExecution>> {
        let mut executions = Vec::new();
        let mut bid_book = self.bid_book.write().await;
        
        // Use efficient price range query (reverse for bids)
        let order_price = OrderedFloat::from(order.base.price_per_unit as f64);
        let matching_prices: Vec<_> = bid_book.range(order_price..).map(|(p, _)| *p).collect();
        
        for price in matching_prices.into_iter().rev() {
            if order.remaining_quantity <= 0.0 { break; }
            
            if let Some(bid_orders) = bid_book.get_mut(&price) {
                // Process matches at this price level
                while let Some(mut bid_order) = bid_orders.pop_front() {
                    if order.remaining_quantity <= 0.0 { break; }
                    
                    if self.market_data_manager.are_orders_compatible(&order.base, &bid_order.base) {
                        let trade_quantity = order.remaining_quantity.min(bid_order.remaining_quantity);
                        let execution = self.create_execution(&mut bid_order, order, price.into(), trade_quantity, false).await?;
                        executions.push(execution);
                        
                        // Update quantities
                        order.remaining_quantity -= trade_quantity;
                        bid_order.remaining_quantity -= trade_quantity;
                        
                        // Put back if not fully filled
                        if bid_order.remaining_quantity > 0.0 {
                            bid_orders.push_front(bid_order);
                        }
                    } else {
                        bid_orders.push_back(bid_order); // Put non-matching order back
                    }
                }
                
                if bid_orders.is_empty() {
                    bid_book.remove(&price);
                }
            }
        }
        
        Ok(executions)
    }
    
    /// Create trade execution record
    async fn create_execution(
        &self,
        buy_order: &mut CDAOrder,
        sell_order: &mut CDAOrder,
        price: f64,
        quantity: f64,
        is_aggressive_buy: bool,
    ) -> SystemResult<TradeExecution> {
        let fees = self.fee_calculator.calculate_fees(price, quantity, is_aggressive_buy)?;
        let execution_time = chrono::Utc::now();
        
        Ok(TradeExecution {
            trade_id: Uuid::new_v4(),
            buy_order_id: buy_order.base.id,
            sell_order_id: sell_order.base.id,
            price,
            quantity,
            buyer_id: buy_order.base.account_id.clone(),
            seller_id: sell_order.base.account_id.clone(),
            execution_time,
            is_aggressive_buy,
            location: sell_order.base.location.clone(),
            energy_source: sell_order.base.energy_source.clone().unwrap_or(EnergySource::Mixed),
            fees,
            // Additional fields for compatibility
            buyer: buy_order.base.account_id.clone(),
            seller: sell_order.base.account_id.clone(),
            energy_amount: quantity,
            grid_location: sell_order.base.location.clone(),
            executed_at: execution_time,
            settlement_status: crate::runtime::cda::types::SettlementStatus::Pending,
        })
    }
    
    /// Add order to appropriate book
    async fn add_to_book(&self, order: CDAOrder) -> SystemResult<()> {
        let price = OrderedFloat::from(order.base.price_per_unit as f64);
        
        // Add to order manager
        {
            let mut manager = self.order_manager.write().await;
            manager.add_order(order.clone())?;
        }
        
        // Add to appropriate book
        match order.base.order_type {
            OrderType::Buy => {
                let mut bid_book = self.bid_book.write().await;
                bid_book.entry(price).or_default().push_back(order.clone());
            },
            OrderType::Sell => {
                let mut ask_book = self.ask_book.write().await;
                ask_book.entry(price).or_default().push_back(order.clone());
            }
        }
        
        let _ = self.event_sender.send(OrderBookEvent::OrderAdded(order));
        Ok(())
    }
    
    /// Remove order from book
    async fn remove_from_book(&self, order: &CDAOrder) -> SystemResult<()> {
        let price = OrderedFloat::from(order.base.price_per_unit as f64);
        
        match order.base.order_type {
            OrderType::Buy => {
                let mut bid_book = self.bid_book.write().await;
                if let Some(orders) = bid_book.get_mut(&price) {
                    orders.retain(|o| o.base.id != order.base.id);
                    if orders.is_empty() {
                        bid_book.remove(&price);
                    }
                }
            },
            OrderType::Sell => {
                let mut ask_book = self.ask_book.write().await;
                if let Some(orders) = ask_book.get_mut(&price) {
                    orders.retain(|o| o.base.id != order.base.id);
                    if orders.is_empty() {
                        ask_book.remove(&price);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Store executions efficiently with memory management
    async fn store_executions_efficiently(&self, executions: &[TradeExecution]) -> SystemResult<()> {
        let mut trades = self.trades.write().await;
        
        for execution in executions {
            // Maintain memory limit efficiently
            if trades.len() >= self.max_trades_in_memory {
                trades.pop_front();
            }
            trades.push_back(execution.clone());
            
            // Broadcast individual execution events
            let _ = self.event_sender.send(OrderBookEvent::OrderExecuted(execution.clone()));
        }
        
        Ok(())
    }
    
    /// Lightweight maintenance task
    async fn start_maintenance_task(&self) -> SystemResult<()> {
        let order_manager = Arc::clone(&self.order_manager);
        let event_sender = self.event_sender.clone();
        let running = Arc::clone(&self.running);
        
        tokio::spawn(async move {
            while *running.read().await {
                let current_time = crate::utils::now();
                let expired_orders = {
                    let mut manager = order_manager.write().await;
                    manager.cleanup_expired(current_time)
                };
                
                for order_id in expired_orders {
                    let _ = event_sender.send(OrderBookEvent::OrderCancelled(order_id));
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
        
        Ok(())
    }
}

// Re-export types for external use
pub use crate::runtime::cda::types::{
    MarketDepth, OrderBookEvent, TradeExecution, CDAOrder, TradeFees, 
    TimeInForce, MatchingAlgorithm, OrderBookLevel, OrderedFloat, OrderStatus, SettlementStatus
};

impl Clone for ContinuousDoubleAuction {
    fn clone(&self) -> Self {
        Self {
            bid_book: Arc::clone(&self.bid_book),
            ask_book: Arc::clone(&self.ask_book),
            order_manager: Arc::clone(&self.order_manager),
            trades: Arc::clone(&self.trades),
            max_trades_in_memory: self.max_trades_in_memory,
            matching_algorithm: Arc::clone(&self.matching_algorithm),
            event_sender: self.event_sender.clone(),
            running: Arc::clone(&self.running),
            market_data_manager: Arc::clone(&self.market_data_manager),
            fee_calculator: Arc::clone(&self.fee_calculator),
        }
    }
}
