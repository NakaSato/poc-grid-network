//! # Comprehensive CDA Engine Unit Tests
//! 
//! Tests for the core Continuous Double Auction engine functionality

use super::*;
use crate::runtime::continuous_double_auction::*;
use crate::runtime::cda::types::*;
use crate::types::*;
use crate::utils::testing::*;
use tokio_test;
use std::time::Duration;
use uuid::Uuid;
use chrono::Utc;

/// Test fixture for CDA engine testing
pub struct CDATestFixture {
    pub engine: ContinuousDoubleAuction,
    pub test_orders: Vec<EnergyOrder>,
    pub test_location: GridLocation,
}

impl CDATestFixture {
    pub async fn new() -> Self {
        let engine = ContinuousDoubleAuction::new().await.expect("Failed to create CDA engine");
        let test_location = create_test_grid_location();
        let test_orders = vec![
            create_test_buy_order(&test_location, 100.0, 50.0),
            create_test_sell_order(&test_location, 80.0, 52.0),
            create_test_buy_order(&test_location, 120.0, 51.0),
            create_test_sell_order(&test_location, 90.0, 49.0),
        ];
        
        Self {
            engine,
            test_orders,
            test_location,
        }
    }
    
    pub async fn start_engine(&self) {
        self.engine.start().await.expect("Failed to start CDA engine");
    }
    
    pub async fn stop_engine(&self) {
        self.engine.stop().await.expect("Failed to stop CDA engine");
    }
}

#[tokio::test]
async fn test_cda_engine_creation() {
    let engine = ContinuousDoubleAuction::new().await;
    assert!(engine.is_ok(), "CDA engine creation should succeed");
}

#[tokio::test]
async fn test_cda_engine_with_custom_config() {
    let engine = ContinuousDoubleAuction::with_config(500, 5000).await;
    assert!(engine.is_ok(), "CDA engine creation with custom config should succeed");
}

#[tokio::test]
async fn test_cda_engine_lifecycle() {
    let fixture = CDATestFixture::new().await;
    
    // Test start
    fixture.start_engine().await;
    
    // Test stop
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_single_order_submission() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    let order = &fixture.test_orders[0];
    let result = fixture.engine.submit_order(order.clone()).await;
    
    assert!(result.is_ok(), "Order submission should succeed");
    let executions = result.unwrap();
    assert!(executions.is_empty(), "Single order should not execute against empty book");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_matching_buy_sell_orders() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Submit sell order first
    let sell_order = create_test_sell_order(&fixture.test_location, 100.0, 50.0);
    let result1 = fixture.engine.submit_order(sell_order).await;
    assert!(result1.is_ok());
    assert!(result1.unwrap().is_empty(), "First order should not match");
    
    // Submit matching buy order
    let buy_order = create_test_buy_order(&fixture.test_location, 100.0, 51.0);
    let result2 = fixture.engine.submit_order(buy_order).await;
    assert!(result2.is_ok());
    
    let executions = result2.unwrap();
    assert!(!executions.is_empty(), "Matching orders should execute");
    assert_eq!(executions.len(), 1, "Should have exactly one execution");
    
    let execution = &executions[0];
    assert_eq!(execution.quantity, 100.0, "Full quantity should be matched");
    assert_eq!(execution.price, 50.0, "Price should match sell order price");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_partial_order_matching() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Submit large sell order
    let sell_order = create_test_sell_order(&fixture.test_location, 200.0, 50.0);
    fixture.engine.submit_order(sell_order).await.unwrap();
    
    // Submit smaller buy order
    let buy_order = create_test_buy_order(&fixture.test_location, 100.0, 51.0);
    let result = fixture.engine.submit_order(buy_order).await.unwrap();
    
    assert_eq!(result.len(), 1, "Should have one execution");
    assert_eq!(result[0].quantity, 100.0, "Should match smaller quantity");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_order_cancellation() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Submit order
    let order = create_test_buy_order(&fixture.test_location, 100.0, 50.0);
    let order_id = order.id;
    fixture.engine.submit_order(order).await.unwrap();
    
    // Cancel order
    let cancel_result = fixture.engine.cancel_order(order_id).await;
    assert!(cancel_result.is_ok(), "Order cancellation should succeed");
    assert!(cancel_result.unwrap(), "Order should be found and cancelled");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_cancel_nonexistent_order() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    let nonexistent_id = Uuid::new_v4();
    let cancel_result = fixture.engine.cancel_order(nonexistent_id).await;
    
    assert!(cancel_result.is_ok(), "Cancellation should not error");
    assert!(!cancel_result.unwrap(), "Nonexistent order should return false");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_invalid_order_validation() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Test zero quantity order
    let mut invalid_order = create_test_buy_order(&fixture.test_location, 0.0, 50.0);
    let result = fixture.engine.submit_order(invalid_order).await;
    assert!(result.is_err(), "Zero quantity order should be rejected");
    
    // Test zero price order
    invalid_order = create_test_buy_order(&fixture.test_location, 100.0, 0.0);
    let result = fixture.engine.submit_order(invalid_order).await;
    assert!(result.is_err(), "Zero price order should be rejected");
    
    // Test empty account ID
    invalid_order = create_test_buy_order(&fixture.test_location, 100.0, 50.0);
    invalid_order.account_id = "".to_string();
    let result = fixture.engine.submit_order(invalid_order).await;
    assert!(result.is_err(), "Empty account ID should be rejected");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_market_depth_calculation() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Add multiple orders at different price levels
    let orders = vec![
        create_test_buy_order(&fixture.test_location, 100.0, 49.0),
        create_test_buy_order(&fixture.test_location, 150.0, 48.0),
        create_test_sell_order(&fixture.test_location, 200.0, 51.0),
        create_test_sell_order(&fixture.test_location, 120.0, 52.0),
    ];
    
    for order in orders {
        fixture.engine.submit_order(order).await.unwrap();
    }
    
    let depth = fixture.engine.get_market_depth(5).await.unwrap();
    
    assert!(!depth.bids.is_empty(), "Should have bid levels");
    assert!(!depth.asks.is_empty(), "Should have ask levels");
    assert!(depth.spread > 0.0, "Spread should be positive");
    assert!(depth.mid_price > 0.0, "Mid price should be positive");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_recent_trades_retrieval() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Execute some trades
    fixture.engine.submit_order(create_test_sell_order(&fixture.test_location, 100.0, 50.0)).await.unwrap();
    fixture.engine.submit_order(create_test_buy_order(&fixture.test_location, 100.0, 51.0)).await.unwrap();
    
    let trades = fixture.engine.get_recent_trades(Some(10)).await.unwrap();
    assert!(!trades.is_empty(), "Should have trade history");
    assert_eq!(trades.len(), 1, "Should have exactly one trade");
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_event_subscription() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    let mut event_receiver = fixture.engine.subscribe_to_events();
    
    // Submit order in background task
    let engine_clone = fixture.engine.clone();
    let order = create_test_buy_order(&fixture.test_location, 100.0, 50.0);
    
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        engine_clone.submit_order(order).await.unwrap();
    });
    
    // Wait for event
    let event = tokio::time::timeout(Duration::from_secs(1), event_receiver.recv()).await;
    assert!(event.is_ok(), "Should receive event within timeout");
    
    let received_event = event.unwrap().unwrap();
    match received_event {
        OrderBookEvent::OrderAdded(_) => {
            // Expected event type
        },
        _ => panic!("Unexpected event type"),
    }
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_concurrent_order_submission() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    let engine = fixture.engine.clone();
    let location = fixture.test_location.clone();
    
    // Submit multiple orders concurrently
    let mut handles = vec![];
    
    for i in 0..10 {
        let engine_clone = engine.clone();
        let location_clone = location.clone();
        
        let handle = tokio::spawn(async move {
            let order = create_test_buy_order(&location_clone, 100.0, 50.0 + i as f64);
            engine_clone.submit_order(order).await
        });
        
        handles.push(handle);
    }
    
    // Wait for all orders to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok(), "Concurrent order submission should succeed");
    }
    
    fixture.stop_engine().await;
}

#[tokio::test]
async fn test_price_time_priority() {
    let fixture = CDATestFixture::new().await;
    fixture.start_engine().await;
    
    // Submit two buy orders at same price with slight time difference
    let order1 = create_test_buy_order(&fixture.test_location, 100.0, 50.0);
    let order1_id = order1.id;
    fixture.engine.submit_order(order1).await.unwrap();
    
    tokio::time::sleep(Duration::from_millis(10)).await;
    
    let order2 = create_test_buy_order(&fixture.test_location, 100.0, 50.0);
    fixture.engine.submit_order(order2).await.unwrap();
    
    // Submit sell order that should match first order due to time priority
    let sell_order = create_test_sell_order(&fixture.test_location, 50.0, 50.0);
    let executions = fixture.engine.submit_order(sell_order).await.unwrap();
    
    assert_eq!(executions.len(), 1, "Should have one execution");
    assert_eq!(executions[0].buy_order_id, order1_id, "First order should be matched");
    
    fixture.stop_engine().await;
}

/// Helper functions for test data creation
fn create_test_buy_order(location: &GridLocation, amount: f64, price: f64) -> EnergyOrder {
    EnergyOrder {
        id: Uuid::new_v4(),
        account_id: create_test_account_id(),
        order_type: OrderType::Buy,
        energy_amount: amount,
        price_per_unit: price as u64,
        location: location.clone(),
        energy_source: Some(EnergySource::Solar),
        status: OrderStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        expires_at: None,
    }
}

fn create_test_sell_order(location: &GridLocation, amount: f64, price: f64) -> EnergyOrder {
    EnergyOrder {
        id: Uuid::new_v4(),
        account_id: create_test_account_id(),
        order_type: OrderType::Sell,
        energy_amount: amount,
        price_per_unit: price as u64,
        location: location.clone(),
        energy_source: Some(EnergySource::Solar),
        status: OrderStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        expires_at: None,
    }
}
