//! # Test Helper Functions and Utilities
//! 
//! Common utilities for testing across the system

use crate::types::*;
use uuid::Uuid;
use chrono::Utc;
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Test timing utilities
pub struct TestTimer {
    start: Instant,
}

impl TestTimer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
    
    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    
    pub fn elapsed_us(&self) -> u64 {
        self.start.elapsed().as_micros() as u64
    }
}

/// Async test utilities
pub struct AsyncTestUtils;

impl AsyncTestUtils {
    /// Wait for condition with timeout
    pub async fn wait_for_condition<F, Fut>(
        mut condition: F,
        timeout_ms: u64,
        check_interval_ms: u64,
    ) -> bool
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let timeout_duration = Duration::from_millis(timeout_ms);
        let check_interval = Duration::from_millis(check_interval_ms);
        
        let result = timeout(timeout_duration, async {
            loop {
                if condition().await {
                    return true;
                }
                tokio::time::sleep(check_interval).await;
            }
        }).await;
        
        result.unwrap_or(false)
    }
    
    /// Run function with timeout
    pub async fn with_timeout<F, Fut, T>(
        operation: F,
        timeout_ms: u64,
    ) -> Result<T, &'static str>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        match timeout(Duration::from_millis(timeout_ms), operation()).await {
            Ok(result) => Ok(result),
            Err(_) => Err("Operation timed out"),
        }
    }
    
    /// Retry operation with backoff
    pub async fn retry_with_backoff<F, Fut, T, E>(
        mut operation: F,
        max_retries: usize,
        initial_delay_ms: u64,
    ) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let mut delay = Duration::from_millis(initial_delay_ms);
        
        for attempt in 0..max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries - 1 {
                        return Err(e);
                    }
                    tokio::time::sleep(delay).await;
                    delay = delay * 2; // Exponential backoff
                }
            }
        }
        
        unreachable!()
    }
}

/// Performance measurement utilities
pub struct PerformanceTracker {
    measurements: Vec<u64>,
    operation_name: String,
}

impl PerformanceTracker {
    pub fn new(operation_name: &str) -> Self {
        Self {
            measurements: Vec::new(),
            operation_name: operation_name.to_string(),
        }
    }
    
    pub async fn measure<F, Fut, T>(&mut self, operation: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let timer = TestTimer::new();
        let result = operation().await;
        self.measurements.push(timer.elapsed_us());
        result
    }
    
    pub fn stats(&self) -> PerformanceStats {
        if self.measurements.is_empty() {
            return PerformanceStats::default();
        }
        
        let mut sorted = self.measurements.clone();
        sorted.sort_unstable();
        
        let len = sorted.len();
        let sum: u64 = sorted.iter().sum();
        
        PerformanceStats {
            operation_name: self.operation_name.clone(),
            sample_count: len,
            min_us: sorted[0],
            max_us: sorted[len - 1],
            mean_us: sum / len as u64,
            median_us: sorted[len / 2],
            p95_us: sorted[(len as f64 * 0.95) as usize],
            p99_us: sorted[(len as f64 * 0.99) as usize],
        }
    }
    
    pub fn print_stats(&self) {
        let stats = self.stats();
        println!("=== Performance Stats: {} ===", stats.operation_name);
        println!("Samples: {}", stats.sample_count);
        println!("Min: {}μs", stats.min_us);
        println!("Mean: {}μs", stats.mean_us);
        println!("Median: {}μs", stats.median_us);
        println!("P95: {}μs", stats.p95_us);
        println!("P99: {}μs", stats.p99_us);
        println!("Max: {}μs", stats.max_us);
        println!("===============================");
    }
}

#[derive(Debug, Default)]
pub struct PerformanceStats {
    pub operation_name: String,
    pub sample_count: usize,
    pub min_us: u64,
    pub max_us: u64,
    pub mean_us: u64,
    pub median_us: u64,
    pub p95_us: u64,
    pub p99_us: u64,
}

/// Test assertion helpers
#[macro_export]
macro_rules! assert_within_range {
    ($value:expr, $min:expr, $max:expr) => {
        assert!(
            $value >= $min && $value <= $max,
            "Value {} is not within range [{}, {}]",
            $value, $min, $max
        );
    };
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $tolerance:expr) => {
        let diff = ($left - $right).abs();
        assert!(
            diff <= $tolerance,
            "Values {} and {} differ by {} which exceeds tolerance {}",
            $left, $right, diff, $tolerance
        );
    };
}

#[macro_export]
macro_rules! assert_eventually {
    ($timeout_ms:expr, $condition:expr) => {
        let result = crate::helpers::test_utils::AsyncTestUtils::wait_for_condition(
            || async { $condition },
            $timeout_ms,
            100,
        ).await;
        assert!(result, "Condition was not eventually true within {}ms", $timeout_ms);
    };
}

/// Order validation helpers
pub struct OrderValidationHelpers;

impl OrderValidationHelpers {
    pub fn validate_order_consistency(order: &EnergyOrder) -> Result<(), String> {
        if order.energy_amount <= 0.0 {
            return Err("Energy amount must be positive".to_string());
        }
        
        if order.price_per_unit == 0 {
            return Err("Price must be positive".to_string());
        }
        
        if order.account_id.is_empty() {
            return Err("Account ID cannot be empty".to_string());
        }
        
        if order.created_at > order.updated_at {
            return Err("Created time cannot be after updated time".to_string());
        }
        
        if let Some(expires_at) = order.expires_at {
            if expires_at <= order.created_at {
                return Err("Expiry time must be after creation time".to_string());
            }
        }
        
        Ok(())
    }
    
    pub fn validate_trade_execution(execution: &crate::runtime::cda::types::TradeExecution) -> Result<(), String> {
        if execution.quantity <= 0.0 {
            return Err("Trade quantity must be positive".to_string());
        }
        
        if execution.price <= 0.0 {
            return Err("Trade price must be positive".to_string());
        }
        
        if execution.buyer_id == execution.seller_id {
            return Err("Buyer and seller cannot be the same".to_string());
        }
        
        if execution.buy_order_id == execution.sell_order_id {
            return Err("Buy and sell order IDs cannot be the same".to_string());
        }
        
        Ok(())
    }
    
    pub fn validate_market_depth(depth: &crate::runtime::cda::types::MarketDepth) -> Result<(), String> {
        if depth.bids.is_empty() && depth.asks.is_empty() {
            return Ok(()); // Empty book is valid
        }
        
        // Validate bid ordering (highest to lowest)
        for window in depth.bids.windows(2) {
            if window[0].price < window[1].price {
                return Err("Bids should be ordered from highest to lowest price".to_string());
            }
        }
        
        // Validate ask ordering (lowest to highest)  
        for window in depth.asks.windows(2) {
            if window[0].price > window[1].price {
                return Err("Asks should be ordered from lowest to highest price".to_string());
            }
        }
        
        // Validate spread calculation
        if !depth.bids.is_empty() && !depth.asks.is_empty() {
            let expected_spread = depth.asks[0].price - depth.bids[0].price;
            if (depth.spread - expected_spread).abs() > 0.001 {
                return Err("Spread calculation is incorrect".to_string());
            }
            
            let expected_mid = (depth.bids[0].price + depth.asks[0].price) / 2.0;
            if (depth.mid_price - expected_mid).abs() > 0.001 {
                return Err("Mid price calculation is incorrect".to_string());
            }
        }
        
        Ok(())
    }
}

/// Concurrent testing utilities
pub struct ConcurrencyTestUtils;

impl ConcurrencyTestUtils {
    /// Run operations concurrently and collect results
    pub async fn run_concurrent<F, Fut, T>(
        operations: Vec<F>,
    ) -> Vec<T>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = T> + Send,
        T: Send + 'static,
    {
        let handles: Vec<_> = operations
            .into_iter()
            .map(|op| tokio::spawn(async move { op().await }))
            .collect();
        
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        
        results
    }
    
    /// Run operation multiple times concurrently
    pub async fn run_parallel<F, Fut, T>(
        operation: F,
        count: usize,
    ) -> Vec<T>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = T> + Send,
        T: Send + 'static,
    {
        use std::sync::Arc;
        let operation = Arc::new(operation);
        
        let handles: Vec<_> = (0..count)
            .map(|_| {
                let op = Arc::clone(&operation);
                tokio::spawn(async move { op().await })
            })
            .collect();
        
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.await.unwrap());
        }
        
        results
    }
    
    /// Measure concurrency performance
    pub async fn measure_concurrent_performance<F, Fut, T>(
        operation: F,
        concurrent_count: usize,
        iterations_per_task: usize,
    ) -> ConcurrencyPerformanceResult
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = T> + Send,
        T: Send + 'static,
    {
        use std::sync::Arc;
        let operation = Arc::new(operation);
        let start_time = Instant::now();
        
        let handles: Vec<_> = (0..concurrent_count)
            .map(|_| {
                let op = Arc::clone(&operation);
                tokio::spawn(async move {
                    let task_start = Instant::now();
                    for _ in 0..iterations_per_task {
                        op().await;
                    }
                    task_start.elapsed()
                })
            })
            .collect();
        
        let mut task_durations = Vec::new();
        for handle in handles {
            task_durations.push(handle.await.unwrap());
        }
        
        let total_duration = start_time.elapsed();
        let total_operations = concurrent_count * iterations_per_task;
        
        ConcurrencyPerformanceResult {
            concurrent_tasks: concurrent_count,
            operations_per_task: iterations_per_task,
            total_operations,
            total_duration,
            operations_per_second: total_operations as f64 / total_duration.as_secs_f64(),
            task_durations,
        }
    }
}

#[derive(Debug)]
pub struct ConcurrencyPerformanceResult {
    pub concurrent_tasks: usize,
    pub operations_per_task: usize,
    pub total_operations: usize,
    pub total_duration: Duration,
    pub operations_per_second: f64,
    pub task_durations: Vec<Duration>,
}

impl ConcurrencyPerformanceResult {
    pub fn print_stats(&self) {
        println!("=== Concurrency Performance ===");
        println!("Concurrent tasks: {}", self.concurrent_tasks);
        println!("Operations per task: {}", self.operations_per_task);
        println!("Total operations: {}", self.total_operations);
        println!("Total duration: {:?}", self.total_duration);
        println!("Operations/second: {:.2}", self.operations_per_second);
        
        let avg_task_duration = self.task_durations.iter().sum::<Duration>() / self.task_durations.len() as u32;
        println!("Average task duration: {:?}", avg_task_duration);
        
        let min_task_duration = self.task_durations.iter().min().unwrap();
        let max_task_duration = self.task_durations.iter().max().unwrap();
        println!("Task duration range: {:?} - {:?}", min_task_duration, max_task_duration);
        println!("==============================");
    }
}
