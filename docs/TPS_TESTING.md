# 🔋 GridTokenX POC - Full TPS Testing Suite Documentation

## Overview

The GridTokenX POC Blockchain includes a comprehensive TPS (Transactions Per Second) testing framework designed to benchmark, monitor, and validate the performance characteristics of the energy trading system.

## 🎯 Testing Components

### 1. Shell Script Test Suite (`tps_full_test.sh`)

A comprehensive bash script that provides:
- **Pre-flight system checks** - Validates system components are running
- **Test data generation** - Creates realistic test transaction data
- **Multi-user load testing** - Simulates concurrent users (1, 5, 10, 20, 50, 100)
- **Transaction type testing** - Tests energy orders, token transfers, governance votes
- **Performance monitoring** - Real-time TPS, latency, and resource monitoring
- **Comprehensive reporting** - Detailed markdown reports with insights

#### Usage:
```bash
# Make executable
chmod +x tps_full_test.sh

# Run full test suite
./tps_full_test.sh

# Results will be saved to tps_test_results/ directory
```

#### Key Features:
- **Concurrent User Testing**: 1-100 simultaneous users
- **Transaction Variety**: Energy orders, token transfers, governance votes
- **Real-time Monitoring**: Memory, CPU, connections, blockchain status
- **Failure Analysis**: Success rates, error tracking, bottleneck identification
- **Performance Recommendations**: Optimization suggestions based on results

### 2. Rust TPS Testing Framework

#### Core Components:

**`src/tps_test.rs`** - Core TPS testing engine:
```rust
// Initialize TPS testing
let config = SystemConfig::default();
let engine = TpsTestEngine::new(config).await?;

// Run single test
let test_config = TpsTestConfig {
    test_duration_seconds: 60,
    concurrent_users: 10,
    transaction_type: TransactionTestType::EnergyOrders,
    target_tps: 0,
    max_latency_ms: 1000,
    min_success_rate: 90.0,
};
let results = engine.run_single_test(test_config).await?;
```

**`src/tps_benchmark.rs`** - Advanced benchmarking suite:
```rust
// Run comprehensive benchmark
let engine = TpsTestEngine::new(config).await?;
let mut suite = TpsBenchmarkSuite::new(engine).await?;
let summary = suite.run_comprehensive_benchmark().await?;
```

#### Supported Transaction Types:
- **EnergyOrders**: Buy/sell energy order processing
- **TokenTransfers**: Token movement between accounts  
- **GovernanceVotes**: Decentralized governance voting
- **MixedWorkload**: Realistic combination of all transaction types

### 3. Integration Tests (`tests/tps_integration_tests.rs`)

Automated test suite covering:
- **System Integration**: Component initialization and lifecycle
- **Performance Baselines**: Response time validation
- **System Stability**: Long-running stability verification
- **Concurrent Access**: Multi-threaded access patterns
- **Resource Management**: Memory leak detection

### 4. TPS Examples (`examples/tps_examples.rs`)

Ready-to-run examples demonstrating:
- Basic system status monitoring
- Real-time TPS monitoring
- Production readiness validation
- Performance comparison workflows

## 📊 Performance Metrics

### Primary Metrics:
- **TPS (Transactions Per Second)**: Peak and sustained throughput
- **Latency**: Average, P95, P99 response times
- **Success Rate**: Transaction completion percentage
- **System Resources**: Memory, CPU, connections

### Secondary Metrics:
- **Throughput Scaling**: Performance vs concurrent users
- **Transaction Type Performance**: Relative efficiency by type
- **System Stability**: Consistency over time
- **Resource Efficiency**: Memory and CPU utilization

## 🎯 Test Scenarios

### 1. Baseline Performance (Single User)
- **Purpose**: Establish baseline TPS and latency
- **Configuration**: 1 user, 30 seconds, various transaction types
- **Expected**: 5-50 TPS, <500ms latency, >95% success

### 2. Concurrency Testing (5-100 Users)
- **Purpose**: Identify scalability limits and optimal concurrency
- **Configuration**: Escalating user count, 60-120 seconds
- **Expected**: Peak TPS 100-500, graceful degradation

### 3. Stress Testing (High Load)
- **Purpose**: Determine system breaking points
- **Configuration**: 100+ users, mixed workload, extended duration
- **Expected**: Identify bottlenecks, failure modes, recovery patterns

### 4. Production Readiness
- **Purpose**: Validate production deployment readiness
- **Configuration**: 25 users, 300 seconds, realistic workload
- **Expected**: >50 TPS, <1000ms latency, >95% success

## 🔧 Configuration

### System Configuration
```rust
let mut config = SystemConfig::default();
config.database.max_connections = 20;
config.blockchain.consensus_timeout = 5000;
config.test_mode = false; // Use production-like settings
```

### Test Configuration
```rust
let test_config = TpsTestConfig {
    test_duration_seconds: 120,    // Test duration
    concurrent_users: 25,          // Concurrent load
    transaction_type: TransactionTestType::MixedWorkload,
    target_tps: 100,              // Target throughput (0 = unlimited)
    max_latency_ms: 2000,         // SLA requirement
    min_success_rate: 90.0,       // Acceptance threshold
};
```

## 📋 Running Tests

### Quick Start
```bash
# Basic system test
cargo test tps_integration_tests --release

# Run examples
cargo run --example tps_examples --release

# Full shell script suite
./tps_full_test.sh
```

### Advanced Testing
```bash
# Specific test scenario
cargo test test_performance_baseline --release

# Custom configuration
TPS_DURATION=300 TPS_USERS=50 ./tps_full_test.sh

# Generate detailed reports
cargo run --bin tps_benchmark --release
```

## 📊 Results and Analysis

### Report Generation
Tests automatically generate:
- **JSON Results**: Machine-readable test data
- **Markdown Reports**: Human-readable analysis
- **Performance Charts**: Visual performance trends
- **Recommendations**: Optimization suggestions

### Key Performance Indicators (KPIs)
- **Peak TPS**: Maximum sustained throughput
- **P95 Latency**: 95th percentile response time
- **Success Rate**: Transaction completion percentage
- **Stability Score**: Performance consistency metric

### Example Results
```
📊 GridTokenX POC TPS Test Results:
├─ Peak TPS: 156.3 transactions/second
├─ Average Latency: 245ms
├─ P95 Latency: 890ms
├─ Success Rate: 94.2%
├─ System Stability: 88.5%
└─ Production Ready: ✅ PASS
```

## 🎯 Performance Targets

### Production Requirements:
- **Minimum TPS**: 50 transactions/second
- **Maximum Latency**: 1000ms average
- **Success Rate**: >95%
- **P99 Latency**: <3000ms
- **Memory Usage**: <512MB sustained

### Scalability Goals:
- **Optimal Concurrency**: 20-30 users
- **Peak Capacity**: 100+ users
- **Graceful Degradation**: Maintained service under overload
- **Recovery Time**: <30 seconds after load reduction

## 🔍 Troubleshooting

### Common Issues:
1. **Low TPS**: Check database connection pool, query optimization
2. **High Latency**: Verify network conditions, cache configuration
3. **Memory Growth**: Monitor for memory leaks, implement limits
4. **Connection Errors**: Review database and Redis connectivity

### Optimization Strategies:
1. **Database Tuning**: Connection pooling, query optimization, indexing
2. **Caching**: Redis utilization, hot data caching
3. **Concurrency**: Async processing, batch operations
4. **Resource Management**: Memory limits, connection limits

## 🚀 Production Deployment

### Pre-Deployment Checklist:
- [ ] TPS baseline established (>50 TPS)
- [ ] Latency requirements met (<1000ms avg)
- [ ] Success rate validated (>95%)
- [ ] System stability confirmed (>85%)
- [ ] Resource limits configured
- [ ] Monitoring dashboards ready
- [ ] Alert thresholds set

### Monitoring Setup:
```bash
# Real-time TPS monitoring
./tps_full_test.sh --monitor --duration=3600

# Production health check
cargo run --example tps_examples --release
```

## 📈 Continuous Improvement

### Regular Testing:
- **Daily**: Automated baseline tests
- **Weekly**: Full benchmark suite
- **Monthly**: Comprehensive stress testing
- **Quarterly**: Capacity planning review

### Performance Tracking:
- Track TPS trends over time
- Monitor latency distributions
- Analyze failure patterns
- Optimize bottleneck components

---

## 🔗 Additional Resources

- **System Architecture**: `docs/SYSTEM_DOCUMENT.md`
- **API Documentation**: `docs/API_DOCUMENTATION.md`
- **Deployment Guide**: `docs/DEPLOYMENT.md`
- **Performance Monitoring**: Real-time dashboards and alerting

*GridTokenX POC Blockchain - TPS Testing Framework v1.0*
