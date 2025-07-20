//! # CDA Fee Calculation
//! 
//! Fee calculation utilities for the Continuous Double Auction system.

use super::types::*;
use crate::types::*;
use crate::utils::SystemResult;

/// Fee calculator for CDA trades
pub struct FeeCalculator {
    maker_fee_rate: f64,
    taker_fee_rate: f64,
    grid_fee_rate: f64,
    regulatory_fee_rate: f64,
}

impl FeeCalculator {
    pub fn new() -> Self {
        Self {
            maker_fee_rate: 0.001,  // 0.1%
            taker_fee_rate: 0.002,  // 0.2%
            grid_fee_rate: 0.005,   // 0.5%
            regulatory_fee_rate: 0.0005, // 0.05%
        }
    }
    
    /// Calculate fees for a trade
    pub fn calculate_fees(&self, price: TokenPrice, quantity: EnergyAmount, is_taker: bool) -> SystemResult<TradeFees> {
        let trade_value = price * quantity;
        
        let maker_fee = if is_taker { 0.0 } else { trade_value * self.maker_fee_rate };
        let taker_fee = if is_taker { trade_value * self.taker_fee_rate } else { 0.0 };
        let grid_fee = trade_value * self.grid_fee_rate;
        let regulatory_fee = trade_value * self.regulatory_fee_rate;
        let total_fee = maker_fee + taker_fee + grid_fee + regulatory_fee;
        
        Ok(TradeFees {
            maker_fee,
            taker_fee,
            grid_fee,
            regulatory_fee,
            total_fee,
        })
    }
    
    /// Update fee rates (for dynamic fee adjustment)
    pub fn update_rates(&mut self, maker: f64, taker: f64, grid: f64, regulatory: f64) {
        self.maker_fee_rate = maker;
        self.taker_fee_rate = taker;
        self.grid_fee_rate = grid;
        self.regulatory_fee_rate = regulatory;
    }
}

impl Default for FeeCalculator {
    fn default() -> Self {
        Self::new()
    }
}
