//! # Energy Trading Example
//! 
//! This example demonstrates how to use the trading service to place
//! and manage energy orders.

use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem, 
    SystemConfig,
    types::{EnergyOrder, OrderType, EnergySource, GridLocation, OrderStatus},
};
use uuid::Uuid;
use chrono::Utc;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    info!("⚡ Energy Trading Example");
    
    // Start the system
    let system = ThaiEnergyTradingSystem::new(SystemConfig::default()).await?;
    system.start().await?;
    
    let trading_service = system.trading_service();
    
    // Create a location in Bangkok
    let location = GridLocation {
        province: "Bangkok".to_string(),
        district: "Pathum Wan".to_string(),
        coordinates: (13.7563, 100.5018),
        region: "Central".to_string(),
        substation: "Siam".to_string(),
        grid_code: "BKK001".to_string(),
        meter_id: "M001".to_string(),
    };
    
    // Create a buy order for solar energy
    let buy_order = EnergyOrder {
        id: Uuid::new_v4(),
        account_id: "buyer_account_001".to_string(),
        order_type: OrderType::Buy,
        energy_source: Some(EnergySource::Solar),
        energy_amount: 100.0, // 100 kWh
        price_per_unit: 3500000000000000000000u128, // 3.5 tokens per kWh
        location: location.clone(),
        status: OrderStatus::Pending,
        timestamp: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Place the buy order
    info!("📋 Placing buy order for {} kWh of solar energy at {} tokens/kWh", 
          buy_order.energy_amount, buy_order.price_per_unit);
    
    let buy_order_id = trading_service.place_order(buy_order).await?;
    info!("✅ Buy order placed with ID: {}", buy_order_id);
    
    // Create a sell order for solar energy
    let sell_order = EnergyOrder {
        id: Uuid::new_v4(),
        account_id: "seller_account_001".to_string(),
        order_type: OrderType::Sell,
        energy_source: Some(EnergySource::Solar),
        energy_amount: 50.0, // 50 kWh
        price_per_unit: 3200000000000000000000u128, // 3.2 tokens per kWh
        location: location.clone(),
        status: OrderStatus::Pending,
        timestamp: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Place the sell order
    info!("📋 Placing sell order for {} kWh of solar energy at {} tokens/kWh", 
          sell_order.energy_amount, sell_order.price_per_unit);
    
    let sell_order_id = trading_service.place_order(sell_order).await?;
    info!("✅ Sell order placed with ID: {}", sell_order_id);
    
    // Get order book
    let (buy_orders, sell_orders) = trading_service.get_order_book(
        &location,
        Some(EnergySource::Solar)
    ).await?;
    
    info!("📊 Order Book - Buy Orders: {}, Sell Orders: {}", 
          buy_orders.len(), sell_orders.len());
    
    // Get market data
    let market_data = trading_service.get_market_data(&location).await?;
    info!("📈 Market Data: {:?}", market_data);
    
    // Demonstrate order cancellation
    info!("❌ Cancelling buy order");
    trading_service.cancel_order(buy_order_id, "buyer_account_001".to_string()).await?;
    info!("✅ Buy order cancelled");
    
    // Stop the system
    system.stop().await?;
    
    info!("✅ Energy trading example completed");
    Ok(())
}
