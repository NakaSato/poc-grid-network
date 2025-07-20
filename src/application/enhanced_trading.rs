//! # Enhanced Trading Service with CDA Integration
//! 
//! This module provides a high-level trading service that integrates the
//! Continuous Double Auction engine with the existing energy trading infrastructure.

use crate::blockchain::node::BlockchainNode;
use crate::infrastructure::database::DatabaseManager;
use crate::infrastructure::grid::GridManager;
use crate::runtime::continuous_double_auction::{
    ContinuousDoubleAuction, MarketDepth, OrderBookEvent, TradeExecution
};
use crate::types::*;
use crate::utils::SystemResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Enhanced trading service with CDA integration
pub struct EnhancedTradingService {
    /// Continuous Double Auction engine
    cda_engine: Arc<ContinuousDoubleAuction>,
    /// Blockchain node for transaction recording
    blockchain_node: Option<Arc<BlockchainNode>>,
    /// Database manager for persistence
    database_manager: Option<Arc<DatabaseManager>>,
    /// Grid manager for capacity checks
    grid_manager: Option<Arc<GridManager>>,
    /// Real-time market data cache
    market_data_cache: Arc<RwLock<HashMap<String, MarketData>>>,
    /// Event listeners
    event_receiver: Arc<RwLock<Option<broadcast::Receiver<OrderBookEvent>>>>,
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl EnhancedTradingService {
    /// Create a new enhanced trading service
    pub async fn new(
        blockchain_node: Arc<BlockchainNode>,
        database_manager: Arc<DatabaseManager>,
        grid_manager: Arc<GridManager>,
    ) -> SystemResult<Self> {
        let cda_engine = Arc::new(ContinuousDoubleAuction::new().await?);
        let event_receiver = Some(cda_engine.subscribe_to_events());
        
        Ok(Self {
            cda_engine,
            blockchain_node: Some(blockchain_node),
            database_manager: Some(database_manager),
            grid_manager: Some(grid_manager),
            market_data_cache: Arc::new(RwLock::new(HashMap::new())),
            event_receiver: Arc::new(RwLock::new(event_receiver)),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Create a placeholder instance for testing
    pub async fn new_placeholder() -> SystemResult<Self> {
        let cda_engine = Arc::new(ContinuousDoubleAuction::new().await?);
        let event_receiver = Some(cda_engine.subscribe_to_events());
        
        Ok(Self {
            cda_engine,
            blockchain_node: None,
            database_manager: None,
            grid_manager: None,
            market_data_cache: Arc::new(RwLock::new(HashMap::new())),
            event_receiver: Arc::new(RwLock::new(event_receiver)),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Start the enhanced trading service
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Enhanced Trading Service");
        
        // Start CDA engine
        self.cda_engine.start().await?;
        
        // Start background services
        self.start_event_processor().await?;
        self.start_market_data_updater().await?;
        self.start_settlement_processor().await?;
        
        Ok(())
    }
    
    /// Stop the enhanced trading service
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Enhanced Trading Service");
        
        // Stop CDA engine
        self.cda_engine.stop().await?;
        
        Ok(())
    }
    
    /// Place a new energy order using CDA
    pub async fn place_order(&self, order: EnergyOrder) -> SystemResult<PlaceOrderResult> {
        // Validate order
        self.validate_order(&order).await?;
        
        // Check grid capacity
        self.check_grid_capacity(&order).await?;
        
        // Submit to CDA engine
        let executions = self.cda_engine.submit_order(order.clone()).await?;
        
        // Process executions
        let mut trades = Vec::new();
        for execution in executions {
            // Convert to energy trade
            let trade = self.convert_execution_to_trade(&execution).await?;
            
            // Store trade in database
            self.store_trade(&trade).await?;
            
            // Submit to blockchain
            self.submit_trade_to_blockchain(&trade).await?;
            
            trades.push(trade);
        }
        
        crate::utils::logging::log_info(
            "EnhancedTradingService",
            &format!("Order processed: {} - {} executions", order.id, trades.len())
        );
        
        Ok(PlaceOrderResult {
            order_id: order.id,
            executions: trades.clone(),
            status: if trades.is_empty() { 
                OrderStatus::Pending 
            } else { 
                OrderStatus::PartiallyFilled 
            },
        })
    }
    
    /// Cancel an existing order
    pub async fn cancel_order(&self, order_id: Uuid, account_id: AccountId) -> SystemResult<()> {
        // Verify order ownership (if database is available)
        if let Some(_db) = &self.database_manager {
            self.verify_order_ownership(order_id, &account_id).await?;
        }
        
        // Cancel in CDA engine
        self.cda_engine.cancel_order(order_id).await?;
        
        crate::utils::logging::log_info(
            "EnhancedTradingService",
            &format!("Order cancelled: {}", order_id)
        );
        
        Ok(())
    }
    
    /// Get current market depth for a location
    pub async fn get_market_depth(&self, location: &GridLocation, levels: usize) -> SystemResult<MarketDepth> {
        // For now, return global market depth
        // In a full implementation, this would filter by location
        self.cda_engine.get_market_depth(levels).await
    }
    
    /// Get recent trades for a location
    pub async fn get_recent_trades(&self, location: &GridLocation, limit: Option<usize>) -> SystemResult<Vec<TradeExecution>> {
        // For now, return all trades
        // In a full implementation, this would filter by location
        self.cda_engine.get_recent_trades(limit).await
    }
    
    /// Get enhanced market data with CDA integration
    pub async fn get_market_data(&self, location: &GridLocation) -> SystemResult<MarketData> {
        let location_key = format!("{}_{}", location.province, location.district);
        
        // Check cache first
        {
            let cache = self.market_data_cache.read().await;
            if let Some(cached_data) = cache.get(&location_key) {
                // Return cached data if it's recent (within last 30 seconds)
                let cache_age = crate::utils::now().timestamp() - cached_data.timestamp.timestamp();
                if cache_age < 30 {
                    return Ok(cached_data.clone());
                }
            }
        }
        
        // Generate fresh market data
        let market_data = self.generate_market_data(location).await?;
        
        // Update cache
        {
            let mut cache = self.market_data_cache.write().await;
            cache.insert(location_key, market_data.clone());
        }
        
        Ok(market_data)
    }
    
    /// Get user's trading history
    pub async fn get_user_trades(&self, account_id: &AccountId) -> SystemResult<Vec<EnergyTrade>> {
        // Get all trades from CDA
        let executions = self.cda_engine.get_recent_trades(None).await?;
        
        // Filter and convert to energy trades
        let mut user_trades = Vec::new();
        for execution in executions {
            if execution.buyer_id == *account_id || execution.seller_id == *account_id {
                let trade = self.convert_execution_to_trade(&execution).await?;
                user_trades.push(trade);
            }
        }
        
        Ok(user_trades)
    }
    
    /// Get order book for display
    pub async fn get_order_book(&self, location: &GridLocation, energy_source: Option<EnergySource>) -> SystemResult<(Vec<EnergyOrder>, Vec<EnergyOrder>)> {
        let depth = self.get_market_depth(location, 10).await?;
        
        // Convert market depth to order format for compatibility
        let mut buy_orders = Vec::new();
        let mut sell_orders = Vec::new();
        
        for bid in depth.bids {
            let order = EnergyOrder {
                id: Uuid::new_v4(),
                order_type: OrderType::Buy,
                energy_amount: bid.total_quantity,
                price_per_unit: bid.price as Balance,
                location: location.clone(),
                energy_source: energy_source.clone(),
                timestamp: bid.timestamp,
                status: OrderStatus::Pending,
                account_id: "aggregated".to_string(),
                updated_at: bid.timestamp,
            };
            buy_orders.push(order);
        }
        
        for ask in depth.asks {
            let order = EnergyOrder {
                id: Uuid::new_v4(),
                order_type: OrderType::Sell,
                energy_amount: ask.total_quantity,
                price_per_unit: ask.price as Balance,
                location: location.clone(),
                energy_source: energy_source.clone(),
                timestamp: ask.timestamp,
                status: OrderStatus::Pending,
                account_id: "aggregated".to_string(),
                updated_at: ask.timestamp,
            };
            sell_orders.push(order);
        }
        
        Ok((buy_orders, sell_orders))
    }
    
    /// Subscribe to real-time market events
    pub fn subscribe_to_market_events(&self) -> broadcast::Receiver<OrderBookEvent> {
        self.cda_engine.subscribe_to_events()
    }
    
    // Private helper methods
    
    /// Convert trade execution to energy trade
    async fn convert_execution_to_trade(&self, execution: &TradeExecution) -> SystemResult<EnergyTrade> {
        Ok(EnergyTrade {
            trade_id: execution.trade_id.to_string(),
            energy_amount: execution.quantity,
            price_per_unit: execution.price as Balance,
            buyer_id: execution.buyer_id.clone(),
            seller_id: execution.seller_id.clone(),
            timestamp: execution.execution_time.timestamp() as u64,
            status: TradeStatus::Confirmed,
            grid_location: execution.location.clone(),
            // Legacy field mappings
            id: execution.trade_id.to_string(),
            buy_order_id: execution.buy_order_id.to_string(),
            sell_order_id: execution.sell_order_id.to_string(),
            price_per_kwh: execution.price as Balance,
            total_price: (execution.price * execution.quantity) as Balance,
            grid_fee: execution.fees.grid_fee as Balance,
            energy_source: execution.energy_source.clone(),
            carbon_offset: CarbonOffset {
                offset_credits: execution.quantity * 0.5, // Simplified calculation
                verified: true,
                certification_body: "Thai Energy Authority".to_string(),
                timestamp: execution.execution_time,
            },
        })
    }
    
    /// Generate market data from CDA state
    async fn generate_market_data(&self, location: &GridLocation) -> SystemResult<MarketData> {
        let depth = self.get_market_depth(location, 1).await?;
        let trades = self.get_recent_trades(location, Some(100)).await?;
        
        // Calculate market statistics
        let current_price = if let Some(last_trade) = trades.first() {
            last_trade.price
        } else {
            let (best_bid, best_ask) = (depth.bids.first().map(|b| b.price), depth.asks.first().map(|a| a.price));
            match (best_bid, best_ask) {
                (Some(bid), Some(ask)) => (bid + ask) / 2.0,
                (Some(bid), None) => bid,
                (None, Some(ask)) => ask,
                (None, None) => 5000.0, // Default price
            }
        };
        
        let volume_24h = self.calculate_24h_volume(&trades).await?;
        let (high_24h, low_24h) = self.calculate_24h_range(&trades).await?;
        let price_change_24h = self.calculate_price_change(&trades).await?;
        let trades_24h = trades.len() as u32;
        
        let price_trend = if price_change_24h > 0.01 {
            PriceTrend::Rising
        } else if price_change_24h < -0.01 {
            PriceTrend::Falling
        } else {
            PriceTrend::Stable
        };
        
        Ok(MarketData {
            energy_source: EnergySource::Mixed,
            location: location.clone(),
            current_price,
            price_trend,
            volume_24h,
            price_change_24h,
            trades_24h,
            high_24h,
            low_24h,
            timestamp: crate::utils::now(),
        })
    }
    
    /// Calculate 24-hour volume from trades
    async fn calculate_24h_volume(&self, trades: &[TradeExecution]) -> SystemResult<EnergyAmount> {
        let now = crate::utils::now();
        let day_ago = now - chrono::Duration::hours(24);
        
        let volume = trades.iter()
            .filter(|t| t.execution_time >= day_ago)
            .map(|t| t.quantity)
            .sum();
        
        Ok(volume)
    }
    
    /// Calculate 24-hour price range
    async fn calculate_24h_range(&self, trades: &[TradeExecution]) -> SystemResult<(TokenPrice, TokenPrice)> {
        let now = crate::utils::now();
        let day_ago = now - chrono::Duration::hours(24);
        
        let recent_trades: Vec<_> = trades.iter()
            .filter(|t| t.execution_time >= day_ago)
            .collect();
        
        if recent_trades.is_empty() {
            return Ok((5000.0, 5000.0)); // Default range
        }
        
        let prices: Vec<TokenPrice> = recent_trades.iter().map(|t| t.price).collect();
        let high = prices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let low = prices.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        Ok((high, low))
    }
    
    /// Calculate price change over 24 hours
    async fn calculate_price_change(&self, trades: &[TradeExecution]) -> SystemResult<f64> {
        if trades.len() < 2 {
            return Ok(0.0);
        }
        
        let now = crate::utils::now();
        let day_ago = now - chrono::Duration::hours(24);
        
        let recent_trades: Vec<_> = trades.iter()
            .filter(|t| t.execution_time >= day_ago)
            .collect();
        
        if recent_trades.len() < 2 {
            return Ok(0.0);
        }
        
        let current_price = recent_trades.first().unwrap().price;
        let old_price = recent_trades.last().unwrap().price;
        
        if old_price == 0.0 {
            return Ok(0.0);
        }
        
        Ok((current_price - old_price) / old_price)
    }
    
    /// Start event processing background task
    async fn start_event_processor(&self) -> SystemResult<()> {
        let mut receiver = {
            let mut event_receiver = self.event_receiver.write().await;
            if let Some(receiver) = event_receiver.take() {
                receiver
            } else {
                return Ok(()); // No receiver available
            }
        };
        
        let self_clone = self.clone();
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                if let Err(e) = self_clone.process_event(event).await {
                    crate::utils::logging::log_error("EnhancedTradingService", &e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Process order book events
    async fn process_event(&self, event: OrderBookEvent) -> SystemResult<()> {
        match event {
            OrderBookEvent::OrderExecuted(execution) => {
                // Handle trade execution
                let trade = self.convert_execution_to_trade(&execution).await?;
                self.store_trade(&trade).await?;
                self.submit_trade_to_blockchain(&trade).await?;
            },
            _ => {
                // Handle other events as needed
            }
        }
        Ok(())
    }
    
    /// Start market data updater
    async fn start_market_data_updater(&self) -> SystemResult<()> {
        let self_clone = self.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = self_clone.update_market_data_cache().await {
                    crate::utils::logging::log_error("EnhancedTradingService", &e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            }
        });
        Ok(())
    }
    
    /// Update market data cache
    async fn update_market_data_cache(&self) -> SystemResult<()> {
        // This would update cached market data for all active locations
        // For now, we'll just log that it's running
        crate::utils::logging::log_info("EnhancedTradingService", "Market data cache updated");
        Ok(())
    }
    
    /// Start settlement processor
    async fn start_settlement_processor(&self) -> SystemResult<()> {
        let self_clone = self.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = self_clone.process_settlements().await {
                    crate::utils::logging::log_error("EnhancedTradingService", &e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
        Ok(())
    }
    
    /// Process settlements
    async fn process_settlements(&self) -> SystemResult<()> {
        // This would handle settlement processing
        // For now, we'll just log that it's running
        crate::utils::logging::log_info("EnhancedTradingService", "Settlements processed");
        Ok(())
    }
    
    /// Validate order
    async fn validate_order(&self, order: &EnergyOrder) -> SystemResult<()> {
        if order.energy_amount <= 0.0 {
            return Err(crate::utils::SystemError::Validation("Invalid energy amount".to_string()));
        }
        
        if (order.price_per_unit as f64) <= 0.0 {
            return Err(crate::utils::SystemError::Validation("Invalid price".to_string()));
        }
        
        Ok(())
    }
    
    /// Check grid capacity
    async fn check_grid_capacity(&self, _order: &EnergyOrder) -> SystemResult<()> {
        // Implementation would check actual grid capacity
        // For now, always return OK
        Ok(())
    }
    
    /// Store trade in database
    async fn store_trade(&self, _trade: &EnergyTrade) -> SystemResult<()> {
        // Implementation would store in database
        // For now, always return OK
        Ok(())
    }
    
    /// Submit trade to blockchain
    async fn submit_trade_to_blockchain(&self, _trade: &EnergyTrade) -> SystemResult<()> {
        // Implementation would submit to blockchain
        // For now, always return OK
        Ok(())
    }
    
    /// Verify order ownership
    async fn verify_order_ownership(&self, _order_id: Uuid, _account_id: &AccountId) -> SystemResult<()> {
        // Implementation would verify ownership
        // For now, always return OK
        Ok(())
    }
}

/// Result of placing an order
#[derive(Debug, Clone)]
pub struct PlaceOrderResult {
    pub order_id: Uuid,
    pub executions: Vec<EnergyTrade>,
    pub status: OrderStatus,
}

impl Clone for EnhancedTradingService {
    fn clone(&self) -> Self {
        Self {
            cda_engine: self.cda_engine.clone(),
            blockchain_node: self.blockchain_node.clone(),
            database_manager: self.database_manager.clone(),
            grid_manager: self.grid_manager.clone(),
            market_data_cache: self.market_data_cache.clone(),
            event_receiver: Arc::new(RwLock::new(None)), // Can't clone receiver
            running: self.running.clone(),
        }
    }
}
