//! # Continuous Double Auction - Types Module
//! 
//! Core data structures and types for the CDA system

use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Order book level representing price and total quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: TokenPrice,
    pub total_quantity: EnergyAmount,
    pub order_count: u32,
    pub timestamp: DateTime<Utc>,
}

/// Market depth information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDepth {
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub spread: TokenPrice,
    pub mid_price: TokenPrice,
    pub total_bid_volume: EnergyAmount,
    pub total_ask_volume: EnergyAmount,
    pub timestamp: DateTime<Utc>,
    // Additional fields for compatibility
    pub grid_location: GridLocation,
    pub last_updated: DateTime<Utc>,
}

/// Matching algorithm types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatchingAlgorithm {
    /// First-In-First-Out - orders matched in arrival order
    FIFO,
    /// Pro-rata - proportional allocation based on order size
    ProRata,
    /// Price-time priority with pro-rata for same price levels
    PriceTimeProRata,
}

/// Trade execution record with enhanced details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    pub trade_id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub price: TokenPrice,
    pub quantity: EnergyAmount,
    pub buyer_id: AccountId,
    pub seller_id: AccountId,
    pub execution_time: DateTime<Utc>,
    pub is_aggressive_buy: bool,
    pub location: GridLocation,
    pub energy_source: EnergySource,
    pub fees: TradeFees,
    // Additional fields for compatibility
    pub buyer: AccountId,
    pub seller: AccountId,
    pub energy_amount: EnergyAmount,
    pub grid_location: GridLocation,
    pub executed_at: DateTime<Utc>,
    pub settlement_status: SettlementStatus,
}

/// CDA-specific order with additional fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDAOrder {
    pub base: EnergyOrder,
    pub original_quantity: EnergyAmount,
    pub filled_quantity: EnergyAmount,
    pub remaining_quantity: EnergyAmount,
    pub is_hidden: bool,
    pub time_in_force: TimeInForce,
    pub priority_timestamp: DateTime<Utc>,
    pub ice_berg_quantity: Option<EnergyAmount>,
    // Direct field access compatibility
    pub id: Uuid,
    pub account_id: AccountId,
    pub order_type: OrderType,
    pub energy_amount: EnergyAmount,
    pub price: TokenPrice,
    pub grid_location: GridLocation,
    pub energy_source: EnergySource,
    pub filled_amount: EnergyAmount,
    pub minimum_fill: Option<EnergyAmount>,
    pub post_only: bool,
}

/// Trading fees breakdown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TradeFees {
    pub maker_fee: TokenPrice,
    pub taker_fee: TokenPrice,
    pub grid_fee: TokenPrice,
    pub regulatory_fee: TokenPrice,
    pub total_fee: TokenPrice,
}

/// Time-in-Force options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeInForce {
    GTC,                        // Good Till Cancelled (same as GoodTillCancelled)
    GoodTillCancelled,          // GTC
    IOC,                        // Immediate or Cancel
    ImmediateOrCancel,          // IOC
    FOK,                        // Fill or Kill
    FillOrKill,                 // FOK
    DAY,                        // Day order
    GTT(DateTime<Utc>),         // Good Till Time
}

/// Order book event for real-time updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderBookEvent {
    OrderAdded(CDAOrder),
    OrderCancelled(Uuid),
    OrderExecuted(TradeExecution),
    OrderExpired(Uuid),
    MarketDepthUpdate(MarketDepth),
}

/// Wrapper for ordered float comparisons
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFloat(pub f64);

impl Eq for OrderedFloat {}

impl std::cmp::Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::cmp::PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<f64> for OrderedFloat {
    fn from(value: f64) -> Self {
        OrderedFloat(value)
    }
}

impl From<OrderedFloat> for f64 {
    fn from(value: OrderedFloat) -> Self {
        value.0
    }
}

/// Order status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Expired,
}

/// Settlement status for trades
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SettlementStatus {
    Pending,
    Settled,
    Failed,
}