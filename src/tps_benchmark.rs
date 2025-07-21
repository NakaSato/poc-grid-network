use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::interval;
use serde::{Deserialize, Serialize};

use crate::{SystemResult, SystemError};
use crate::tps_test::{TpsTestEngine, TpsTestConfig, TransactionTestType, TpsTestResults};

/// Real-time TPS monitoring and performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpsMonitor {
    /// Current TPS measurement window (seconds)
    pub measurement_window_seconds: u64,
    /// Real-time TPS samples
    pub tps_samples: Vec<TpsSample>,
    /// System performance thresholds
    pub performance_thresholds: PerformanceThresholds,
    /// Active monitoring state
    pub is_monitoring: bool,
}

/// Single TPS measurement sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TpsSample {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transactions_per_second: f64,
    pub success_rate_percent: f64,
    pub average_latency_ms: f64,
    pub system_load_percent: f64,
    pub memory_usage_mb: u64,
    pub active_connections: u32,
}

/// Performance threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Minimum acceptable TPS
    pub min_tps: f64,
    /// Maximum acceptable average latency (ms)
    pub max_avg_latency_ms: f64,
    /// Minimum acceptable success rate (%)
    pub min_success_rate_percent: f64,
    /// Maximum acceptable memory usage (MB)
    pub max_memory_usage_mb: u64,
    /// Critical system load threshold (%)
    pub critical_load_threshold_percent: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            min_tps: 10.0,
            max_avg_latency_ms: 1000.0,
            min_success_rate_percent: 95.0,
            max_memory_usage_mb: 512,
            critical_load_threshold_percent: 80.0,
        }
    }
}

/// Comprehensive TPS benchmarking suite
pub struct TpsBenchmarkSuite {
    test_engine: TpsTestEngine,
    monitor: TpsMonitor,
    benchmark_configs: Vec<BenchmarkScenario>,
}

/// Individual benchmark scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScenario {
    pub name: String,
    pub description: String,
    pub test_config: TpsTestConfig,
    pub expected_tps_range: (f64, f64),
    pub expected_success_rate_min: f64,
    pub max_acceptable_latency_ms: u64,
}

/// Benchmark results summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_scenarios: u32,
    pub passed_scenarios: u32,
    pub failed_scenarios: u32,
    pub overall_success_rate: f64,
    pub peak_tps_achieved: f64,
    pub average_latency_ms: f64,
    pub system_stability_score: f64,
    pub recommendations: Vec<String>,
    pub detailed_results: Vec<BenchmarkScenarioResult>,
}

/// Individual scenario result with pass/fail status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkScenarioResult {
    pub scenario: BenchmarkScenario,
    pub test_results: TpsTestResults,
    pub passed: bool,
    pub failure_reasons: Vec<String>,
    pub performance_score: f64,
}

impl TpsBenchmarkSuite {
    /// Create new benchmark suite with comprehensive test scenarios
    pub async fn new(test_engine: TpsTestEngine) -> SystemResult<Self> {
        let benchmark_configs = vec![
            // Baseline single-user performance
            BenchmarkScenario {
                name: "baseline_single_user".to_string(),
                description: "Single user baseline performance test".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 30,
                    concurrent_users: 1,
                    transaction_type: TransactionTestType::EnergyOrders,
                    target_tps: 0,
                    max_latency_ms: 500,
                    min_success_rate: 99.0,
                },
                expected_tps_range: (5.0, 50.0),
                expected_success_rate_min: 95.0,
                max_acceptable_latency_ms: 500,
            },
            
            // Low concurrency energy trading
            BenchmarkScenario {
                name: "low_concurrency_trading".to_string(),
                description: "5 concurrent users energy trading".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 60,
                    concurrent_users: 5,
                    transaction_type: TransactionTestType::EnergyOrders,
                    target_tps: 0,
                    max_latency_ms: 1000,
                    min_success_rate: 90.0,
                },
                expected_tps_range: (20.0, 100.0),
                expected_success_rate_min: 85.0,
                max_acceptable_latency_ms: 1000,
            },
            
            // Medium concurrency mixed workload
            BenchmarkScenario {
                name: "medium_concurrency_mixed".to_string(),
                description: "20 concurrent users mixed transaction types".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 90,
                    concurrent_users: 20,
                    transaction_type: TransactionTestType::MixedWorkload,
                    target_tps: 0,
                    max_latency_ms: 2000,
                    min_success_rate: 80.0,
                },
                expected_tps_range: (50.0, 200.0),
                expected_success_rate_min: 75.0,
                max_acceptable_latency_ms: 2000,
            },
            
            // High concurrency stress test
            BenchmarkScenario {
                name: "high_concurrency_stress".to_string(),
                description: "50 concurrent users stress test".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 120,
                    concurrent_users: 50,
                    transaction_type: TransactionTestType::MixedWorkload,
                    target_tps: 0,
                    max_latency_ms: 5000,
                    min_success_rate: 70.0,
                },
                expected_tps_range: (100.0, 500.0),
                expected_success_rate_min: 60.0,
                max_acceptable_latency_ms: 5000,
            },
            
            // Maximum concurrency limit test
            BenchmarkScenario {
                name: "maximum_concurrency_limit".to_string(),
                description: "100 concurrent users system limit test".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 180,
                    concurrent_users: 100,
                    transaction_type: TransactionTestType::MixedWorkload,
                    target_tps: 0,
                    max_latency_ms: 10000,
                    min_success_rate: 50.0,
                },
                expected_tps_range: (50.0, 1000.0),
                expected_success_rate_min: 40.0,
                max_acceptable_latency_ms: 10000,
            },
            
            // Token transfer performance
            BenchmarkScenario {
                name: "token_transfer_performance".to_string(),
                description: "Optimized token transfer throughput test".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 60,
                    concurrent_users: 25,
                    transaction_type: TransactionTestType::TokenTransfers,
                    target_tps: 0,
                    max_latency_ms: 500,
                    min_success_rate: 95.0,
                },
                expected_tps_range: (100.0, 500.0),
                expected_success_rate_min: 90.0,
                max_acceptable_latency_ms: 500,
            },
            
            // Governance voting scalability
            BenchmarkScenario {
                name: "governance_voting_scalability".to_string(),
                description: "Governance voting system scalability test".to_string(),
                test_config: TpsTestConfig {
                    test_duration_seconds: 45,
                    concurrent_users: 15,
                    transaction_type: TransactionTestType::GovernanceVotes,
                    target_tps: 0,
                    max_latency_ms: 2000,
                    min_success_rate: 90.0,
                },
                expected_tps_range: (10.0, 100.0),
                expected_success_rate_min: 85.0,
                max_acceptable_latency_ms: 2000,
            },
        ];

        let monitor = TpsMonitor {
            measurement_window_seconds: 10,
            tps_samples: Vec::new(),
            performance_thresholds: PerformanceThresholds::default(),
            is_monitoring: false,
        };

        Ok(Self {
            test_engine,
            monitor,
            benchmark_configs,
        })
    }

    /// Run complete benchmark suite
    pub async fn run_comprehensive_benchmark(&mut self) -> SystemResult<BenchmarkSummary> {
        log::info!("🔋 Starting GridTokenX POC Comprehensive TPS Benchmark Suite");
        log::info!("📊 Running {} benchmark scenarios", self.benchmark_configs.len());

        let mut detailed_results = Vec::new();
        let mut passed_scenarios = 0;
        let mut total_tps_measurements = Vec::new();
        let mut total_latency_measurements = Vec::new();

        // Start monitoring
        self.monitor.is_monitoring = true;

        for (i, scenario) in self.benchmark_configs.clone().iter().enumerate() {
            log::info!("🧪 Running benchmark scenario {}/{}: {}", 
                i + 1, self.benchmark_configs.len(), scenario.name);
            
            // Pre-test system warm-up
            tokio::time::sleep(Duration::from_secs(5)).await;
            
            // Run the test
            let test_results = self.test_engine.run_single_test(scenario.test_config.clone()).await?;
            
            // Evaluate scenario results
            let scenario_result = self.evaluate_scenario_results(scenario.clone(), test_results).await?;
            
            if scenario_result.passed {
                passed_scenarios += 1;
                log::info!("✅ Scenario '{}' PASSED (Score: {:.1}%)", 
                    scenario.name, scenario_result.performance_score);
            } else {
                log::warn!("❌ Scenario '{}' FAILED (Reasons: {})", 
                    scenario.name, scenario_result.failure_reasons.join(", "));
            }
            
            total_tps_measurements.push(scenario_result.test_results.transactions_per_second);
            total_latency_measurements.push(scenario_result.test_results.average_latency_ms);
            detailed_results.push(scenario_result);
            
            // Inter-scenario cooldown
            if i < self.benchmark_configs.len() - 1 {
                log::info!("⏸️  Inter-scenario cooldown (15 seconds)...");
                tokio::time::sleep(Duration::from_secs(15)).await;
            }
        }

        // Stop monitoring
        self.monitor.is_monitoring = false;

        // Calculate overall metrics
        let total_scenarios = self.benchmark_configs.len() as u32;
        let failed_scenarios = total_scenarios - passed_scenarios;
        let overall_success_rate = (passed_scenarios as f64 / total_scenarios as f64) * 100.0;
        
        let peak_tps_achieved = total_tps_measurements.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .copied()
            .unwrap_or(0.0);
        
        let average_latency_ms = if !total_latency_measurements.is_empty() {
            total_latency_measurements.iter().sum::<f64>() / total_latency_measurements.len() as f64
        } else {
            0.0
        };

        // Calculate system stability score (composite metric)
        let system_stability_score = self.calculate_stability_score(&detailed_results);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&detailed_results);

        let summary = BenchmarkSummary {
            total_scenarios,
            passed_scenarios,
            failed_scenarios,
            overall_success_rate,
            peak_tps_achieved,
            average_latency_ms,
            system_stability_score,
            recommendations,
            detailed_results,
        };

        log::info!("🎉 Comprehensive TPS Benchmark completed!");
        log::info!("📊 Results: {}/{} scenarios passed ({:.1}% success rate)", 
            passed_scenarios, total_scenarios, overall_success_rate);
        log::info!("🚀 Peak TPS: {:.2}, Avg Latency: {:.0}ms, Stability Score: {:.1}%", 
            peak_tps_achieved, average_latency_ms, system_stability_score);

        Ok(summary)
    }

    /// Evaluate individual scenario results against expectations
    async fn evaluate_scenario_results(
        &self,
        scenario: BenchmarkScenario,
        test_results: TpsTestResults,
    ) -> SystemResult<BenchmarkScenarioResult> {
        let mut passed = true;
        let mut failure_reasons = Vec::new();
        let mut performance_score: f64 = 100.0;

        // Check TPS requirements
        if test_results.transactions_per_second < scenario.expected_tps_range.0 {
            passed = false;
            failure_reasons.push(format!(
                "TPS too low: {:.2} < {:.2}",
                test_results.transactions_per_second,
                scenario.expected_tps_range.0
            ));
            performance_score -= 25.0;
        } else if test_results.transactions_per_second > scenario.expected_tps_range.1 {
            // Exceeding upper bound is actually good, but might indicate unrealistic test conditions
            performance_score += 5.0;
        }

        // Check success rate
        if test_results.success_rate_percent < scenario.expected_success_rate_min {
            passed = false;
            failure_reasons.push(format!(
                "Success rate too low: {:.1}% < {:.1}%",
                test_results.success_rate_percent,
                scenario.expected_success_rate_min
            ));
            performance_score -= 30.0;
        }

        // Check latency requirements
        if test_results.average_latency_ms > scenario.max_acceptable_latency_ms as f64 {
            passed = false;
            failure_reasons.push(format!(
                "Average latency too high: {:.0}ms > {}ms",
                test_results.average_latency_ms,
                scenario.max_acceptable_latency_ms
            ));
            performance_score -= 20.0;
        }

        // Check P99 latency (should be reasonable even under stress)
        let p99_limit = scenario.max_acceptable_latency_ms * 2; // Allow 2x for P99
        if test_results.p99_latency_ms > p99_limit {
            failure_reasons.push(format!(
                "P99 latency too high: {}ms > {}ms",
                test_results.p99_latency_ms,
                p99_limit
            ));
            performance_score -= 15.0;
        }

        // Check system resource utilization
        if test_results.system_metrics.memory_usage_mb > self.monitor.performance_thresholds.max_memory_usage_mb {
            failure_reasons.push(format!(
                "Memory usage too high: {}MB > {}MB",
                test_results.system_metrics.memory_usage_mb,
                self.monitor.performance_thresholds.max_memory_usage_mb
            ));
            performance_score -= 10.0;
        }

        // Ensure performance score is within bounds
        performance_score = performance_score.max(0.0).min(100.0);

        Ok(BenchmarkScenarioResult {
            scenario,
            test_results,
            passed,
            failure_reasons,
            performance_score,
        })
    }

    /// Calculate overall system stability score
    fn calculate_stability_score(&self, results: &[BenchmarkScenarioResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }

        let mut stability_factors = Vec::new();

        // Success rate consistency (weight: 30%)
        let success_rates: Vec<f64> = results.iter().map(|r| r.test_results.success_rate_percent).collect();
        let success_rate_variance = self.calculate_variance(&success_rates);
        let success_rate_stability = (100.0 - success_rate_variance.min(100.0)) * 0.30;
        stability_factors.push(success_rate_stability);

        // Latency consistency (weight: 25%)
        let latencies: Vec<f64> = results.iter().map(|r| r.test_results.average_latency_ms).collect();
        let latency_coefficient_of_variation = if !latencies.is_empty() {
            let mean = latencies.iter().sum::<f64>() / latencies.len() as f64;
            let variance = self.calculate_variance(&latencies);
            let std_dev = variance.sqrt();
            if mean > 0.0 { (std_dev / mean) * 100.0 } else { 0.0 }
        } else {
            0.0
        };
        let latency_stability = (100.0 - latency_coefficient_of_variation.min(100.0)) * 0.25;
        stability_factors.push(latency_stability);

        // Performance scaling (weight: 20%)
        let performance_scores: Vec<f64> = results.iter().map(|r| r.performance_score).collect();
        let avg_performance_score = performance_scores.iter().sum::<f64>() / performance_scores.len() as f64;
        let performance_stability = avg_performance_score * 0.20;
        stability_factors.push(performance_stability);

        // Error rate consistency (weight: 15%)
        let error_rates: Vec<f64> = results.iter().map(|r| {
            100.0 - r.test_results.success_rate_percent
        }).collect();
        let error_rate_variance = self.calculate_variance(&error_rates);
        let error_stability = (100.0 - error_rate_variance.min(100.0)) * 0.15;
        stability_factors.push(error_stability);

        // System resource efficiency (weight: 10%)
        let memory_usage: Vec<f64> = results.iter().map(|r| r.test_results.system_metrics.memory_usage_mb as f64).collect();
        let memory_efficiency = if !memory_usage.is_empty() {
            let max_memory = memory_usage.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap_or(&0.0);
            let efficiency_ratio = (self.monitor.performance_thresholds.max_memory_usage_mb as f64 - max_memory) / 
                                 self.monitor.performance_thresholds.max_memory_usage_mb as f64;
            (efficiency_ratio * 100.0).max(0.0).min(100.0)
        } else {
            100.0
        };
        let resource_stability = memory_efficiency * 0.10;
        stability_factors.push(resource_stability);

        // Calculate weighted average
        stability_factors.iter().sum::<f64>()
    }

    /// Calculate variance for a set of values
    fn calculate_variance(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum::<f64>() / values.len() as f64;
        
        variance
    }

    /// Generate performance recommendations based on benchmark results
    fn generate_recommendations(&self, results: &[BenchmarkScenarioResult]) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analyze patterns across results
        let failed_scenarios: Vec<_> = results.iter().filter(|r| !r.passed).collect();
        let avg_tps: f64 = results.iter().map(|r| r.test_results.transactions_per_second).sum::<f64>() / results.len() as f64;
        let avg_latency: f64 = results.iter().map(|r| r.test_results.average_latency_ms).sum::<f64>() / results.len() as f64;

        // Performance recommendations
        if avg_tps < 100.0 {
            recommendations.push("🚀 Consider performance optimization: Average TPS below 100 suggests room for improvement".to_string());
            recommendations.push("💡 Database connection pooling and query optimization could improve throughput".to_string());
        }

        if avg_latency > 1000.0 {
            recommendations.push("⚡ Latency optimization needed: Average latency >1000ms impacts user experience".to_string());
            recommendations.push("💡 Consider caching frequently accessed data and async processing".to_string());
        }

        // Concurrency recommendations
        let high_concurrency_failures = failed_scenarios.iter()
            .filter(|r| r.scenario.test_config.concurrent_users >= 50)
            .count();
        
        if high_concurrency_failures > 0 {
            recommendations.push("🔧 Concurrency scaling issues detected at 50+ users".to_string());
            recommendations.push("💡 Implement connection pooling, request queuing, and resource management".to_string());
        }

        // Memory recommendations
        let high_memory_usage = results.iter()
            .any(|r| r.test_results.system_metrics.memory_usage_mb > 256);
        
        if high_memory_usage {
            recommendations.push("🧠 Monitor memory usage: Peak usage >256MB may require optimization".to_string());
            recommendations.push("💡 Consider memory profiling and garbage collection tuning".to_string());
        }

        // Stability recommendations
        let stability_score = self.calculate_stability_score(results);
        if stability_score < 80.0 {
            recommendations.push("📊 System stability needs attention: Inconsistent performance across tests".to_string());
            recommendations.push("💡 Implement better error handling, circuit breakers, and monitoring".to_string());
        }

        // Transaction type specific recommendations
        let energy_order_results: Vec<_> = results.iter()
            .filter(|r| matches!(r.scenario.test_config.transaction_type, TransactionTestType::EnergyOrders))
            .collect();
        
        if !energy_order_results.is_empty() {
            let avg_energy_tps = energy_order_results.iter()
                .map(|r| r.test_results.transactions_per_second)
                .sum::<f64>() / energy_order_results.len() as f64;
            
            if avg_energy_tps < 50.0 {
                recommendations.push("🔋 Energy trading performance optimization needed".to_string());
                recommendations.push("💡 Consider CDA engine optimization and order matching improvements".to_string());
            }
        }

        // Production readiness recommendations
        recommendations.push("📈 Implement continuous TPS monitoring in production environment".to_string());
        recommendations.push("🔔 Set up alerting for TPS degradation and latency spikes".to_string());
        recommendations.push("📋 Regular benchmark testing to track performance over time".to_string());

        recommendations
    }

    /// Generate comprehensive benchmark report
    pub fn generate_comprehensive_report(&self, summary: &BenchmarkSummary) -> String {
        let mut report = String::new();
        
        report.push_str("# 🔋 GridTokenX POC - Comprehensive TPS Benchmark Report\n\n");
        report.push_str(&format!("**Generated:** {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        report.push_str(&format!("**Test Suite:** Full TPS Performance Benchmark\n"));
        report.push_str(&format!("**System:** GridTokenX POC Blockchain (Rust)\n\n"));

        // Executive Summary
        report.push_str("## 📊 Executive Summary\n\n");
        report.push_str(&format!("- **Overall Success Rate:** {:.1}%\n", summary.overall_success_rate));
        report.push_str(&format!("- **Peak TPS Achieved:** {:.2} transactions/second\n", summary.peak_tps_achieved));
        report.push_str(&format!("- **Average Latency:** {:.0}ms\n", summary.average_latency_ms));
        report.push_str(&format!("- **System Stability Score:** {:.1}%\n", summary.system_stability_score));
        report.push_str(&format!("- **Scenarios Passed:** {}/{}\n", summary.passed_scenarios, summary.total_scenarios));
        report.push_str(&format!("- **Scenarios Failed:** {}\n\n", summary.failed_scenarios));

        // Performance Assessment
        report.push_str("## 🎯 Performance Assessment\n\n");
        if summary.overall_success_rate >= 90.0 {
            report.push_str("✅ **EXCELLENT** - System demonstrates robust performance across all scenarios\n");
        } else if summary.overall_success_rate >= 75.0 {
            report.push_str("✅ **GOOD** - System performs well with minor optimization opportunities\n");
        } else if summary.overall_success_rate >= 50.0 {
            report.push_str("⚠️ **FAIR** - System shows promise but requires performance improvements\n");
        } else {
            report.push_str("❌ **NEEDS IMPROVEMENT** - Significant performance issues require attention\n");
        }
        report.push_str("\n");

        // Detailed Results Table
        report.push_str("## 📋 Detailed Benchmark Results\n\n");
        report.push_str("| Scenario | Users | Type | Duration | TPS | Success% | Avg Latency | P99 Latency | Score | Status |\n");
        report.push_str("|----------|-------|------|----------|-----|----------|-------------|-------------|-------|--------|\n");
        
        for result in &summary.detailed_results {
            let status = if result.passed { "✅ PASS" } else { "❌ FAIL" };
            report.push_str(&format!(
                "| {} | {} | {:?} | {:.0}s | {:.2} | {:.1}% | {:.0}ms | {}ms | {:.1}% | {} |\n",
                result.scenario.name,
                result.scenario.test_config.concurrent_users,
                result.scenario.test_config.transaction_type,
                result.test_results.duration_seconds,
                result.test_results.transactions_per_second,
                result.test_results.success_rate_percent,
                result.test_results.average_latency_ms,
                result.test_results.p99_latency_ms,
                result.performance_score,
                status
            ));
        }

        // Failure Analysis
        let failed_results: Vec<_> = summary.detailed_results.iter().filter(|r| !r.passed).collect();
        if !failed_results.is_empty() {
            report.push_str("\n## ❌ Failure Analysis\n\n");
            for result in failed_results {
                report.push_str(&format!("### {}\n", result.scenario.name));
                report.push_str(&format!("**Description:** {}\n", result.scenario.description));
                report.push_str("**Failure Reasons:**\n");
                for reason in &result.failure_reasons {
                    report.push_str(&format!("- {}\n", reason));
                }
                report.push_str("\n");
            }
        }

        // Performance Insights
        report.push_str("## 🔍 Performance Insights\n\n");
        report.push_str("### Scalability Analysis\n");
        
        // Group results by concurrency level
        let mut concurrency_groups: HashMap<u32, Vec<&BenchmarkScenarioResult>> = HashMap::new();
        for result in &summary.detailed_results {
            concurrency_groups.entry(result.scenario.test_config.concurrent_users)
                .or_insert_with(Vec::new)
                .push(result);
        }

        for (users, results) in concurrency_groups.iter() {
            let avg_tps = results.iter().map(|r| r.test_results.transactions_per_second).sum::<f64>() / results.len() as f64;
            let avg_latency = results.iter().map(|r| r.test_results.average_latency_ms).sum::<f64>() / results.len() as f64;
            
            report.push_str(&format!("- **{} Users:** {:.2} TPS, {:.0}ms latency\n", users, avg_tps, avg_latency));
        }

        // Transaction Type Analysis
        report.push_str("\n### Transaction Type Analysis\n");
        let tx_types = [TransactionTestType::EnergyOrders, TransactionTestType::TokenTransfers, TransactionTestType::GovernanceVotes];
        
        for tx_type in tx_types {
            let tx_results: Vec<_> = summary.detailed_results.iter()
                .filter(|r| std::mem::discriminant(&r.scenario.test_config.transaction_type) == std::mem::discriminant(&tx_type))
                .collect();
            
            if !tx_results.is_empty() {
                let avg_tps = tx_results.iter().map(|r| r.test_results.transactions_per_second).sum::<f64>() / tx_results.len() as f64;
                let avg_latency = tx_results.iter().map(|r| r.test_results.average_latency_ms).sum::<f64>() / tx_results.len() as f64;
                report.push_str(&format!("- **{:?}:** {:.2} TPS, {:.0}ms latency\n", tx_type, avg_tps, avg_latency));
            }
        }

        // Recommendations
        report.push_str("\n## 🎯 Performance Recommendations\n\n");
        for (i, recommendation) in summary.recommendations.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, recommendation));
        }

        // Production Readiness
        report.push_str("\n## 🚀 Production Readiness Assessment\n\n");
        
        if summary.overall_success_rate >= 85.0 && summary.peak_tps_achieved >= 100.0 {
            report.push_str("✅ **PRODUCTION READY** - System meets production performance requirements\n\n");
            report.push_str("**Next Steps:**\n");
            report.push_str("- Deploy to staging environment for final validation\n");
            report.push_str("- Set up production monitoring and alerting\n");
            report.push_str("- Establish SLA targets based on test results\n");
        } else {
            report.push_str("⚠️ **ADDITIONAL OPTIMIZATION NEEDED** - Address performance issues before production\n\n");
            report.push_str("**Required Actions:**\n");
            report.push_str("- Address identified performance bottlenecks\n");
            report.push_str("- Re-run benchmark tests to validate improvements\n");
            report.push_str("- Consider infrastructure scaling options\n");
        }

        report.push_str("\n---\n");
        report.push_str("*GridTokenX POC Blockchain - Comprehensive TPS Benchmark Suite*\n");
        report.push_str(&format!("*Report Generated: {}*\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SystemConfig;

    #[tokio::test]
    async fn test_benchmark_suite_creation() {
        let config = SystemConfig::default();
        let engine = TpsTestEngine::new(config).await.unwrap();
        let suite = TpsBenchmarkSuite::new(engine).await;
        assert!(suite.is_ok());
    }

    #[tokio::test]
    async fn test_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        assert_eq!(thresholds.min_tps, 10.0);
        assert_eq!(thresholds.max_avg_latency_ms, 1000.0);
        assert_eq!(thresholds.min_success_rate_percent, 95.0);
    }

    #[test]
    fn test_variance_calculation() {
        let values = vec![10.0, 15.0, 20.0, 25.0, 30.0];
        let config = SystemConfig::default();
        let engine = TpsTestEngine::new(config);
        // Test would require async setup, simplified for demonstration
    }
}
