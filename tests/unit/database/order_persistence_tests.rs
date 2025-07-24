//! # Order Persistence Tests
//! 
//! Tests for CRUD operations on energy trading orders

use super::*;
use testcontainers::clients::Cli;
use tokio_test;
use chrono::Utc;

#[tokio::test]
async fn test_order_creation() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    fixture.clean_database().await;
    
    let location = create_test_grid_location();
    let account_id = create_test_account_id();
    let order_id = Uuid::new_v4();
    
    // Insert order
    let result = sqlx::query!(
        r#"
        INSERT INTO trading.orders (id, account_id, order_type, energy_amount, price_per_unit, total_price, energy_source, grid_location, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, created_at
        "#,
        order_id,
        account_id,
        "buy",
        100.0,
        50.0,
        5000.0,
        "solar",
        serde_json::to_value(&location).unwrap(),
        "pending"
    )
    .fetch_one(&fixture.pool)
    .await;
    
    assert!(result.is_ok(), "Order creation should succeed");
    
    let row = result.unwrap();
    assert_eq!(row.id, order_id, "Returned ID should match inserted ID");
    assert!(row.created_at.is_some(), "Created timestamp should be set");
}

#[tokio::test]
async fn test_order_retrieval_by_id() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    let test_data = fixture.insert_test_data().await;
    
    // Retrieve order by ID
    let result = sqlx::query!(
        "SELECT * FROM trading.orders WHERE id = $1",
        test_data.buy_order_id
    )
    .fetch_one(&fixture.pool)
    .await;
    
    assert!(result.is_ok(), "Order retrieval should succeed");
    
    let order = result.unwrap();
    assert_eq!(order.id, test_data.buy_order_id, "ID should match");
    assert_eq!(order.account_id, test_data.account_id, "Account ID should match");
    assert_eq!(order.order_type, "buy", "Order type should match");
    assert_eq!(order.energy_amount, Some(100.0), "Energy amount should match");
}

#[tokio::test]
async fn test_order_retrieval_by_account() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    let test_data = fixture.insert_test_data().await;
    
    // Retrieve all orders for account
    let result = sqlx::query!(
        "SELECT * FROM trading.orders WHERE account_id = $1 ORDER BY created_at",
        test_data.account_id
    )
    .fetch_all(&fixture.pool)
    .await;
    
    assert!(result.is_ok(), "Account orders retrieval should succeed");
    
    let orders = result.unwrap();
    assert_eq!(orders.len(), 2, "Should have both buy and sell orders");
    
    // Verify order types
    assert_eq!(orders[0].order_type, "buy", "First order should be buy");
    assert_eq!(orders[1].order_type, "sell", "Second order should be sell");
}

#[tokio::test]
async fn test_order_update() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    let test_data = fixture.insert_test_data().await;
    
    let new_status = "filled";
    let update_time = Utc::now();
    
    // Update order status
    let result = sqlx::query!(
        r#"
        UPDATE trading.orders 
        SET status = $1, updated_at = $2
        WHERE id = $3
        RETURNING updated_at
        "#,
        new_status,
        update_time,
        test_data.buy_order_id
    )
    .fetch_one(&fixture.pool)
    .await;
    
    assert!(result.is_ok(), "Order update should succeed");
    
    // Verify update
    let updated_order = sqlx::query!(
        "SELECT status, updated_at FROM trading.orders WHERE id = $1",
        test_data.buy_order_id
    )
    .fetch_one(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(updated_order.status, new_status, "Status should be updated");
    assert!(updated_order.updated_at.is_some(), "Updated timestamp should be set");
}

#[tokio::test]
async fn test_order_partial_fill_update() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    let test_data = fixture.insert_test_data().await;
    
    let filled_amount = 30.0;
    let remaining_amount = 70.0;
    
    // Simulate partial fill by updating energy amount
    let result = sqlx::query!(
        r#"
        UPDATE trading.orders 
        SET energy_amount = $1, status = $2, updated_at = NOW()
        WHERE id = $3
        RETURNING energy_amount, status
        "#,
        remaining_amount,
        "partially_filled",
        test_data.buy_order_id
    )
    .fetch_one(&fixture.pool)
    .await;
    
    assert!(result.is_ok(), "Partial fill update should succeed");
    
    let updated_order = result.unwrap();
    assert_eq!(updated_order.energy_amount, Some(remaining_amount), "Remaining amount should be updated");
    assert_eq!(updated_order.status, "partially_filled", "Status should reflect partial fill");
}

#[tokio::test]
async fn test_order_deletion() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    let test_data = fixture.insert_test_data().await;
    
    // Delete order
    let result = sqlx::query!(
        "DELETE FROM trading.orders WHERE id = $1",
        test_data.buy_order_id
    )
    .execute(&fixture.pool)
    .await;
    
    assert!(result.is_ok(), "Order deletion should succeed");
    assert_eq!(result.unwrap().rows_affected(), 1, "Exactly one row should be deleted");
    
    // Verify deletion
    let check_result = sqlx::query!(
        "SELECT id FROM trading.orders WHERE id = $1",
        test_data.buy_order_id
    )
    .fetch_optional(&fixture.pool)
    .await
    .unwrap();
    
    assert!(check_result.is_none(), "Deleted order should not exist");
}

#[tokio::test]
async fn test_order_expiry_handling() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    fixture.clean_database().await;
    
    let location = create_test_grid_location();
    let account_id = create_test_account_id();
    let expires_at = Utc::now() + chrono::Duration::hours(1);
    
    // Insert order with expiry
    let order_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO trading.orders (id, account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        order_id,
        account_id,
        "buy",
        100.0,
        50.0,
        5000.0,
        serde_json::to_value(&location).unwrap(),
        "pending",
        expires_at
    )
    .execute(&fixture.pool)
    .await
    .unwrap();
    
    // Query for expired orders (simulating future time)
    let expired_orders = sqlx::query!(
        "SELECT id FROM trading.orders WHERE expires_at < $1 AND status = 'pending'",
        Utc::now() + chrono::Duration::hours(2)
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(expired_orders.len(), 1, "Should find one expired order");
    assert_eq!(expired_orders[0].id, order_id, "Expired order ID should match");
}

#[tokio::test]
async fn test_order_search_by_criteria() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    fixture.clean_database().await;
    
    let location = create_test_grid_location();
    let account1 = "account_1".to_string();
    let account2 = "account_2".to_string();
    
    // Insert multiple orders with different criteria
    let orders_data = vec![
        (account1.clone(), "buy", 100.0, 50.0, "solar", "pending"),
        (account1.clone(), "sell", 80.0, 52.0, "wind", "filled"),
        (account2.clone(), "buy", 120.0, 48.0, "solar", "pending"),
        (account2.clone(), "sell", 90.0, 55.0, "hydro", "cancelled"),
    ];
    
    for (account, order_type, amount, price, energy_source, status) in orders_data {
        sqlx::query!(
            r#"
            INSERT INTO trading.orders (account_id, order_type, energy_amount, price_per_unit, total_price, energy_source, grid_location, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            account,
            order_type,
            amount,
            price,
            amount * price,
            energy_source,
            serde_json::to_value(&location).unwrap(),
            status
        )
        .execute(&fixture.pool)
        .await
        .unwrap();
    }
    
    // Search by order type
    let buy_orders = sqlx::query!(
        "SELECT * FROM trading.orders WHERE order_type = 'buy'"
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(buy_orders.len(), 2, "Should find 2 buy orders");
    
    // Search by status
    let pending_orders = sqlx::query!(
        "SELECT * FROM trading.orders WHERE status = 'pending'"
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(pending_orders.len(), 2, "Should find 2 pending orders");
    
    // Search by energy source
    let solar_orders = sqlx::query!(
        "SELECT * FROM trading.orders WHERE energy_source = 'solar'"
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(solar_orders.len(), 2, "Should find 2 solar orders");
    
    // Complex search with multiple criteria
    let complex_search = sqlx::query!(
        r#"
        SELECT * FROM trading.orders 
        WHERE account_id = $1 AND order_type = 'buy' AND status = 'pending'
        "#,
        account1
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(complex_search.len(), 1, "Complex search should find 1 order");
}

#[tokio::test]
async fn test_order_pagination() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    fixture.clean_database().await;
    
    // Generate test data
    DatabaseTestUtils::generate_load_data(&fixture.pool, 25).await;
    
    // Test pagination
    let page_size = 10;
    let page1 = sqlx::query!(
        r#"
        SELECT * FROM trading.orders 
        ORDER BY created_at 
        LIMIT $1 OFFSET $2
        "#,
        page_size,
        0
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    let page2 = sqlx::query!(
        r#"
        SELECT * FROM trading.orders 
        ORDER BY created_at 
        LIMIT $1 OFFSET $2
        "#,
        page_size,
        page_size
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(page1.len(), page_size as usize, "First page should have full page size");
    assert_eq!(page2.len(), page_size as usize, "Second page should have full page size");
    
    // Verify no overlap
    let page1_ids: Vec<_> = page1.iter().map(|o| o.id).collect();
    let page2_ids: Vec<_> = page2.iter().map(|o| o.id).collect();
    
    for id in &page1_ids {
        assert!(!page2_ids.contains(id), "Pages should not overlap");
    }
}

#[tokio::test]
async fn test_order_bulk_operations() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    fixture.clean_database().await;
    
    let location = create_test_grid_location();
    let location_json = serde_json::to_value(&location).unwrap();
    let account_id = create_test_account_id();
    
    // Bulk insert orders
    let mut order_ids = vec![];
    for i in 0..5 {
        let order_id = Uuid::new_v4();
        order_ids.push(order_id);
        
        sqlx::query!(
            r#"
            INSERT INTO trading.orders (id, account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            order_id,
            account_id,
            "buy",
            100.0 + i as f64,
            50.0 + i as f64,
            (100.0 + i as f64) * (50.0 + i as f64),
            location_json,
            "pending"
        )
        .execute(&fixture.pool)
        .await
        .unwrap();
    }
    
    // Bulk status update
    let updated_count = sqlx::query!(
        r#"
        UPDATE trading.orders 
        SET status = 'cancelled', updated_at = NOW()
        WHERE id = ANY($1)
        "#,
        &order_ids
    )
    .execute(&fixture.pool)
    .await
    .unwrap()
    .rows_affected();
    
    assert_eq!(updated_count, 5, "All 5 orders should be updated");
    
    // Verify bulk update
    let cancelled_orders = sqlx::query!(
        "SELECT id FROM trading.orders WHERE status = 'cancelled'"
    )
    .fetch_all(&fixture.pool)
    .await
    .unwrap();
    
    assert_eq!(cancelled_orders.len(), 5, "All orders should be cancelled");
}

#[tokio::test]
async fn test_order_data_integrity() {
    let docker = Cli::default();
    let fixture = DatabaseTestFixture::new(&docker).await;
    fixture.clean_database().await;
    
    let location = create_test_grid_location();
    
    // Test constraint violations
    
    // Test NOT NULL constraint
    let null_account_result = sqlx::query!(
        r#"
        INSERT INTO trading.orders (account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
        VALUES (NULL, 'buy', 100.0, 50.0, 5000.0, $1, 'pending')
        "#,
        serde_json::to_value(&location).unwrap()
    )
    .execute(&fixture.pool)
    .await;
    
    assert!(null_account_result.is_err(), "NULL account_id should be rejected");
    
    // Test CHECK constraint on order_type
    let invalid_type_result = sqlx::query!(
        r#"
        INSERT INTO trading.orders (account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
        VALUES ($1, 'invalid', 100.0, 50.0, 5000.0, $2, 'pending')
        "#,
        create_test_account_id(),
        serde_json::to_value(&location).unwrap()
    )
    .execute(&fixture.pool)
    .await;
    
    assert!(invalid_type_result.is_err(), "Invalid order type should be rejected");
    
    // Test valid data insertion
    let valid_result = sqlx::query!(
        r#"
        INSERT INTO trading.orders (account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
        VALUES ($1, 'buy', 100.0, 50.0, 5000.0, $2, 'pending')
        "#,
        create_test_account_id(),
        serde_json::to_value(&location).unwrap()
    )
    .execute(&fixture.pool)
    .await;
    
    assert!(valid_result.is_ok(), "Valid order should be inserted successfully");
}
