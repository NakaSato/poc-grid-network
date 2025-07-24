//! # Comprehensive Unit Tests for Thai Energy Trading System
//! 
//! This module organizes all unit tests for the system components

pub mod unit {
    pub mod cda;
    pub mod database;
}

// Individual test modules  
pub mod simple_cda_tests;
pub mod tps_integration_tests;
pub mod types_tests;
pub mod utils_tests;

// Test support modules
pub mod fixtures;
pub mod helpers;
