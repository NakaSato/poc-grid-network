//! # Database Connection Tests
//! 
//! Tests for database connectivity, pooling, and configuration

use super::*;
use testcontainers::clients::Cli;
use tokio_test;
use std::time::Duration;

#[tokio::test]
async fn test_database_connection_establishment() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Test basic connectivity
    let result = sqlx::query("SELECT 1 as test")
        .fetch_one(&fixture.pool)
        .await;
    
    assert!(result.is_ok(), "Database connection should be established");
    
    let row = result.unwrap();
    let test_value: i32 = row.get("test");
    assert_eq!(test_value, 1, "Query should return expected value");
}

#[tokio::test]
async fn test_database_connection_pooling() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Test multiple concurrent connections
    let mut handles = vec![];
    
    for i in 0..10 {
        let pool = fixture.pool.clone();
        let handle = tokio::spawn(async move {
            sqlx::query("SELECT $1 as id")
                .bind(i)
                .fetch_one(&pool)
                .await
        });
        handles.push(handle);
    }
    
    // Wait for all connections to complete
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok(), "Concurrent connections should succeed");
    }
}

#[tokio::test]
async fn test_database_connection_recovery() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Simulate connection issues by exhausting the pool
    let pool_size = 5; // Typical pool size
    let mut connections = vec![];
    
    // Acquire all connections
    for _ in 0..pool_size {
        let conn = fixture.pool.acquire().await.unwrap();
        connections.push(conn);
    }
    
    // Try to acquire another connection (should timeout or fail gracefully)
    let timeout_result = tokio::time::timeout(
        Duration::from_millis(100),
        fixture.pool.acquire()
    ).await;
    
    assert!(timeout_result.is_err(), "Should timeout when pool is exhausted");
    
    // Release connections
    drop(connections);
    
    // Verify recovery
    let recovery_result = fixture.pool.acquire().await;
    assert!(recovery_result.is_ok(), "Should recover after connections are released");
}

#[tokio::test]
async fn test_database_schema_validation() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Test that all required tables exist
    let required_tables = vec![
        "blockchain.blocks",
        "blockchain.transactions", 
        "trading.orders",
        "trading.trades",
        "grid.locations",
    ];
    
    for table in required_tables {
        let result = sqlx::query(&format!(
            "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_schema = $1 AND table_name = $2)"
        ))
        .bind(table.split('.').next().unwrap())
        .bind(table.split('.').nth(1).unwrap())
        .fetch_one(&fixture.pool)
        .await;
        
        assert!(result.is_ok(), "Should be able to query table existence");
        let exists: bool = result.unwrap().get(0);
        assert!(exists, "Required table {} should exist", table);
    }
}

#[tokio::test]
async fn test_database_indexes_exist() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Check for critical indexes
    let index_queries = vec![
        ("trading.orders", "account_id"),
        ("trading.orders", "status"),
        ("trading.orders", "created_at"),
        ("trading.trades", "executed_at"),
        ("blockchain.blocks", "block_number"),
        ("blockchain.transactions", "hash"),
    ];
    
    for (table, column) in index_queries {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as index_count
            FROM pg_indexes 
            WHERE schemaname = $1 AND tablename = $2 AND indexdef LIKE '%' || $3 || '%'
            "#,
            table.split('.').next().unwrap(),
            table.split('.').nth(1).unwrap(),
            column
        )
        .fetch_one(&fixture.pool)
        .await;
        
        assert!(result.is_ok(), "Should be able to query indexes");
        let count = result.unwrap().index_count.unwrap_or(0);
        assert!(count > 0, "Index on {}.{} should exist", table, column);
    }
}

#[tokio::test]
async fn test_database_constraints() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Test foreign key constraints
    let test_data = fixture.insert_test_data().await;
    
    // Try to insert trade with invalid order ID (should fail)
    let invalid_trade_result = sqlx::query!(
        r#"
        INSERT INTO trading.trades (buy_order_id, sell_order_id, buyer_id, seller_id, energy_amount, price_per_unit, total_price, grid_fee, grid_location)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        Uuid::new_v4(), // Invalid order ID
        test_data.sell_order_id,
        test_data.account_id,
        test_data.account_id,
        50.0,
        51.0,
        2550.0,
        25.5,
        serde_json::to_value(&test_data.location).unwrap()
    )
    .execute(&fixture.pool)
    .await;
    
    assert!(invalid_trade_result.is_err(), "Foreign key constraint should prevent invalid trade");
}

#[tokio::test]
async fn test_database_transaction_rollback() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    fixture.clean_database().await;
    
    // Start transaction
    let mut tx = fixture.pool.begin().await.unwrap();
    
    // Insert test order
    let order_id = Uuid::new_v4();
    let location = create_test_grid_location();
    
    sqlx::query!(
        r#"
        INSERT INTO trading.orders (id, account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
        VALUES ($1, $2, 'buy', $3, $4, $5, $6, 'pending')
        "#,
        order_id,
        create_test_account_id(),
        100.0,
        50.0,
        5000.0,
        serde_json::to_value(&location).unwrap()
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    
    // Rollback transaction
    tx.rollback().await.unwrap();
    
    // Verify order was not persisted
    let count = DatabaseTestUtils::count_rows(&fixture.pool, "trading.orders").await;
    assert_eq!(count, 0, "Rolled back transaction should not persist data");
}

#[tokio::test]
async fn test_database_connection_timeout() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    // Test query timeout
    let timeout_result = tokio::time::timeout(
        Duration::from_millis(100),
        sqlx::query("SELECT pg_sleep(1)")
            .execute(&fixture.pool)
    ).await;
    
    assert!(timeout_result.is_err(), "Long queries should timeout");
}

#[tokio::test]
async fn test_database_concurrent_access() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    fixture.clean_database().await;
    let location = create_test_grid_location();
    let location_json = serde_json::to_value(&location).unwrap();
    
    // Start multiple concurrent transactions
    let mut handles = vec![];
    
    for i in 0..5 {
        let pool = fixture.pool.clone();
        let location_json = location_json.clone();
        
        let handle = tokio::spawn(async move {
            let mut tx = pool.begin().await.unwrap();
            
            // Insert order
            sqlx::query!(
                r#"
                INSERT INTO trading.orders (account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
                VALUES ($1, 'buy', $2, $3, $4, $5, 'pending')
                "#,
                format!("account_{}", i),
                100.0 + i as f64,
                50.0 + i as f64,
                (100.0 + i as f64) * (50.0 + i as f64),
                location_json
            )
            .execute(&mut *tx)
            .await
            .unwrap();
            
            tx.commit().await.unwrap();
        });
        
        handles.push(handle);
    }
    
    // Wait for all transactions
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Verify all orders were inserted
    let count = DatabaseTestUtils::count_rows(&fixture.pool, "trading.orders").await;
    assert_eq!(count, 5, "All concurrent transactions should complete");
}

#[tokio::test]
async fn test_database_deadlock_detection() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    
    let test_data = fixture.insert_test_data().await;
    
    // Create potential deadlock scenario
    let pool1 = fixture.pool.clone();
    let pool2 = fixture.pool.clone();
    let order1_id = test_data.buy_order_id;
    let order2_id = test_data.sell_order_id;
    
    let handle1 = tokio::spawn(async move {
        let mut tx = pool1.begin().await.unwrap();
        
        // Lock order1 first
        sqlx::query!(
            "SELECT * FROM trading.orders WHERE id = $1 FOR UPDATE",
            order1_id
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Try to lock order2
        let result = sqlx::query!(
            "SELECT * FROM trading.orders WHERE id = $1 FOR UPDATE",
            order2_id
        )
        .fetch_one(&mut *tx)
        .await;
        
        tx.commit().await.unwrap();
        result
    });
    
    let handle2 = tokio::spawn(async move {
        let mut tx = pool2.begin().await.unwrap();
        
        // Lock order2 first  
        sqlx::query!(
            "SELECT * FROM trading.orders WHERE id = $1 FOR UPDATE",
            order2_id
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Try to lock order1
        let result = sqlx::query!(
            "SELECT * FROM trading.orders WHERE id = $1 FOR UPDATE", 
            order1_id
        )
        .fetch_one(&mut *tx)
        .await;
        
        tx.commit().await.unwrap();
        result
    });
    
    // At least one should complete (deadlock detection should resolve it)
    let results = tokio::join!(handle1, handle2);
    assert!(
        results.0.is_ok() || results.1.is_ok(),
        "At least one transaction should complete despite potential deadlock"
    );
}
