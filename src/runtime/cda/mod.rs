//! # Continuous Double Auction - Main Module
//! 
//! Modular CDA implementation split into focused submodules

pub mod types;
pub mod matching;
pub mod orders;
pub mod fees;
pub mod market_data;

use self::types::*;
use self::matching::MatchingEngine;
use crate::types::*;
use crate::utils::SystemResult;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

/// Main Continuous Double Auction engine
pub struct ContinuousDoubleAuction {
    /// Matching engines per grid location
    matching_engines: Arc<RwLock<HashMap<GridLocation, MatchingEngine>>>,
    /// Event broadcaster for real-time updates
    event_sender: broadcast::Sender<OrderBookEvent>,
    /// Event receiver
    event_receiver: broadcast::Receiver<OrderBookEvent>,
}

impl ContinuousDoubleAuction {
    /// Create a new CDA instance
    pub fn new() -> Self {
        let (event_sender, event_receiver) = broadcast::channel(1000);
        
        Self {
            matching_engines: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver,
        }
    }

    /// Place a new order in the CDA
    pub async fn place_order(&self, order: EnergyOrder) -> SystemResult<Vec<TradeExecution>> {
        let cda_order = CDAOrder {
            id: order.id,
            account_id: order.trader,
            order_type: order.order_type,
            energy_amount: order.energy_amount,
            price: order.price,
            grid_location: order.grid_location,
            energy_source: order.energy_source,
            time_in_force: TimeInForce::GoodTillCancelled, // Default
            created_at: Utc::now(),
            expires_at: order.expires_at,
            filled_amount: 0.0,
            status: CDAOrderStatus::Active,
            priority_timestamp: Utc::now(),
        };

        let mut engines = self.matching_engines.write().await;
        let engine = engines.entry(cda_order.grid_location.clone()).or_insert_with(MatchingEngine::new);
        
        let executions = engine.add_order(cda_order.clone())?;
        
        // Broadcast order placed event
        let _ = self.event_sender.send(OrderBookEvent::OrderPlaced(cda_order));
        
        // Broadcast trade executions
        for execution in &executions {
            let _ = self.event_sender.send(OrderBookEvent::OrderMatched(execution.clone()));
        }

        Ok(executions)
    }

    /// Cancel an existing order
    pub async fn cancel_order(&self, order_id: Uuid, grid_location: GridLocation) -> SystemResult<bool> {
        let mut engines = self.matching_engines.write().await;
        
        if let Some(engine) = engines.get_mut(&grid_location) {
            if let Some(_cancelled_order) = engine.cancel_order(order_id)? {
                // Broadcast cancellation event
                let _ = self.event_sender.send(OrderBookEvent::OrderCancelled {
                    order_id,
                    cancelled_at: Utc::now(),
                });
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Get market depth for a specific grid location
    pub async fn get_market_depth(&self, grid_location: &GridLocation, levels: usize) -> SystemResult<MarketDepth> {
        let engines = self.matching_engines.read().await;
        
        if let Some(engine) = engines.get(grid_location) {
            Ok(engine.get_market_depth(levels))
        } else {
            // Return empty market depth if no engine exists for this location
            Ok(MarketDepth {
                grid_location: grid_location.clone(),
                bids: Vec::new(),
                asks: Vec::new(),
                last_updated: Utc::now(),
            })
        }
    }

    /// Get best bid and ask prices
    pub async fn get_best_prices(&self, grid_location: &GridLocation) -> SystemResult<(Option<f64>, Option<f64>)> {
        let engines = self.matching_engines.read().await;
        
        if let Some(engine) = engines.get(grid_location) {
            Ok((engine.get_best_bid(), engine.get_best_ask()))
        } else {
            Ok((None, None))
        }
    }

    /// Subscribe to real-time events
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<OrderBookEvent> {
        self.event_sender.subscribe()
    }
}

impl Clone for ContinuousDoubleAuction {
    fn clone(&self) -> Self {
        Self {
            matching_engines: Arc::clone(&self.matching_engines),
            event_sender: self.event_sender.clone(),
            event_receiver: self.event_sender.subscribe(),
        }
    }
}

impl Default for ContinuousDoubleAuction {
    fn default() -> Self {
        Self::new()
    }
}
