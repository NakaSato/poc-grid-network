use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use tokio::time::sleep;
use tokio::task::JoinSet;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{SystemConfig, ThaiEnergyTradingSystem, SystemResult, SystemError};
use crate::types::*;
use crate::utils::SystemResult as Utils;

/// TPS Test Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpsTestConfig {
    /// Test duration in seconds
    pub test_duration_seconds: u64,
    /// Number of concurrent users/threads
    pub concurrent_users: u32,
    /// Transaction type to test
    pub transaction_type: TransactionTestType,
    /// Target transactions per second (0 = unlimited)
    pub target_tps: u32,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u64,
    /// Minimum success rate percentage
    pub min_success_rate: f64,
}

/// Types of transactions to test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionTestType {
    EnergyOrders,
    TokenTransfers,
    GovernanceVotes,
    MixedWorkload,
}

/// TPS Test Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpsTestResults {
    pub test_config: TpsTestConfig,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub duration_seconds: f64,
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub transactions_per_second: f64,
    pub success_rate_percent: f64,
    pub average_latency_ms: f64,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
    pub system_metrics: SystemMetrics,
}

/// System performance metrics during testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub memory_usage_mb: u64,
    pub cpu_usage_percent: f64,
    pub database_connections: u32,
    pub cache_hit_rate: f64,
    pub blockchain_block_height: u64,
    pub pending_transactions: u32,
}

/// Individual transaction result
#[derive(Debug, Clone)]
struct TransactionResult {
    success: bool,
    latency_ms: u64,
    error: Option<String>,
}

/// TPS Test Engine
pub struct TpsTestEngine {
    system: Arc<ThaiEnergyTradingSystem>,
    results: Arc<Mutex<Vec<TransactionResult>>>,
    start_time: Instant,
}

impl TpsTestEngine {
    /// Create new TPS test engine
    pub async fn new(config: SystemConfig) -> SystemResult<Self> {
        let system = Arc::new(ThaiEnergyTradingSystem::new(config).await?);
        
        Ok(Self {
            system,
            results: Arc::new(Mutex::new(Vec::new())),
            start_time: Instant::now(),
        })
    }

    /// Run comprehensive TPS test suite
    pub async fn run_full_test_suite(&self) -> SystemResult<Vec<TpsTestResults>> {
        log::info!("🔋 Starting GridTokenX POC Full TPS Test Suite");
        
        let test_configs = vec![
            // Single user baseline tests
            TpsTestConfig {
                test_duration_seconds: 30,
                concurrent_users: 1,
                transaction_type: TransactionTestType::EnergyOrders,
                target_tps: 0,
                max_latency_ms: 1000,
                min_success_rate: 95.0,
            },
            TpsTestConfig {
                test_duration_seconds: 30,
                concurrent_users: 1,
                transaction_type: TransactionTestType::TokenTransfers,
                target_tps: 0,
                max_latency_ms: 500,
                min_success_rate: 99.0,
            },
            TpsTestConfig {
                test_duration_seconds: 30,
                concurrent_users: 1,
                transaction_type: TransactionTestType::GovernanceVotes,
                target_tps: 0,
                max_latency_ms: 2000,
                min_success_rate: 99.0,
            },
            // Concurrent user tests
            TpsTestConfig {
                test_duration_seconds: 60,
                concurrent_users: 5,
                transaction_type: TransactionTestType::EnergyOrders,
                target_tps: 0,
                max_latency_ms: 1500,
                min_success_rate: 90.0,
            },
            TpsTestConfig {
                test_duration_seconds: 60,
                concurrent_users: 10,
                transaction_type: TransactionTestType::EnergyOrders,
                target_tps: 0,
                max_latency_ms: 2000,
                min_success_rate: 85.0,
            },
            TpsTestConfig {
                test_duration_seconds: 60,
                concurrent_users: 20,
                transaction_type: TransactionTestType::MixedWorkload,
                target_tps: 0,
                max_latency_ms: 3000,
                min_success_rate: 80.0,
            },
            // High concurrency stress tests
            TpsTestConfig {
                test_duration_seconds: 120,
                concurrent_users: 50,
                transaction_type: TransactionTestType::MixedWorkload,
                target_tps: 0,
                max_latency_ms: 5000,
                min_success_rate: 70.0,
            },
            TpsTestConfig {
                test_duration_seconds: 120,
                concurrent_users: 100,
                transaction_type: TransactionTestType::MixedWorkload,
                target_tps: 0,
                max_latency_ms: 10000,
                min_success_rate: 60.0,
            },
        ];

        let mut all_results = Vec::new();

        for (i, test_config) in test_configs.iter().enumerate() {
            log::info!("🧪 Running TPS Test {}/{}: {:?} with {} users", 
                i + 1, test_configs.len(), test_config.transaction_type, test_config.concurrent_users);
            
            let results = self.run_single_test(test_config.clone()).await?;
            all_results.push(results);
            
            // Brief pause between tests for system stabilization
            if i < test_configs.len() - 1 {
                log::info!("⏸️  Cooling down for 10 seconds...");
                sleep(Duration::from_secs(10)).await;
            }
        }

        log::info!("✅ Full TPS Test Suite completed - {} test configurations", all_results.len());
        Ok(all_results)
    }

    /// Run a single TPS test configuration
    pub async fn run_single_test(&self, config: TpsTestConfig) -> SystemResult<TpsTestResults> {
        log::info!("🚀 Starting TPS test: {:?} - {} users for {}s", 
            config.transaction_type, config.concurrent_users, config.test_duration_seconds);

        // Clear previous results
        self.results.lock().unwrap().clear();
        let start_time = Instant::now();

        // Atomic counters for real-time metrics
        let total_counter = Arc::new(AtomicU64::new(0));
        let success_counter = Arc::new(AtomicU64::new(0));
        let failed_counter = Arc::new(AtomicU64::new(0));

        // Spawn concurrent user tasks
        let mut tasks = JoinSet::new();
        
        for user_id in 0..config.concurrent_users {
            let user_config = config.clone();
            let system_clone = Arc::clone(&self.system);
            let results_clone = Arc::clone(&self.results);
            let total_clone = Arc::clone(&total_counter);
            let success_clone = Arc::clone(&success_counter);
            let failed_clone = Arc::clone(&failed_counter);

            tasks.spawn(async move {
                Self::run_user_load(
                    user_id,
                    user_config,
                    system_clone,
                    results_clone,
                    total_clone,
                    success_clone,
                    failed_clone,
                ).await
            });
        }

        // Wait for all tasks to complete
        while let Some(result) = tasks.join_next().await {
            match result {
                Ok(_) => {},
                Err(e) => log::error!("User task failed: {:?}", e),
            }
        }

        let actual_duration = start_time.elapsed().as_secs_f64();
        let results = self.calculate_results(config, actual_duration).await?;
        
        log::info!("📊 Test completed: {:.2} TPS, {:.1}% success rate, {:.0}ms avg latency",
            results.transactions_per_second, results.success_rate_percent, results.average_latency_ms);

        Ok(results)
    }

    /// Run load generation for a single user/thread
    async fn run_user_load(
        user_id: u32,
        config: TpsTestConfig,
        system: Arc<ThaiEnergyTradingSystem>,
        results: Arc<Mutex<Vec<TransactionResult>>>,
        total_counter: Arc<AtomicU64>,
        success_counter: Arc<AtomicU64>,
        failed_counter: Arc<AtomicU64>,
    ) -> SystemResult<()> {
        let end_time = Instant::now() + Duration::from_secs(config.test_duration_seconds);
        let mut transaction_count = 0;

        while Instant::now() < end_time {
            let tx_start = Instant::now();
            
            let result = match config.transaction_type {
                TransactionTestType::EnergyOrders => {
                    Self::execute_energy_order_transaction(&system, user_id).await
                },
                TransactionTestType::TokenTransfers => {
                    Self::execute_token_transfer_transaction(&system, user_id).await
                },
                TransactionTestType::GovernanceVotes => {
                    Self::execute_governance_vote_transaction(&system, user_id).await
                },
                TransactionTestType::MixedWorkload => {
                    // Randomly select transaction type
                    match transaction_count % 3 {
                        0 => Self::execute_energy_order_transaction(&system, user_id).await,
                        1 => Self::execute_token_transfer_transaction(&system, user_id).await,
                        _ => Self::execute_governance_vote_transaction(&system, user_id).await,
                    }
                }
            };

            let latency_ms = tx_start.elapsed().as_millis() as u64;
            
            let tx_result = match result {
                Ok(_) => {
                    success_counter.fetch_add(1, Ordering::Relaxed);
                    TransactionResult {
                        success: true,
                        latency_ms,
                        error: None,
                    }
                },
                Err(e) => {
                    failed_counter.fetch_add(1, Ordering::Relaxed);
                    TransactionResult {
                        success: false,
                        latency_ms,
                        error: Some(e.to_string()),
                    }
                }
            };

            // Store result
            {
                let mut results_guard = results.lock().unwrap();
                results_guard.push(tx_result);
            }

            total_counter.fetch_add(1, Ordering::Relaxed);
            transaction_count += 1;

            // Small delay to prevent overwhelming the system
            if config.target_tps > 0 {
                let target_interval = Duration::from_millis(1000 / config.target_tps as u64);
                if tx_start.elapsed() < target_interval {
                    sleep(target_interval - tx_start.elapsed()).await;
                }
            } else {
                sleep(Duration::from_millis(1)).await;
            }
        }

        log::debug!("User {} completed {} transactions", user_id, transaction_count);
        Ok(())
    }

    /// Execute energy order transaction
    async fn execute_energy_order_transaction(
        system: &ThaiEnergyTradingSystem,
        user_id: u32,
    ) -> SystemResult<()> {
        use crate::types::EnergyOrder;
        use crate::types::OrderStatus;
        use chrono::{Duration, DateTime, Utc};
        use uuid::Uuid;
        
        let order = EnergyOrder {
            id: Uuid::new_v4(),
            order_type: if user_id % 2 == 0 { OrderType::Buy } else { OrderType::Sell },
            energy_amount: 50.0 + (user_id as f64 * 10.0),
            price_per_unit: 100000 + (user_id as u128 * 1000), // Token price
            energy_source: Some(EnergySource::Solar),
            location: GridLocation {
                province: "Bangkok".to_string(),
                district: "Chatuchak".to_string(),
                coordinates: GridCoordinates { lat: 13.8, lng: 100.55 },
                region: "Central".to_string(),
                substation: "BKK-001".to_string(),
                grid_code: "TH-BKK-001".to_string(),
                meter_id: format!("meter_{}", user_id),
            },
            timestamp: Utc::now(),
            status: OrderStatus::Pending,
            account_id: format!("trader_{}", user_id),
            updated_at: Utc::now(),
        };

        // Simulate order processing
        system.get_trading_service().place_order(order).await?;
        Ok(())
    }

    /// Execute token transfer transaction
    async fn execute_token_transfer_transaction(
        system: &ThaiEnergyTradingSystem,
        user_id: u32,
    ) -> SystemResult<()> {
        use crate::application::trading::TokenTransfer;
        use crate::application::trading::TransferType;
        
        let transfer = TokenTransfer {
            from_account: format!("account_{}", user_id),
            to_account: format!("account_{}", (user_id + 1) % 1000),
            amount: 10 + (user_id as u128 % 100),
            transfer_type: TransferType::EnergyPayment,
            timestamp: chrono::Utc::now(),
            transaction_id: format!("tx_{}_{}", user_id, chrono::Utc::now().timestamp()),
        };

        // Simulate token transfer processing
        system.get_trading_service().process_transfer(transfer).await
    }

    /// Execute governance vote transaction
    async fn execute_governance_vote_transaction(
        system: &ThaiEnergyTradingSystem,
        user_id: u32,
    ) -> SystemResult<()> {
        use crate::application::governance::GovernanceVote;
        use crate::application::governance::VoteType;
        
        let vote = GovernanceVote {
            proposal_id: "test_proposal".to_string(),
            voter_id: format!("voter_{}", user_id),
            vote: if user_id % 3 == 0 { VoteType::Approve } else if user_id % 3 == 1 { VoteType::Reject } else { VoteType::Abstain },
            voting_power: 100 + (user_id as u64 % 500),
            timestamp: chrono::Utc::now(),
        };

        // Simulate governance vote processing
        system.get_governance_service().cast_vote(vote).await
    }

    /// Calculate final test results
    async fn calculate_results(
        &self,
        config: TpsTestConfig,
        actual_duration: f64,
    ) -> SystemResult<TpsTestResults> {
        let results_guard = self.results.lock().unwrap();
        let total_transactions = results_guard.len() as u64;
        let successful_transactions = results_guard.iter().filter(|r| r.success).count() as u64;
        let failed_transactions = total_transactions - successful_transactions;

        // Calculate latency statistics
        let mut latencies: Vec<u64> = results_guard.iter().map(|r| r.latency_ms).collect();
        latencies.sort_unstable();

        let average_latency_ms = if !latencies.is_empty() {
            latencies.iter().sum::<u64>() as f64 / latencies.len() as f64
        } else {
            0.0
        };

        let min_latency_ms = latencies.first().copied().unwrap_or(0);
        let max_latency_ms = latencies.last().copied().unwrap_or(0);
        
        let p95_latency_ms = if !latencies.is_empty() {
            let index = (latencies.len() as f64 * 0.95) as usize;
            latencies.get(index.saturating_sub(1)).copied().unwrap_or(0)
        } else {
            0
        };

        let p99_latency_ms = if !latencies.is_empty() {
            let index = (latencies.len() as f64 * 0.99) as usize;
            latencies.get(index.saturating_sub(1)).copied().unwrap_or(0)
        } else {
            0
        };

        let transactions_per_second = total_transactions as f64 / actual_duration;
        let success_rate_percent = if total_transactions > 0 {
            (successful_transactions as f64 / total_transactions as f64) * 100.0
        } else {
            0.0
        };

        // Get system metrics
        let system_metrics = self.collect_system_metrics().await?;

        Ok(TpsTestResults {
            test_config: config,
            timestamp: chrono::Utc::now(),
            duration_seconds: actual_duration,
            total_transactions,
            successful_transactions,
            failed_transactions,
            transactions_per_second,
            success_rate_percent,
            average_latency_ms,
            min_latency_ms,
            max_latency_ms,
            p95_latency_ms,
            p99_latency_ms,
            system_metrics,
        })
    }

    /// Collect system performance metrics
    async fn collect_system_metrics(&self) -> SystemResult<SystemMetrics> {
        // Get system status from blockchain system
        let status = self.system.get_system_status().await?;
        
        Ok(SystemMetrics {
            memory_usage_mb: status.memory_usage_kb / 1024,
            cpu_usage_percent: 0.0, // Would need additional monitoring
            database_connections: status.active_connections,
            cache_hit_rate: status.cache_hit_rate,
            blockchain_block_height: status.current_block_height,
            pending_transactions: status.pending_transactions,
        })
    }

    /// Generate comprehensive TPS test report
    pub fn generate_report(&self, results: &[TpsTestResults]) -> String {
        let mut report = String::new();
        
        report.push_str("# 🔋 GridTokenX POC - TPS Performance Test Report\n\n");
        report.push_str(&format!("**Generated:** {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**Test Configurations:** {}\n\n", results.len()));

        report.push_str("## 📊 Executive Summary\n\n");
        
        if let Some(best_tps) = results.iter().map(|r| r.transactions_per_second).fold(None, |max, x| {
            Some(max.map_or(x, |m| if x > m { x } else { m }))
        }) {
            report.push_str(&format!("**Peak TPS:** {:.2} transactions/second\n", best_tps));
        }
        
        let avg_success_rate = results.iter().map(|r| r.success_rate_percent).sum::<f64>() / results.len() as f64;
        report.push_str(&format!("**Average Success Rate:** {:.1}%\n", avg_success_rate));
        
        let total_transactions: u64 = results.iter().map(|r| r.total_transactions).sum();
        report.push_str(&format!("**Total Transactions Tested:** {}\n\n", total_transactions));

        report.push_str("## 🎯 Detailed Results\n\n");
        report.push_str("| Users | Type | Duration | TPS | Success Rate | Avg Latency | P95 Latency | P99 Latency |\n");
        report.push_str("|-------|------|----------|-----|--------------|-------------|-------------|-------------|\n");
        
        for result in results {
            report.push_str(&format!(
                "| {} | {:?} | {:.0}s | {:.2} | {:.1}% | {:.0}ms | {}ms | {}ms |\n",
                result.test_config.concurrent_users,
                result.test_config.transaction_type,
                result.duration_seconds,
                result.transactions_per_second,
                result.success_rate_percent,
                result.average_latency_ms,
                result.p95_latency_ms,
                result.p99_latency_ms
            ));
        }

        report.push_str("\n## 🔍 Performance Analysis\n\n");
        report.push_str("### Key Insights:\n");
        report.push_str("- **Scalability:** Performance characteristics with increased concurrent users\n");
        report.push_str("- **Transaction Types:** Relative performance of different transaction types\n");
        report.push_str("- **Latency Distribution:** P95 and P99 latencies for SLA planning\n");
        report.push_str("- **System Stability:** Success rates under various load conditions\n\n");

        report.push_str("### Recommendations:\n");
        report.push_str("1. **Optimal Concurrency:** Monitor resource utilization for best performance/cost ratio\n");
        report.push_str("2. **SLA Planning:** Use P95/P99 latency metrics for realistic SLA targets\n");
        report.push_str("3. **Capacity Planning:** Plan for peak TPS with appropriate safety margins\n");
        report.push_str("4. **Monitoring:** Implement real-time TPS and latency monitoring in production\n\n");

        report.push_str("---\n");
        report.push_str("*GridTokenX POC Blockchain - Rust TPS Testing Framework*\n");
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tps_engine_creation() {
        let config = SystemConfig::default();
        let engine = TpsTestEngine::new(config).await;
        assert!(engine.is_ok());
    }
    
    #[tokio::test]
    async fn test_single_user_baseline() {
        let config = SystemConfig::default();
        let engine = TpsTestEngine::new(config).await.unwrap();
        
        let test_config = TpsTestConfig {
            test_duration_seconds: 5,
            concurrent_users: 1,
            transaction_type: TransactionTestType::EnergyOrders,
            target_tps: 0,
            max_latency_ms: 1000,
            min_success_rate: 90.0,
        };
        
        let results = engine.run_single_test(test_config).await;
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert!(results.total_transactions > 0);
        assert!(results.transactions_per_second > 0.0);
    }
}
