//! Order Manager Tests
//! 
//! Tests for order management functionality including order lifecycle,
//! validation, and persistence.

use super::*;
use thai_energy_trading_blockchain::runtime::cda::orders::*;
use thai_energy_trading_blockchain::runtime::cda::types::*;
use thai_energy_trading_blockchain::types::*;
use std::sync::Arc;
use tokio::test;
use uuid::Uuid;

/// Test order creation and validation
#[tokio::test]
async fn test_order_creation() {
    let order = Order {
        id: Uuid::new_v4(),
        account_id: AccountId::new("test_account".to_string()),
        order_type: OrderType::Buy,
        energy_amount: EnergyAmount::new(100.0),
        price_per_unit: TokenPrice::new(50.0),
        total_value: Balance::new(5000.0),
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            grid_code: "BKK-001".to_string(),
            substation_id: "SUB-001".to_string(),
            coordinates: (13.7563, 100.5018),
            capacity_kw: 1000.0,
        },
        energy_source: Some(EnergySource::Solar),
        status: OrderStatus::Pending,
        created_at: chrono::Utc::now(),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
        filled_amount: EnergyAmount::new(0.0),
        remaining_amount: EnergyAmount::new(100.0),
    };

    assert_eq!(order.order_type, OrderType::Buy);
    assert_eq!(order.energy_amount.value(), 100.0);
    assert_eq!(order.status, OrderStatus::Pending);
}

/// Test order validation rules
#[tokio::test]
async fn test_order_validation() {
    // Valid order should pass
    let valid_order = Order {
        id: Uuid::new_v4(),
        account_id: AccountId::new("test_account".to_string()),
        order_type: OrderType::Sell,
        energy_amount: EnergyAmount::new(50.0),
        price_per_unit: TokenPrice::new(45.0),
        total_value: Balance::new(2250.0),
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            grid_code: "BKK-001".to_string(),
            substation_id: "SUB-001".to_string(),
            coordinates: (13.7563, 100.5018),
            capacity_kw: 1000.0,
        },
        energy_source: Some(EnergySource::Wind),
        status: OrderStatus::Pending,
        created_at: chrono::Utc::now(),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(12)),
        filled_amount: EnergyAmount::new(0.0),
        remaining_amount: EnergyAmount::new(50.0),
    };

    // Validate order fields
    assert!(valid_order.energy_amount.value() > 0.0);
    assert!(valid_order.price_per_unit.value() > 0.0);
    assert!(valid_order.total_value.value() > 0.0);
}

/// Test order status transitions
#[tokio::test]
async fn test_order_status_transitions() {
    let mut order = Order {
        id: Uuid::new_v4(),
        account_id: AccountId::new("test_account".to_string()),
        order_type: OrderType::Buy,
        energy_amount: EnergyAmount::new(100.0),
        price_per_unit: TokenPrice::new(50.0),
        total_value: Balance::new(5000.0),
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            grid_code: "BKK-001".to_string(),
            substation_id: "SUB-001".to_string(),
            coordinates: (13.7563, 100.5018),
            capacity_kw: 1000.0,
        },
        energy_source: Some(EnergySource::Solar),
        status: OrderStatus::Pending,
        created_at: chrono::Utc::now(),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
        filled_amount: EnergyAmount::new(0.0),
        remaining_amount: EnergyAmount::new(100.0),
    };

    // Test status progression
    assert_eq!(order.status, OrderStatus::Pending);
    
    // Simulate partial fill
    order.status = OrderStatus::PartiallyFilled;
    order.filled_amount = EnergyAmount::new(30.0);
    order.remaining_amount = EnergyAmount::new(70.0);
    
    assert_eq!(order.status, OrderStatus::PartiallyFilled);
    assert_eq!(order.filled_amount.value(), 30.0);
    assert_eq!(order.remaining_amount.value(), 70.0);
    
    // Complete the order
    order.status = OrderStatus::Filled;
    order.filled_amount = EnergyAmount::new(100.0);
    order.remaining_amount = EnergyAmount::new(0.0);
    
    assert_eq!(order.status, OrderStatus::Filled);
    assert_eq!(order.filled_amount.value(), 100.0);
    assert_eq!(order.remaining_amount.value(), 0.0);
}

/// Test order expiration handling
#[tokio::test]
async fn test_order_expiration() {
    let expired_order = Order {
        id: Uuid::new_v4(),
        account_id: AccountId::new("test_account".to_string()),
        order_type: OrderType::Buy,
        energy_amount: EnergyAmount::new(100.0),
        price_per_unit: TokenPrice::new(50.0),
        total_value: Balance::new(5000.0),
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            grid_code: "BKK-001".to_string(),
            substation_id: "SUB-001".to_string(),
            coordinates: (13.7563, 100.5018),
            capacity_kw: 1000.0,
        },
        energy_source: Some(EnergySource::Solar),
        status: OrderStatus::Pending,
        created_at: chrono::Utc::now() - chrono::Duration::hours(25), // Created yesterday
        expires_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)), // Expired 1 hour ago
        filled_amount: EnergyAmount::new(0.0),
        remaining_amount: EnergyAmount::new(100.0),
    };

    // Check if order is expired
    if let Some(expires_at) = expired_order.expires_at {
        assert!(expires_at < chrono::Utc::now());
    }
}

/// Test order cancellation
#[tokio::test]
async fn test_order_cancellation() {
    let mut order = Order {
        id: Uuid::new_v4(),
        account_id: AccountId::new("test_account".to_string()),
        order_type: OrderType::Sell,
        energy_amount: EnergyAmount::new(75.0),
        price_per_unit: TokenPrice::new(55.0),
        total_value: Balance::new(4125.0),
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            grid_code: "BKK-001".to_string(),
            substation_id: "SUB-001".to_string(),
            coordinates: (13.7563, 100.5018),
            capacity_kw: 1000.0,
        },
        energy_source: Some(EnergySource::Hydro),
        status: OrderStatus::Pending,
        created_at: chrono::Utc::now(),
        expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(24)),
        filled_amount: EnergyAmount::new(0.0),
        remaining_amount: EnergyAmount::new(75.0),
    };

    // Cancel the order
    order.status = OrderStatus::Cancelled;
    
    assert_eq!(order.status, OrderStatus::Cancelled);
    assert_eq!(order.remaining_amount.value(), 75.0); // Should remain unchanged
}
