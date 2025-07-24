//! # Utility Functions for Thai Energy Trading System
//! 
//! This module provides common utility functions used throughout the system.

use anyhow::Result;
use crate::types::AccountId;
use uuid::Uuid;
use blake2::{Blake2b512, Digest};

/// Generate a unique transaction ID
pub fn generate_transaction_id() -> String {
    format!("tx_{}", Uuid::new_v4())
}

/// Generate a unique order ID
pub fn generate_order_id() -> Uuid {
    Uuid::new_v4()
}

/// Convert account ID to human-readable format
pub fn account_id_to_string(account_id: &AccountId) -> String {
    account_id.clone()
}

/// Convert string to account ID
pub fn string_to_account_id(account_str: &str) -> Result<AccountId> {
    Ok(account_str.to_string())
}

/// Calculate hash of data
pub fn calculate_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Generate a secure random string
pub fn generate_secure_random(length: usize) -> String {
    use rand::Rng;
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789";
    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

/// Cryptographic utilities for key management and account handling
pub mod crypto {
    use sha2::{Sha256, Digest};
    use hex;
    
    /// Validate AccountId format for GridTokenX
    /// 
    /// Valid format: gx_[32 hexadecimal characters]
    /// Example: gx_a1b2c3d4e5f6789012345678901234ab
    pub fn validate_account_id(account_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        // Check minimum length (gx_ + 32 hex chars = 35 total)
        if account_id.len() != 35 {
            return Ok(false);
        }
        
        // Check prefix
        if !account_id.starts_with("gx_") {
            return Ok(false);
        }
        
        // Extract and validate hex part
        let hex_part = &account_id[3..];
        if hex_part.len() != 32 {
            return Ok(false);
        }
        
        // Verify hex encoding (should decode to 16 bytes)
        match hex::decode(hex_part) {
            Ok(decoded) if decoded.len() == 16 => Ok(true),
            _ => Ok(false),
        }
    }
    
    /// Generate a test AccountId for development purposes
    pub fn generate_test_account_id() -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        let random_bytes: [u8; 16] = rng.random();
        let hex_string = hex::encode(random_bytes);
        format!("gx_{}", hex_string)
    }
    
    /// Hash data using SHA256
    pub fn hash_sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
    
    /// Hash data using SHA256 and return hex string
    pub fn hash_sha256_hex(data: &[u8]) -> String {
        let hash = hash_sha256(data);
        hex::encode(hash)
    }
    
    /// Derive a consistent hash from multiple inputs
    pub fn hash_multiple_inputs(inputs: &[&[u8]]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for input in inputs {
            hasher.update(input);
        }
        hasher.finalize().into()
    }
    
    /// Generate a secure nonce for cryptographic operations
    pub fn generate_nonce() -> [u8; 12] {
        use rand::Rng;
        let mut rng = rand::rng();
        rng.random()
    }
    
    /// Generate secure random bytes
    pub fn generate_random_bytes(length: usize) -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::rng();
        (0..length).map(|_| rng.random()).collect()
    }
    
    /// Constant-time comparison to prevent timing attacks
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
    
    /// Validate hex string format
    pub fn is_valid_hex(hex_str: &str) -> bool {
        hex::decode(hex_str).is_ok()
    }
    
    /// Convert bytes to hex string
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }
    
    /// Convert hex string to bytes
    pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(hex_str)
    }
    
    /// Verify a signature using Ed25519
    pub fn verify_signature(public_key: &[u8], signature: &[u8], message: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        use ed25519_dalek::{VerifyingKey, Signature, Verifier};
        
        if public_key.len() != 32 {
            return Ok(false);
        }
        
        if signature.len() != 64 {
            return Ok(false);
        }
        
        let verifying_key = VerifyingKey::from_bytes(public_key.try_into().unwrap())?;
        let signature = Signature::from_bytes(signature.try_into().unwrap());
        
        match verifying_key.verify(message, &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Derive AccountId from public key bytes
    pub fn derive_account_id(public_key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        if public_key.len() != 32 {
            return Err("Public key must be 32 bytes".into());
        }
        
        let hash = hash_sha256(public_key);
        let hex_string = hex::encode(&hash[0..16]); // Take first 16 bytes (32 hex chars)
        Ok(format!("gx_{}", hex_string))
    }
}

/// Time utilities
pub mod time {
    use chrono::{DateTime, Utc};
    
    /// Get current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// Convert timestamp to Unix epoch
    pub fn to_unix_timestamp(dt: DateTime<Utc>) -> i64 {
        dt.timestamp()
    }
    
    /// Convert Unix epoch to timestamp
    pub fn from_unix_timestamp(timestamp: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| Utc::now())
    }
    
    /// Check if timestamp is within the last N seconds
    pub fn is_within_last_seconds(timestamp: DateTime<Utc>, seconds: i64) -> bool {
        let now = Utc::now();
        let diff = now.signed_duration_since(timestamp);
        diff.num_seconds() <= seconds
    }
    
    /// Format timestamp for display
    pub fn format_timestamp(timestamp: DateTime<Utc>) -> String {
        timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

/// Validation utilities
/// Validation utilities
pub mod validation {
    use crate::utils::string_to_account_id;
    
    /// Validate email format
    pub fn is_valid_email(email: &str) -> bool {
        let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }
    
    /// Validate energy amount
    pub fn is_valid_energy_amount(amount: u64) -> bool {
        amount > 0 && amount <= 1_000_000 // Max 1 million kWh
    }
    
    /// Validate price
    pub fn is_valid_price(price: u128) -> bool {
        price > 0 && price <= 100_000_000 // Max 100 million tokens
    }
    
    /// Validate grid location
    pub fn is_valid_grid_location(location: &crate::types::GridLocation) -> bool {
        !location.province.is_empty() &&
        !location.district.is_empty() &&
        !location.substation.is_empty() &&
        !location.grid_code.is_empty() &&
        location.coordinates.lat >= -90.0 && location.coordinates.lat <= 90.0 &&
        location.coordinates.lng >= -180.0 && location.coordinates.lng <= 180.0
    }
    
    /// Validate account ID format
    pub fn is_valid_account_id(account_id: &str) -> bool {
        string_to_account_id(account_id).is_ok()
    }
}

/// Conversion utilities
pub mod conversion {
    /// Convert kWh to tokens (1:1 ratio)
    pub fn kwh_to_tokens(kwh: u64) -> u128 {
        kwh as u128
    }
    
    /// Convert tokens to kWh (1:1 ratio)
    pub fn tokens_to_kwh(tokens: u128) -> u64 {
        tokens as u64
    }
    
    /// Convert price per kWh to total price
    pub fn calculate_total_price(kwh: u64, price_per_kwh: u128) -> u128 {
        kwh as u128 * price_per_kwh
    }
    
    /// Convert grid utilization to congestion level
    pub fn utilization_to_congestion(utilization: f32) -> crate::types::CongestionLevel {
        match utilization {
            0.0..=50.0 => crate::types::CongestionLevel::Low,
            50.1..=80.0 => crate::types::CongestionLevel::Medium,
            80.1..=95.0 => crate::types::CongestionLevel::High,
            _ => crate::types::CongestionLevel::Critical,
        }
    }
}

/// Formatting utilities
pub mod formatting {
    
    /// Format balance with proper decimal places
    pub fn format_balance(balance: u128) -> String {
        format!("{:.4} THB", balance as f64 / 10000.0)
    }
    
    /// Format energy amount
    pub fn format_energy(energy: u64) -> String {
        format!("{} kWh", energy)
    }
    
    /// Format price
    pub fn format_price(price: u128) -> String {
        format!("{:.4} THB/kWh", price as f64 / 10000.0)
    }
    
    /// Format percentage
    pub fn format_percentage(value: f32) -> String {
        format!("{:.2}%", value)
    }
    
    /// Format file size
    pub fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;
        
        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }
        
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Mathematical utilities
pub mod math {
    
    /// Calculate percentage
    pub fn calculate_percentage(value: f64, total: f64) -> f64 {
        if total == 0.0 {
            0.0
        } else {
            (value / total) * 100.0
        }
    }
    
    /// Calculate average
    pub fn calculate_average(values: &[f64]) -> f64 {
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    }
    
    /// Calculate median
    pub fn calculate_median(values: &mut [f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = values.len();
        
        if len % 2 == 0 {
            (values[len / 2 - 1] + values[len / 2]) / 2.0
        } else {
            values[len / 2]
        }
    }
    
    /// Calculate standard deviation
    pub fn calculate_standard_deviation(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        
        let mean = calculate_average(values);
        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        
        variance.sqrt()
    }
    
    /// Calculate compound interest
    pub fn calculate_compound_interest(
        principal: f64,
        rate: f64,
        time: f64,
        compound_frequency: f64,
    ) -> f64 {
        principal * (1.0 + rate / compound_frequency).powf(compound_frequency * time)
    }
}

/// Network utilities
pub mod network {
    
    /// Check if IP address is valid
    pub fn is_valid_ip(ip: &str) -> bool {
        ip.parse::<std::net::IpAddr>().is_ok()
    }
    
    /// Check if port is valid
    pub fn is_valid_port(port: u16) -> bool {
        port > 0 && port <= 65535
    }
    
    /// Generate network address
    pub fn generate_network_address(ip: &str, port: u16) -> String {
        format!("{}:{}", ip, port)
    }
    
    /// Extract IP from network address
    pub fn extract_ip_from_address(address: &str) -> Option<String> {
        address.split(':').next().map(|s| s.to_string())
    }
    
    /// Extract port from network address
    pub fn extract_port_from_address(address: &str) -> Option<u16> {
        address.split(':').nth(1)?.parse().ok()
    }
}

/// Caching utilities
pub mod cache {
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    
    /// Simple in-memory cache with TTL
    pub struct Cache<K, V> {
        data: HashMap<K, CacheEntry<V>>,
        ttl: Duration,
    }
    
    struct CacheEntry<V> {
        value: V,
        expires_at: Instant,
    }
    
    impl<K, V> Cache<K, V>
    where
        K: std::hash::Hash + Eq + Clone,
        V: Clone,
    {
        pub fn new(ttl: Duration) -> Self {
            Self {
                data: HashMap::new(),
                ttl,
            }
        }
        
        pub fn get(&mut self, key: &K) -> Option<V> {
            self.cleanup_expired();
            self.data.get(key).map(|entry| entry.value.clone())
        }
        
        pub fn set(&mut self, key: K, value: V) {
            let entry = CacheEntry {
                value,
                expires_at: Instant::now() + self.ttl,
            };
            self.data.insert(key, entry);
        }
        
        pub fn remove(&mut self, key: &K) -> Option<V> {
            self.data.remove(key).map(|entry| entry.value)
        }
        
        pub fn clear(&mut self) {
            self.data.clear();
        }
        
        fn cleanup_expired(&mut self) {
            let now = Instant::now();
            self.data.retain(|_, entry| entry.expires_at > now);
        }
    }
}

/// Error handling utilities
pub mod error {
    
    /// Standard error types for the system
    #[derive(Debug, thiserror::Error)]
    pub enum SystemError {
        #[error("Database error: {0}")]
        Database(#[from] sqlx::Error),
        
        #[error("Network error: {0}")]
        Network(String),
        
        #[error("Validation error: {0}")]
        Validation(String),
        
        #[error("Authentication error: {0}")]
        Authentication(String),
        
        #[error("Authorization error: {0}")]
        Authorization(String),
        
        #[error("Grid error: {0}")]
        Grid(String),
        
        #[error("Trading error: {0}")]
        Trading(String),
        
        #[error("Blockchain error: {0}")]
        Blockchain(String),
        
        #[error("Configuration error: {0}")]
        Configuration(String),
        
        #[error("Internal error: {0}")]
        Internal(String),
        
        #[error("Invalid order: {0}")]
        InvalidOrder(String),
        
        #[error("Invalid input: {0}")]
        InvalidInput(String),
        
        #[error("Not found: {0}")]
        NotFound(String),
        
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
    }
    
    // Add From<anyhow::Error> implementation for TPS testing
    impl From<anyhow::Error> for SystemError {
        fn from(err: anyhow::Error) -> Self {
            SystemError::Internal(err.to_string())
        }
    }
    
    /// Result type alias for system operations
    pub type SystemResult<T> = Result<T, SystemError>;
    
    /// Convert any error to system error
    pub fn to_system_error(err: impl std::error::Error) -> SystemError {
        SystemError::Internal(err.to_string())
    }
}

/// Testing utilities
pub mod testing {
    use crate::types::GridCoordinates;
    use crate::AccountId;
    use crate::utils::{generate_order_id, time};
    
    /// Create a test account ID
    pub fn create_test_account_id() -> AccountId {
        "test_account_123".to_string()
    }
    
    /// Create test grid location
    pub fn create_test_grid_location() -> crate::types::GridLocation {
        crate::types::GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            substation: "Siam".to_string(),
            grid_code: "BKK-001".to_string(),
            coordinates: GridCoordinates { lat: 13.7463, lng: 100.5352 },
            region: "Central".to_string(),
            meter_id: "METER-001".to_string(),
        }
    }
    
    /// Create test energy order
    pub fn create_test_energy_order() -> crate::types::EnergyOrder {
        crate::types::EnergyOrder {
            id: generate_order_id(),
            account_id: create_test_account_id(),
            order_type: crate::types::OrderType::Buy,
            energy_amount: 100.0,
            price_per_unit: 5000,
            energy_source: Some(crate::types::EnergySource::Solar),
            location: create_test_grid_location(),
            timestamp: time::now(),
            status: crate::types::OrderStatus::Pending,
            updated_at: time::now(),
        }
    }
}

/// Logging utilities
pub mod logging {
    use log::{debug, error, info, warn};
    
    /// Initialize logging with proper formatting
    pub fn init_logging() {
        env_logger::Builder::from_default_env()
            .format_timestamp_secs()
            .init();
    }
    
    /// Log system startup
    pub fn log_startup(component: &str) {
        info!("🚀 Starting {}", component);
    }
    
    /// Log system shutdown
    pub fn log_shutdown(component: &str) {
        info!("🛑 Stopping {}", component);
    }
    
    /// Log error with context
    pub fn log_error(component: &str, error: &dyn std::error::Error) {
        error!("❌ [{}] Error: {}", component, error);
    }
    
    /// Log warning with context
    pub fn log_warning(component: &str, message: &str) {
        warn!("⚠️  [{}] Warning: {}", component, message);
    }
    
    /// Log info with context
    pub fn log_info(component: &str, message: &str) {
        info!("ℹ️  [{}] {}", component, message);
    }
    
    /// Log debug with context
    pub fn log_debug(component: &str, message: &str) {
        debug!("🔍 [{}] {}", component, message);
    }
}

// Re-export commonly used functions
pub use time::now;
pub use validation::*;
pub use conversion::*;
pub use formatting::*;
pub use error::{SystemError, SystemResult};
pub use crypto::*;
