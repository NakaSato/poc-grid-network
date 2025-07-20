//! # Continuous Double Auction - Matching Engine
//! 
//! Core order matching algorithms for the CDA system

use super::types::*;
use crate::types::*;
use crate::utils::SystemResult;
use chrono::Utc;
use std::collections::BTreeMap;
use uuid::Uuid;

/// Order matching engine with price-time priority
pub struct MatchingEngine {
    /// Buy orders sorted by price (highest first) then time (earliest first)
    buy_orders: BTreeMap<(OrderedFloat, i64), CDAOrder>,
    /// Sell orders sorted by price (lowest first) then time (earliest first)  
    sell_orders: BTreeMap<(OrderedFloat, i64), CDAOrder>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    /// Add an order to the order book
    pub fn add_order(&mut self, order: CDAOrder) -> SystemResult<Vec<TradeExecution>> {
        let mut executions = Vec::new();

        match order.order_type {
            OrderType::Buy => {
                // Try to match against sell orders
                executions = self.match_buy_order(&order)?;
                
                // Add remaining quantity to buy order book if not IOC/FOK
                if !executions.is_empty() || order.time_in_force != TimeInForce::ImmediateOrCancel {
                    let remaining_amount = order.energy_amount - executions.iter().map(|e| e.energy_amount).sum::<f64>();
                    if remaining_amount > 0.0 && order.time_in_force != TimeInForce::FillOrKill {
                        let mut remaining_order = order.clone();
                        remaining_order.energy_amount = remaining_amount;
                        self.insert_buy_order(remaining_order);
                    }
                }
            }
            OrderType::Sell => {
                // Try to match against buy orders
                executions = self.match_sell_order(&order)?;
                
                // Add remaining quantity to sell order book if not IOC/FOK
                if !executions.is_empty() || order.time_in_force != TimeInForce::ImmediateOrCancel {
                    let remaining_amount = order.energy_amount - executions.iter().map(|e| e.energy_amount).sum::<f64>();
                    if remaining_amount > 0.0 && order.time_in_force != TimeInForce::FillOrKill {
                        let mut remaining_order = order.clone();
                        remaining_order.energy_amount = remaining_amount;
                        self.insert_sell_order(remaining_order);
                    }
                }
            }
        }

        Ok(executions)
    }

    /// Cancel an order from the order book
    pub fn cancel_order(&mut self, order_id: Uuid) -> SystemResult<Option<CDAOrder>> {
        // Check buy orders
        if let Some(order) = self.remove_buy_order(order_id) {
            return Ok(Some(order));
        }
        
        // Check sell orders
        if let Some(order) = self.remove_sell_order(order_id) {
            return Ok(Some(order));
        }
        
        Ok(None)
    }

    /// Get current market depth
    pub fn get_market_depth(&self, levels: usize) -> MarketDepth {
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        // Aggregate buy orders by price
        let mut bid_levels: BTreeMap<OrderedFloat, (f64, usize)> = BTreeMap::new();
        for order in self.buy_orders.values() {
            let price_key = OrderedFloat(order.price);
            let (total_qty, count) = bid_levels.entry(price_key).or_insert((0.0, 0));
            *total_qty += order.energy_amount;
            *count += 1;
        }

        // Get top bid levels
        for (price, (total_quantity, order_count)) in bid_levels.iter().rev().take(levels) {
            bids.push(OrderBookLevel {
                price: price.0,
                total_quantity: *total_quantity,
                order_count: *order_count,
            });
        }

        // Aggregate sell orders by price
        let mut ask_levels: BTreeMap<OrderedFloat, (f64, usize)> = BTreeMap::new();
        for order in self.sell_orders.values() {
            let price_key = OrderedFloat(order.price);
            let (total_qty, count) = ask_levels.entry(price_key).or_insert((0.0, 0));
            *total_qty += order.energy_amount;
            *count += 1;
        }

        // Get top ask levels
        for (price, (total_quantity, order_count)) in ask_levels.iter().take(levels) {
            asks.push(OrderBookLevel {
                price: price.0,
                total_quantity: *total_quantity,
                order_count: *order_count,
            });
        }

        MarketDepth {
            grid_location: GridLocation::Bangkok, // TODO: Make this configurable per location
            bids,
            asks,
            last_updated: Utc::now(),
        }
    }

    /// Get best bid price
    pub fn get_best_bid(&self) -> Option<f64> {
        self.buy_orders.keys().next_back().map(|(price, _)| price.0)
    }

    /// Get best ask price  
    pub fn get_best_ask(&self) -> Option<f64> {
        self.sell_orders.keys().next().map(|(price, _)| price.0)
    }

    // Private helper methods
    
    fn match_buy_order(&mut self, buy_order: &CDAOrder) -> SystemResult<Vec<TradeExecution>> {
        let mut executions = Vec::new();
        let mut remaining_amount = buy_order.energy_amount;

        // Find matching sell orders (price <= buy_order.price)
        let matching_keys: Vec<_> = self.sell_orders
            .range(..(OrderedFloat(buy_order.price + 0.0001), i64::MAX))
            .map(|(key, _)| *key)
            .collect();

        for key in matching_keys {
            if remaining_amount <= 0.0 {
                break;
            }

            if let Some(sell_order) = self.sell_orders.get(&key) {
                let execution_amount = remaining_amount.min(sell_order.energy_amount);
                
                let execution = TradeExecution {
                    trade_id: Uuid::new_v4(),
                    buy_order_id: buy_order.id,
                    sell_order_id: sell_order.id,
                    buyer: buy_order.account_id.clone(),
                    seller: sell_order.account_id.clone(),
                    energy_amount: execution_amount,
                    price: sell_order.price, // Price improvement for buyer
                    grid_location: buy_order.grid_location.clone(),
                    energy_source: sell_order.energy_source.clone(),
                    executed_at: Utc::now(),
                    settlement_status: SettlementStatus::Pending,
                };

                executions.push(execution);
                remaining_amount -= execution_amount;

                // Update or remove the sell order
                if sell_order.energy_amount <= execution_amount {
                    self.sell_orders.remove(&key);
                } else {
                    let mut updated_order = sell_order.clone();
                    updated_order.energy_amount -= execution_amount;
                    updated_order.filled_amount += execution_amount;
                    self.sell_orders.insert(key, updated_order);
                }
            }
        }

        Ok(executions)
    }

    fn match_sell_order(&mut self, sell_order: &CDAOrder) -> SystemResult<Vec<TradeExecution>> {
        let mut executions = Vec::new();
        let mut remaining_amount = sell_order.energy_amount;

        // Find matching buy orders (price >= sell_order.price)
        let matching_keys: Vec<_> = self.buy_orders
            .range((OrderedFloat(sell_order.price), i64::MIN)..)
            .map(|(key, _)| *key)
            .collect();

        for key in matching_keys {
            if remaining_amount <= 0.0 {
                break;
            }

            if let Some(buy_order) = self.buy_orders.get(&key) {
                let execution_amount = remaining_amount.min(buy_order.energy_amount);
                
                let execution = TradeExecution {
                    trade_id: Uuid::new_v4(),
                    buy_order_id: buy_order.id,
                    sell_order_id: sell_order.id,
                    buyer: buy_order.account_id.clone(),
                    seller: sell_order.account_id.clone(),
                    energy_amount: execution_amount,
                    price: buy_order.price, // Price improvement for seller
                    grid_location: sell_order.grid_location.clone(),
                    energy_source: sell_order.energy_source.clone(),
                    executed_at: Utc::now(),
                    settlement_status: SettlementStatus::Pending,
                };

                executions.push(execution);
                remaining_amount -= execution_amount;

                // Update or remove the buy order
                if buy_order.energy_amount <= execution_amount {
                    self.buy_orders.remove(&key);
                } else {
                    let mut updated_order = buy_order.clone();
                    updated_order.energy_amount -= execution_amount;
                    updated_order.filled_amount += execution_amount;
                    self.buy_orders.insert(key, updated_order);
                }
            }
        }

        Ok(executions)
    }

    fn insert_buy_order(&mut self, order: CDAOrder) {
        let key = (OrderedFloat(-order.price), order.priority_timestamp.timestamp_nanos_opt().unwrap_or(0));
        self.buy_orders.insert(key, order);
    }

    fn insert_sell_order(&mut self, order: CDAOrder) {
        let key = (OrderedFloat(order.price), order.priority_timestamp.timestamp_nanos_opt().unwrap_or(0));
        self.sell_orders.insert(key, order);
    }

    fn remove_buy_order(&mut self, order_id: Uuid) -> Option<CDAOrder> {
        let key_to_remove = self.buy_orders
            .iter()
            .find(|(_, order)| order.id == order_id)
            .map(|(key, _)| *key)?;
        
        self.buy_orders.remove(&key_to_remove)
    }

    fn remove_sell_order(&mut self, order_id: Uuid) -> Option<CDAOrder> {
        let key_to_remove = self.sell_orders
            .iter()
            .find(|(_, order)| order.id == order_id)
            .map(|(key, _)| *key)?;
        
        self.sell_orders.remove(&key_to_remove)
    }
}
