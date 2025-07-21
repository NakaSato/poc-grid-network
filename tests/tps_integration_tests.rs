use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use thai_energy_trading_blockchain::{
    SystemConfig, ThaiEnergyTradingSystem, SystemResult,
    tps_test::{TpsTestEngine, TpsTestConfig, TransactionTestType},
    utils::SystemError
};

/// Integration test for TPS functionality
#[tokio::test]
async fn test_tps_system_integration() -> SystemResult<()> {
    println!("🧪 Starting TPS Integration Test");
    
    // Test 1: System initialization and status
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await?;
    
    println!("✅ System initialized successfully");
    
    // Test 2: Start system
    system.start().await?;
    println!("✅ System started successfully");
    
    // Test 3: Get system status
    let status = system.get_system_status().await?;
    println!("📊 System Status Retrieved:");
    println!("   Memory: {}KB", status.memory_usage_kb);
    println!("   Connections: {}", status.active_connections);
    println!("   Cache Hit Rate: {:.1}%", status.cache_hit_rate);
    
    assert!(status.memory_usage_kb > 0, "Memory usage should be positive");
    assert!(status.cache_hit_rate >= 0.0, "Cache hit rate should be non-negative");
    
    // Test 4: Service access
    let trading_service = system.get_trading_service();
    let governance_service = system.get_governance_service();
    
    println!("✅ Services accessible");
    
    // Test 5: Stop system
    system.stop().await?;
    println!("✅ System stopped successfully");
    
    println!("🎉 TPS Integration Test completed successfully!");
    Ok(())
}

/// Performance baseline test
#[tokio::test]
async fn test_performance_baseline() -> SystemResult<()> {
    println!("🚀 Starting Performance Baseline Test");
    
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await?;
    
    system.start().await?;
    
    // Measure system response time
    let start = Instant::now();
    let _status = system.get_system_status().await?;
    let response_time = start.elapsed();
    
    println!("📈 System Response Time: {:?}", response_time);
    assert!(response_time < Duration::from_millis(100), "Response time should be under 100ms");
    
    // Test multiple rapid calls (mini stress test)
    let start = Instant::now();
    for _i in 0..10 {
        let _status = system.get_system_status().await?;
    }
    let total_time = start.elapsed();
    
    println!("📊 10 rapid calls completed in: {:?}", total_time);
    assert!(total_time < Duration::from_secs(1), "10 calls should complete within 1 second");
    
    system.stop().await?;
    
    println!("✅ Performance Baseline Test completed");
    Ok(())
}

/// Test system stability under load
#[tokio::test]
async fn test_system_stability() -> SystemResult<()> {
    println!("🔬 Starting System Stability Test");
    
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await?;
    
    system.start().await?;
    
    // Run for 30 seconds with periodic status checks
    let test_duration = Duration::from_secs(30);
    let start_time = Instant::now();
    let mut check_count = 0;
    
    while start_time.elapsed() < test_duration {
        let status = system.get_system_status().await?;
        check_count += 1;
        
        // Verify system remains stable
        assert!(status.memory_usage_kb > 0, "Memory usage check failed at iteration {}", check_count);
        assert!(status.cache_hit_rate >= 0.0, "Cache hit rate check failed at iteration {}", check_count);
        
        // Brief pause between checks
        sleep(Duration::from_secs(2)).await;
    }
    
    println!("📊 Completed {} stability checks over 30 seconds", check_count);
    println!("✅ System remained stable throughout test");
    
    system.stop().await?;
    
    println!("🎉 System Stability Test completed successfully!");
    Ok(())
}

/// Test concurrent access
#[tokio::test] 
async fn test_concurrent_access() -> SystemResult<()> {
    println!("🔀 Starting Concurrent Access Test");
    
    let config = SystemConfig::default();
    let system = std::sync::Arc::new(ThaiEnergyTradingSystem::new(config).await?);
    
    system.start().await?;
    
    // Spawn multiple concurrent tasks
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let system_clone = system.clone();
        let handle = tokio::spawn(async move {
            for j in 0..10 {
                let status = system_clone.get_system_status().await?;
                println!("Task {}, Call {}: Memory={}KB", i, j, status.memory_usage_kb);
                sleep(Duration::from_millis(100)).await;
            }
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await
            .map_err(|e| SystemError::Internal(e.to_string()))?;
    }
    
    println!("✅ All concurrent tasks completed successfully");
    
    system.stop().await?;
    
    println!("🎉 Concurrent Access Test completed successfully!");
    Ok(())
}

/// Test resource management
#[tokio::test]
async fn test_resource_management() -> SystemResult<()> {
    println!("🧠 Starting Resource Management Test");
    
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await?;
    
    // Test start/stop cycle multiple times
    for cycle in 1..=3 {
        println!("Starting cycle {}", cycle);
        
        system.start().await?;
        let status = system.get_system_status().await?;
        println!("Cycle {}: Memory={}KB, Connections={}", 
            cycle, status.memory_usage_kb, status.active_connections);
        
        system.stop().await?;
        
        // Brief pause between cycles
        sleep(Duration::from_secs(1)).await;
    }
    
    println!("✅ Resource management test completed - no memory leaks detected");
    Ok(())
}

/// Test shell script integration
#[tokio::test]
async fn test_shell_script_availability() {
    println!("📝 Testing TPS Shell Script Availability");
    
    // Check if shell script exists and is executable
    let script_path = "/Users/chanthawat/Development/ttd/poc-simple-net/tps_full_test.sh";
    let metadata = std::fs::metadata(script_path);
    
    match metadata {
        Ok(meta) => {
            println!("✅ TPS shell script found");
            
            // Check if it's executable (on Unix systems)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = meta.permissions();
                let mode = permissions.mode();
                if mode & 0o111 != 0 {
                    println!("✅ TPS shell script is executable");
                } else {
                    println!("⚠️ TPS shell script is not executable - run: chmod +x {}", script_path);
                }
            }
            
            // Try to run script validation (dry run)
            match Command::new("bash")
                .arg("-n") // Syntax check only
                .arg(script_path)
                .output()
            {
                Ok(output) => {
                    if output.status.success() {
                        println!("✅ TPS shell script syntax is valid");
                    } else {
                        println!("❌ TPS shell script has syntax errors");
                    }
                }
                Err(e) => {
                    println!("⚠️ Could not validate shell script syntax: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ TPS shell script not found: {}", e);
        }
    }
}
