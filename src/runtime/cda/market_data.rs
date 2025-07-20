//! # CDA Market Data
//! 
//! Market data generation and analysis for the Continuous Double Auction system.

use super::types::*;
use crate::types::*;
use crate::utils::SystemResult;
use std::collections::{BTreeMap, VecDeque};

/// Market data generator and analyzer
pub struct MarketDataManager {
    fee_calculator: super::fees::FeeCalculator,
}

impl MarketDataManager {
    pub fn new() -> Self {
        Self {
            fee_calculator: super::fees::FeeCalculator::new(),
        }
    }
    
    /// Generate market depth from order books
    pub fn generate_market_depth(
        &self,
        bid_book: &BTreeMap<OrderedFloat, VecDeque<CDAOrder>>,
        ask_book: &BTreeMap<OrderedFloat, VecDeque<CDAOrder>>,
        levels: usize,
    ) -> SystemResult<MarketDepth> {
        let mut bids = Vec::new();
        let mut asks = Vec::new();
        
        // Build bid levels (highest first)
        for (price, orders) in bid_book.iter().rev().take(levels) {
            let total_quantity: EnergyAmount = orders.iter().map(|o| o.remaining_quantity).sum();
            let order_count = orders.len() as u32;
            
            bids.push(OrderBookLevel {
                price: (*price).into(),
                total_quantity,
                order_count,
                timestamp: crate::utils::now(),
            });
        }
        
        // Build ask levels (lowest first)
        for (price, orders) in ask_book.iter().take(levels) {
            let total_quantity: EnergyAmount = orders.iter().map(|o| o.remaining_quantity).sum();
            let order_count = orders.len() as u32;
            
            asks.push(OrderBookLevel {
                price: (*price).into(),
                total_quantity,
                order_count,
                timestamp: crate::utils::now(),
            });
        }
        
        // Calculate spread and mid-price
        let best_bid = bids.first().map(|b| b.price).unwrap_or(0.0);
        let best_ask = asks.first().map(|a| a.price).unwrap_or(0.0);
        let spread = if best_bid > 0.0 && best_ask > 0.0 { best_ask - best_bid } else { 0.0 };
        let mid_price = if best_bid > 0.0 && best_ask > 0.0 { (best_bid + best_ask) / 2.0 } else { 0.0 };
        
        let total_bid_volume: EnergyAmount = bids.iter().map(|b| b.total_quantity).sum();
        let total_ask_volume: EnergyAmount = asks.iter().map(|a| a.total_quantity).sum();
        
        let now = chrono::Utc::now();
        let default_location = GridLocation {
            province: "Default".to_string(),
            district: "Default".to_string(),
            coordinates: (0.0, 0.0),
            region: "Default".to_string(),
            substation: "Default".to_string(),
            grid_code: "DEFAULT".to_string(),
            meter_id: "DEFAULT".to_string(),
        };
        
        Ok(MarketDepth {
            bids,
            asks,
            spread,
            mid_price,
            total_bid_volume,
            total_ask_volume,
            timestamp: now,
            // Additional fields for compatibility
            grid_location: default_location,
            last_updated: now,
        })
    }
    
    /// Check if two orders are compatible for matching
    pub fn are_orders_compatible(&self, order1: &EnergyOrder, order2: &EnergyOrder) -> bool {
        // Check grid location compatibility
        if order1.location.province != order2.location.province {
            return false;
        }
        
        // Check energy source compatibility
        if let (Some(source1), Some(source2)) = (&order1.energy_source, &order2.energy_source) {
            if source1 != source2 && *source1 != EnergySource::Mixed && *source2 != EnergySource::Mixed {
                return false;
            }
        }
        
        true
    }
    
    /// Calculate fees for a trade (delegated to fee calculator)
    pub fn calculate_fees(&self, price: TokenPrice, quantity: EnergyAmount, is_taker: bool) -> SystemResult<TradeFees> {
        self.fee_calculator.calculate_fees(price, quantity, is_taker)
    }
}

impl Default for MarketDataManager {
    fn default() -> Self {
        Self::new()
    }
}
