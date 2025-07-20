//! # Continuous Double Auction Usage Examples
//! 
//! This example demonstrates how to use the Continuous Double Auction (CDA) system
//! for energy trading, including order placement, market data retrieval, and 
//! real-time event handling.

use thai_energy_trading_blockchain::application::enhanced_trading::EnhancedTradingService;
use thai_energy_trading_blockchain::runtime::continuous_double_auction::{
    ContinuousDoubleAuction, OrderBookEvent, TimeInForce
};
use thai_energy_trading_blockchain::types::*;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏭 Thai Energy Trading Blockchain - Continuous Double Auction Demo");
    println!("================================================================");
    
    // Initialize the enhanced trading service
    let trading_service = Arc::new(EnhancedTradingService::new_placeholder().await?);
    trading_service.start().await?;
    
    println!("✅ Enhanced Trading Service started");
    
    // Subscribe to real-time events
    let mut event_receiver = trading_service.subscribe_to_market_events();
    
    // Spawn task to handle events
    let event_handler = tokio::spawn(async move {
        while let Ok(event) = event_receiver.recv().await {
            match event {
                OrderBookEvent::OrderAdded(order) => {
                    println!("📝 New order added: {} {} kWh at {} THB/kWh", 
                             order.base.id, 
                             order.remaining_quantity, 
                             order.base.price_per_unit);
                }
                OrderBookEvent::OrderExecuted(execution) => {
                    println!("⚡ Trade executed: {} kWh at {} THB/kWh (Total: {} THB)", 
                             execution.quantity, 
                             execution.price, 
                             execution.price * execution.quantity);
                }
                OrderBookEvent::OrderCancelled(order_id) => {
                    println!("❌ Order cancelled: {}", order_id);
                }
                OrderBookEvent::PriceUpdate(bid, ask) => {
                    println!("💰 Price update: Bid {} THB/kWh, Ask {} THB/kWh, Spread {} THB", 
                             bid, ask, ask - bid);
                }
                _ => {}
            }
        }
    });
    
    // Demo scenario: Multiple users trading solar energy
    demo_scenario_1(&trading_service).await?;
    
    // Wait a bit for events to process
    sleep(Duration::from_secs(2)).await;
    
    // Demo scenario: Market making and liquidity provision
    demo_scenario_2(&trading_service).await?;
    
    // Wait a bit for events to process
    sleep(Duration::from_secs(2)).await;
    
    // Demo scenario: Market data and analytics
    demo_scenario_3(&trading_service).await?;
    
    // Clean shutdown
    event_handler.abort();
    trading_service.stop().await?;
    
    println!("\n🎯 Demo completed successfully!");
    Ok(())
}

/// Demo Scenario 1: Basic Trading Operations
async fn demo_scenario_1(trading_service: &Arc<EnhancedTradingService>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔋 Scenario 1: Basic Solar Energy Trading");
    println!("==========================================");
    
    // Create sample grid location (Bangkok, Thailand)
    let bangkok_location = GridLocation {
        province: "Bangkok".to_string(),
        district: "Chatuchak".to_string(),
        coordinates: (13.7563, 100.5018),
        region: "Central".to_string(),
        substation: "BKK-SUB-001".to_string(),
        grid_code: "BKK001".to_string(),
        meter_id: "MTR-BKK-001".to_string(),
    };
    
    // Scenario: Solar farm wants to sell excess energy
    let sell_order = EnergyOrder {
        id: Uuid::new_v4(),
        order_type: OrderType::Sell,
        energy_amount: 1000.0, // 1000 kWh
        price_per_unit: 4800, // 4.8 THB per kWh
        location: bangkok_location.clone(),
        energy_source: Some(EnergySource::Solar),
        timestamp: chrono::Utc::now(),
        status: OrderStatus::Pending,
        account_id: "solar_farm_001".to_string(),
        updated_at: chrono::Utc::now(),
    };
    
    println!("📤 Solar farm placing sell order: {} kWh at {} THB/kWh", 
             sell_order.energy_amount, sell_order.price_per_unit);
    
    let result = trading_service.place_order(sell_order).await?;
    println!("   Order ID: {}", result.order_id);
    println!("   Status: {:?}", result.status);
    
    // Scenario: Factory needs energy for production
    let buy_order = EnergyOrder {
        id: Uuid::new_v4(),
        order_type: OrderType::Buy,
        energy_amount: 500.0, // 500 kWh
        price_per_unit: 5000, // 5.0 THB per kWh (willing to pay premium)
        location: bangkok_location.clone(),
        energy_source: Some(EnergySource::Solar), // Prefer solar
        timestamp: chrono::Utc::now(),
        status: OrderStatus::Pending,
        account_id: "factory_001".to_string(),
        updated_at: chrono::Utc::now(),
    };
    
    println!("📥 Factory placing buy order: {} kWh at {} THB/kWh", 
             buy_order.energy_amount, buy_order.price_per_unit);
    
    let result = trading_service.place_order(buy_order).await?;
    println!("   Order ID: {}", result.order_id);
    println!("   Status: {:?}", result.status);
    println!("   Executions: {} trades", result.executions.len());
    
    for (i, trade) in result.executions.iter().enumerate() {
        println!("   Trade {}: {} kWh at {} THB/kWh", 
                 i + 1, trade.energy_amount, trade.price_per_kwh as f64);
    }
    
    Ok(())
}

/// Demo Scenario 2: Advanced Order Types and Market Making
async fn demo_scenario_2(trading_service: &Arc<EnhancedTradingService>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🏪 Scenario 2: Market Making and Liquidity Provision");
    println!("====================================================");
    
    let chiang_mai_location = GridLocation {
        province: "Chiang Mai".to_string(),
        district: "Mueang".to_string(),
        coordinates: (18.7883, 98.9853),
        region: "Northern".to_string(),
        substation: "CNX-SUB-001".to_string(),
        grid_code: "CNX001".to_string(),
        meter_id: "MTR-CNX-001".to_string(),
    };
    
    // Market maker provides liquidity on both sides
    println!("🏦 Market maker providing liquidity...");
    
    // Bid side - willing to buy wind energy
    let bid_order = EnergyOrder {
        id: Uuid::new_v4(),
        order_type: OrderType::Buy,
        energy_amount: 2000.0,
        price_per_unit: 4500, // Lower bid
        location: chiang_mai_location.clone(),
        energy_source: Some(EnergySource::Wind),
        timestamp: chrono::Utc::now(),
        status: OrderStatus::Pending,
        account_id: "market_maker_001".to_string(),
        updated_at: chrono::Utc::now(),
    };
    
    let result = trading_service.place_order(bid_order).await?;
    println!("   Bid order placed: {} at {}", result.order_id, 4500);
    
    // Ask side - willing to sell wind energy
    let ask_order = EnergyOrder {
        id: Uuid::new_v4(),
        order_type: OrderType::Sell,
        energy_amount: 2000.0,
        price_per_unit: 5200, // Higher ask
        location: chiang_mai_location.clone(),
        energy_source: Some(EnergySource::Wind),
        timestamp: chrono::Utc::now(),
        status: OrderStatus::Pending,
        account_id: "market_maker_001".to_string(),
        updated_at: chrono::Utc::now(),
    };
    
    let result = trading_service.place_order(ask_order).await?;
    println!("   Ask order placed: {} at {}", result.order_id, 5200);
    println!("   Market spread: {} THB/kWh", 5200 - 4500);
    
    // Aggressive buyer comes in
    let aggressive_buy = EnergyOrder {
        id: Uuid::new_v4(),
        order_type: OrderType::Buy,
        energy_amount: 800.0,
        price_per_unit: 5300, // Above market ask
        location: chiang_mai_location.clone(),
        energy_source: Some(EnergySource::Wind),
        timestamp: chrono::Utc::now(),
        status: OrderStatus::Pending,
        account_id: "urgent_buyer_001".to_string(),
        updated_at: chrono::Utc::now(),
    };
    
    println!("⚡ Urgent buyer placing aggressive order: {} kWh at {} THB/kWh", 
             aggressive_buy.energy_amount, aggressive_buy.price_per_unit);
    
    let result = trading_service.place_order(aggressive_buy).await?;
    println!("   Executions: {} trades", result.executions.len());
    
    Ok(())
}

/// Demo Scenario 3: Market Data and Analytics
async fn demo_scenario_3(trading_service: &Arc<EnhancedTradingService>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Scenario 3: Market Data and Analytics");
    println!("========================================");
    
    let phuket_location = GridLocation {
        province: "Phuket".to_string(),
        district: "Mueang".to_string(),
        coordinates: (7.8804, 98.3923),
        region: "Southern".to_string(),
        substation: "HKT-SUB-001".to_string(),
        grid_code: "HKT001".to_string(),
        meter_id: "MTR-HKT-001".to_string(),
    };
    
    // Add some orders to create market depth
    let orders = vec![
        // Buy orders (bids)
        (OrderType::Buy, 100.0, 4900, "hotel_001"),
        (OrderType::Buy, 200.0, 4850, "resort_001"),
        (OrderType::Buy, 150.0, 4800, "mall_001"),
        
        // Sell orders (asks)
        (OrderType::Sell, 120.0, 5100, "solar_phuket_001"),
        (OrderType::Sell, 180.0, 5150, "solar_phuket_002"),
        (OrderType::Sell, 250.0, 5200, "solar_phuket_003"),
    ];
    
    println!("📝 Creating sample market depth...");
    for (order_type, amount, price, account) in orders {
        let order = EnergyOrder {
            id: Uuid::new_v4(),
            order_type,
            energy_amount: amount,
            price_per_unit: price,
            location: phuket_location.clone(),
            energy_source: Some(EnergySource::Solar),
            timestamp: chrono::Utc::now(),
            status: OrderStatus::Pending,
            account_id: account.to_string(),
            updated_at: chrono::Utc::now(),
        };
        
        trading_service.place_order(order).await?;
    }
    
    // Get market depth
    println!("\n📈 Market Depth Analysis:");
    let depth = trading_service.get_market_depth(&phuket_location, 5).await?;
    
    println!("   Best Bid: {} THB/kWh", 
             depth.bids.first().map(|b| b.price).unwrap_or(0.0));
    println!("   Best Ask: {} THB/kWh", 
             depth.asks.first().map(|a| a.price).unwrap_or(0.0));
    println!("   Spread: {} THB/kWh", depth.spread);
    println!("   Mid Price: {} THB/kWh", depth.mid_price);
    println!("   Total Bid Volume: {} kWh", depth.total_bid_volume);
    println!("   Total Ask Volume: {} kWh", depth.total_ask_volume);
    
    println!("\n   📊 Order Book:");
    println!("   Bids:");
    for (i, bid) in depth.bids.iter().enumerate() {
        println!("   {}. {} kWh @ {} THB/kWh ({} orders)", 
                 i + 1, bid.total_quantity, bid.price, bid.order_count);
    }
    
    println!("   Asks:");
    for (i, ask) in depth.asks.iter().enumerate() {
        println!("   {}. {} kWh @ {} THB/kWh ({} orders)", 
                 i + 1, ask.total_quantity, ask.price, ask.order_count);
    }
    
    // Get market data
    println!("\n💹 Market Statistics:");
    let market_data = trading_service.get_market_data(&phuket_location).await?;
    
    println!("   Current Price: {} THB/kWh", market_data.current_price);
    println!("   24h Volume: {} kWh", market_data.volume_24h);
    println!("   24h High: {} THB/kWh", market_data.high_24h);
    println!("   24h Low: {} THB/kWh", market_data.low_24h);
    println!("   24h Change: {:.2}%", market_data.price_change_24h * 100.0);
    println!("   Trend: {:?}", market_data.price_trend);
    println!("   Trades: {}", market_data.trades_24h);
    
    // Get user trading history
    println!("\n📜 Sample User Trading History:");
    let user_trades = trading_service.get_user_trades(&"hotel_001".to_string()).await?;
    
    if user_trades.is_empty() {
        println!("   No trades found for user");
    } else {
        for (i, trade) in user_trades.iter().take(5).enumerate() {
            println!("   {}. {} kWh at {} THB/kWh ({:?})", 
                     i + 1, 
                     trade.energy_amount, 
                     trade.price_per_kwh as f64, 
                     trade.status);
        }
    }
    
    Ok(())
}

/// Demo helper: Create sample grid location
fn create_grid_location(province: &str, district: &str, lat: f64, lng: f64) -> GridLocation {
    GridLocation {
        province: province.to_string(),
        district: district.to_string(),
        coordinates: (lat, lng),
        region: match province {
            "Bangkok" | "Nonthaburi" | "Pathum Thani" => "Central",
            "Chiang Mai" | "Chiang Rai" => "Northern",
            "Phuket" | "Surat Thani" => "Southern",
            _ => "Other",
        }.to_string(),
        substation: format!("{}-SUB-001", province.chars().take(3).collect::<String>().to_uppercase()),
        grid_code: format!("{}001", province.chars().take(3).collect::<String>().to_uppercase()),
        meter_id: format!("MTR-{}-001", province.chars().take(3).collect::<String>().to_uppercase()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_cda_basic_functionality() {
        let trading_service = Arc::new(EnhancedTradingService::new_placeholder().await.unwrap());
        trading_service.start().await.unwrap();
        
        let location = create_grid_location("Bangkok", "Chatuchak", 13.7563, 100.5018);
        
        // Test order placement
        let order = EnergyOrder {
            id: Uuid::new_v4(),
            order_type: OrderType::Buy,
            energy_amount: 100.0,
            price_per_unit: 5000,
            location: location.clone(),
            energy_source: Some(EnergySource::Solar),
            timestamp: chrono::Utc::now(),
            status: OrderStatus::Pending,
            account_id: "test_user".to_string(),
            updated_at: chrono::Utc::now(),
        };
        
        let result = trading_service.place_order(order).await;
        assert!(result.is_ok());
        
        // Test market data
        let market_data = trading_service.get_market_data(&location).await;
        assert!(market_data.is_ok());
        
        trading_service.stop().await.unwrap();
    }
    
    #[tokio::test]
    async fn test_market_depth() {
        let trading_service = Arc::new(EnhancedTradingService::new_placeholder().await.unwrap());
        trading_service.start().await.unwrap();
        
        let location = create_grid_location("Bangkok", "Chatuchak", 13.7563, 100.5018);
        
        let depth = trading_service.get_market_depth(&location, 5).await;
        assert!(depth.is_ok());
        
        let depth = depth.unwrap();
        assert!(depth.spread >= 0.0);
        
        trading_service.stop().await.unwrap();
    }
}
