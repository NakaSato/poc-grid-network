//! # Test Fixtures and Data Generators
//! 
//! Common test data and fixtures for consistent testing

use crate::types::*;
use crate::utils::testing::*;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

/// Comprehensive test data generator
pub struct TestDataGenerator;

impl TestDataGenerator {
    /// Generate realistic energy orders with varied parameters
    pub fn generate_energy_orders(count: usize) -> Vec<EnergyOrder> {
        let mut orders = Vec::with_capacity(count);
        let locations = Self::generate_grid_locations(5);
        let energy_sources = vec![
            EnergySource::Solar,
            EnergySource::Wind,
            EnergySource::Hydro,
            EnergySource::Biomass,
        ];
        
        for i in 0..count {
            let location = &locations[i % locations.len()];
            let energy_source = &energy_sources[i % energy_sources.len()];
            let is_buy = i % 2 == 0;
            
            let base_price = 50.0;
            let price_variation = (i as f64 * 0.1) - 5.0; // ±5 variation
            let price = base_price + price_variation;
            
            let base_amount = 100.0;
            let amount_variation = (i as f64 * 10.0) % 200.0; // 0-200 variation
            let amount = base_amount + amount_variation;
            
            orders.push(EnergyOrder {
                id: Uuid::new_v4(),
                account_id: format!("test_account_{}", i % 10), // 10 different accounts
                order_type: if is_buy { OrderType::Buy } else { OrderType::Sell },
                energy_amount: amount,
                price_per_unit: price as u64,
                location: location.clone(),
                energy_source: Some(energy_source.clone()),
                status: OrderStatus::Pending,
                created_at: Utc::now() - chrono::Duration::minutes(i as i64),
                updated_at: Utc::now() - chrono::Duration::minutes(i as i64),
                expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
            });
        }
        
        orders
    }
    
    /// Generate diverse grid locations
    pub fn generate_grid_locations(count: usize) -> Vec<GridLocation> {
        let provinces = vec!["Bangkok", "Chiang Mai", "Phuket", "Khon Kaen", "Songkhla"];
        let districts = vec!["District 1", "District 2", "District 3", "District 4"];
        
        let mut locations = Vec::with_capacity(count);
        
        for i in 0..count {
            locations.push(GridLocation {
                province: provinces[i % provinces.len()].to_string(),
                district: districts[i % districts.len()].to_string(),
                substation: format!("Substation {}", i + 1),
                coordinates: GridCoordinates {
                    latitude: 13.7563 + (i as f64 * 0.1), // Bangkok base + variation
                    longitude: 100.5018 + (i as f64 * 0.1),
                },
                grid_code: format!("GRID{:03}", i + 1),
                capacity_mw: 100.0 + (i as f64 * 10.0),
                current_load_mw: 50.0 + (i as f64 * 5.0),
            });
        }
        
        locations
    }
    
    /// Generate realistic market scenarios
    pub fn generate_market_scenario(scenario_type: MarketScenarioType) -> MarketScenario {
        match scenario_type {
            MarketScenarioType::HighVolume => {
                MarketScenario {
                    name: "High Volume Trading".to_string(),
                    orders: Self::generate_energy_orders(1000),
                    expected_matches: 400,
                    market_conditions: MarketConditions::HighVolume,
                }
            },
            MarketScenarioType::PriceVolatility => {
                let mut orders = Self::generate_energy_orders(200);
                // Add price volatility
                for (i, order) in orders.iter_mut().enumerate() {
                    let volatility = if i < 100 { 0.8 } else { 1.2 }; // 20% price swings
                    order.price_per_unit = (order.price_per_unit as f64 * volatility) as u64;
                }
                MarketScenario {
                    name: "Price Volatility".to_string(),
                    orders,
                    expected_matches: 80,
                    market_conditions: MarketConditions::Volatile,
                }
            },
            MarketScenarioType::LowLiquidity => {
                MarketScenario {
                    name: "Low Liquidity".to_string(),
                    orders: Self::generate_energy_orders(50),
                    expected_matches: 10,
                    market_conditions: MarketConditions::LowLiquidity,
                }
            },
            MarketScenarioType::CrossLocation => {
                let locations = Self::generate_grid_locations(10);
                let mut orders = Vec::new();
                
                // Generate orders across multiple locations
                for location in locations {
                    let mut location_orders = Self::generate_energy_orders(50);
                    for order in &mut location_orders {
                        order.location = location.clone();
                    }
                    orders.extend(location_orders);
                }
                
                MarketScenario {
                    name: "Cross-Location Trading".to_string(),
                    orders,
                    expected_matches: 200,
                    market_conditions: MarketConditions::Normal,
                }
            },
        }
    }
    
    /// Generate stress test data
    pub fn generate_stress_test_data(order_count: usize) -> StressTestData {
        StressTestData {
            orders: Self::generate_energy_orders(order_count),
            concurrent_users: 100,
            operations_per_second: 1000,
            duration_seconds: 300, // 5 minutes
            expected_success_rate: 0.99,
        }
    }
}

/// Market scenario types for testing
#[derive(Debug, Clone)]
pub enum MarketScenarioType {
    HighVolume,
    PriceVolatility,
    LowLiquidity,
    CrossLocation,
}

/// Market scenario data structure
#[derive(Debug, Clone)]
pub struct MarketScenario {
    pub name: String,
    pub orders: Vec<EnergyOrder>,
    pub expected_matches: usize,
    pub market_conditions: MarketConditions,
}

/// Market conditions for testing
#[derive(Debug, Clone)]
pub enum MarketConditions {
    Normal,
    HighVolume,
    Volatile,
    LowLiquidity,
}

/// Stress test data structure
#[derive(Debug, Clone)]
pub struct StressTestData {
    pub orders: Vec<EnergyOrder>,
    pub concurrent_users: usize,
    pub operations_per_second: usize,
    pub duration_seconds: u64,
    pub expected_success_rate: f64,
}

/// Database fixtures for testing
pub struct DatabaseFixtures;

impl DatabaseFixtures {
    /// Create clean test database state
    pub fn clean_state() -> DatabaseState {
        DatabaseState {
            orders: HashMap::new(),
            trades: HashMap::new(),
            accounts: HashMap::new(),
            locations: HashMap::new(),
        }
    }
    
    /// Create populated test database state
    pub fn populated_state() -> DatabaseState {
        let mut state = Self::clean_state();
        
        // Add test accounts
        for i in 0..10 {
            let account_id = format!("test_account_{}", i);
            state.accounts.insert(account_id.clone(), TestAccount {
                id: account_id,
                balance: 10000.0,
                energy_capacity: 1000.0,
            });
        }
        
        // Add test locations
        let locations = TestDataGenerator::generate_grid_locations(5);
        for (i, location) in locations.into_iter().enumerate() {
            state.locations.insert(format!("location_{}", i), location);
        }
        
        // Add test orders
        let orders = TestDataGenerator::generate_energy_orders(20);
        for order in orders {
            state.orders.insert(order.id, order);
        }
        
        state
    }
}

/// Database state for testing
#[derive(Debug, Clone)]
pub struct DatabaseState {
    pub orders: HashMap<Uuid, EnergyOrder>,
    pub trades: HashMap<Uuid, TradeRecord>,
    pub accounts: HashMap<String, TestAccount>,
    pub locations: HashMap<String, GridLocation>,
}

/// Test account structure
#[derive(Debug, Clone)]
pub struct TestAccount {
    pub id: String,
    pub balance: f64,
    pub energy_capacity: f64,
}

/// Trade record for testing
#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub id: Uuid,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
    pub quantity: f64,
    pub price: f64,
    pub timestamp: chrono::DateTime<Utc>,
}

/// Property-based test generators
pub mod property_generators {
    use super::*;
    use proptest::prelude::*;
    
    /// Generate arbitrary energy orders for property testing
    pub fn arb_energy_order() -> impl Strategy<Value = EnergyOrder> {
        (
            any::<u64>().prop_map(|_| Uuid::new_v4()),
            "[a-zA-Z0-9_]{5,20}",
            prop::bool::ANY,
            1.0..10000.0f64,
            1u64..1000u64,
            arb_grid_location(),
            prop::option::of(arb_energy_source()),
        ).prop_map(|(id, account_id, is_buy, amount, price, location, energy_source)| {
            EnergyOrder {
                id,
                account_id,
                order_type: if is_buy { OrderType::Buy } else { OrderType::Sell },
                energy_amount: amount,
                price_per_unit: price,
                location,
                energy_source,
                status: OrderStatus::Pending,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                expires_at: None,
            }
        })
    }
    
    /// Generate arbitrary grid locations
    pub fn arb_grid_location() -> impl Strategy<Value = GridLocation> {
        (
            "[A-Z][a-z]{4,15}",
            "[A-Z][a-z]{4,15}",
            "[A-Z][a-z]{4,15}",
            "[A-Z]{4}[0-9]{3}",
            -90.0..90.0f64,
            -180.0..180.0f64,
            10.0..1000.0f64,
            0.0..500.0f64,
        ).prop_map(|(province, district, substation, grid_code, lat, lon, capacity, load)| {
            GridLocation {
                province,
                district,
                substation,
                grid_code,
                coordinates: GridCoordinates {
                    latitude: lat,
                    longitude: lon,
                },
                capacity_mw: capacity,
                current_load_mw: load,
            }
        })
    }
    
    /// Generate arbitrary energy sources
    pub fn arb_energy_source() -> impl Strategy<Value = EnergySource> {
        prop_oneof![
            Just(EnergySource::Solar),
            Just(EnergySource::Wind),
            Just(EnergySource::Hydro),
            Just(EnergySource::Biomass),
            Just(EnergySource::Mixed),
        ]
    }
}
