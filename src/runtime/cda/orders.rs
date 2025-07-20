//! # CDA Order Management
//! 
//! Order lifecycle management and validation for the Continuous Double Auction system.

use super::types::*;
use crate::types::*;
use crate::utils::SystemResult;
use chrono::Utc;
use std::collections::{BTreeMap, HashMap, VecDeque};
use uuid::Uuid;

/// Order lifecycle manager for CDA operations
pub struct OrderManager {
    /// All orders indexed by ID for quick lookup
    orders: HashMap<Uuid, CDAOrder>,
    /// Orders by expiry time for maintenance
    expiry_index: BTreeMap<chrono::DateTime<Utc>, Vec<Uuid>>,
}

impl OrderManager {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            expiry_index: BTreeMap::new(),
        }
    }
    
    /// Add an order to the manager
    pub fn add_order(&mut self, order: CDAOrder) -> SystemResult<()> {
        // Add to expiry index if needed
        if let TimeInForce::GTT(expiry) = order.time_in_force {
            self.expiry_index.entry(expiry).or_default().push(order.base.id);
        }
        
        self.orders.insert(order.base.id, order);
        Ok(())
    }
    
    /// Get order by ID
    pub fn get_order(&self, order_id: &Uuid) -> Option<&CDAOrder> {
        self.orders.get(order_id)
    }
    
    /// Remove order
    pub fn remove_order(&mut self, order_id: &Uuid) -> Option<CDAOrder> {
        self.orders.remove(order_id)
    }
    
    /// Update order quantities after partial fill
    pub fn update_order_quantities(&mut self, order_id: &Uuid, filled_quantity: f64) -> SystemResult<()> {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.filled_quantity += filled_quantity;
            order.remaining_quantity -= filled_quantity;
        }
        Ok(())
    }
    
    /// Get expired orders
    pub fn get_expired_orders(&self, current_time: chrono::DateTime<Utc>) -> Vec<Uuid> {
        let mut expired = Vec::new();
        
        for (expiry_time, order_ids) in self.expiry_index.range(..=current_time) {
            expired.extend(order_ids.clone());
        }
        
        // Also check for day orders (simplified: expire after 24 hours)
        for order in self.orders.values() {
            if matches!(order.time_in_force, TimeInForce::DAY) {
                let order_age = current_time.timestamp() - order.base.timestamp.timestamp();
                if order_age > 86400 { // 24 hours
                    expired.push(order.base.id);
                }
            }
        }
        
        expired
    }
    
    /// Clean up expired orders
    pub fn cleanup_expired(&mut self, current_time: chrono::DateTime<Utc>) -> Vec<Uuid> {
        let expired = self.get_expired_orders(current_time);
        
        for order_id in &expired {
            self.remove_order(order_id);
        }
        
        // Clean up expiry index
        let expired_keys: Vec<_> = self.expiry_index.range(..=current_time).map(|(k, _)| *k).collect();
        for key in expired_keys {
            self.expiry_index.remove(&key);
        }
        
        expired
    }
}
