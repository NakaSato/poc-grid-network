//! # Database Unit Tests Module
//! 
//! Comprehensive database testing for the Thai Energy Trading system

pub mod connection_tests;
pub mod migration_tests;
pub mod order_persistence_tests;
pub mod trade_persistence_tests;

// Database test utilities  
use thai_energy_trading_blockchain::types::*;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Database test fixture with PostgreSQL testcontainer
pub struct DatabaseTestFixture<'a> {
    pub pool: PgPool,
    pub _container: Container<'a, Postgres>,
}

impl<'a> DatabaseTestFixture<'a> {
    /// Create a new database test fixture with isolated test database
    pub async fn new(docker: &'a Docker) -> Self {
        // Start PostgreSQL container
        let postgres_image = Postgres::default()
            .with_db_name("test_db")
            .with_user("test_user")
            .with_password("test_pass");
        
        let container = docker.run(postgres_image);
        let connection_string = format!(
            "postgres://test_user:test_pass@127.0.0.1:{}/test_db",
            container.get_host_port_ipv4(5432)
        );
        
        // Create connection pool
        let pool = PgPool::connect(&connection_string)
            .await
            .expect("Failed to connect to test database");
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        
        Self {
            pool,
            _container: container,
        }
    }
    
    /// Clean all tables for test isolation
    pub async fn clean_database(&self) {
        let tables = vec![
            "trading.trades",
            "trading.orders", 
            "grid.locations",
            "blockchain.transactions",
            "blockchain.blocks",
        ];
        
        for table in tables {
            sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table))
                .execute(&self.pool)
                .await
                .expect("Failed to truncate table");
        }
    }
    
    /// Insert test data for testing
    pub async fn insert_test_data(&self) -> TestDataSet {
        let location = create_test_grid_location();
        let account_id = create_test_account_id();
        
        // Insert test grid location
        sqlx::query!(
            r#"
            INSERT INTO grid.locations (province, district, grid_code, substation, coordinates, capacity_mw, current_load_mw)
            VALUES ($1, $2, $3, $4, POINT($5, $6), $7, $8)
            "#,
            location.province,
            location.district,
            "TEST001",
            "Test Substation",
            location.coordinates.longitude,
            location.coordinates.latitude,
            100.0,
            50.0
        )
        .execute(&self.pool)
        .await
        .expect("Failed to insert test location");
        
        // Insert test orders
        let buy_order_id = Uuid::new_v4();
        let sell_order_id = Uuid::new_v4();
        
        sqlx::query!(
            r#"
            INSERT INTO trading.orders (id, account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
            VALUES ($1, $2, 'buy', $3, $4, $5, $6, 'pending')
            "#,
            buy_order_id,
            account_id,
            100.0,
            50.0,
            5000.0,
            serde_json::to_value(&location).unwrap()
        )
        .execute(&self.pool)
        .await
        .expect("Failed to insert buy order");
        
        sqlx::query!(
            r#"
            INSERT INTO trading.orders (id, account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
            VALUES ($1, $2, 'sell', $3, $4, $5, $6, 'pending')
            "#,
            sell_order_id,
            account_id,
            80.0,
            52.0,
            4160.0,
            serde_json::to_value(&location).unwrap()
        )
        .execute(&self.pool)
        .await
        .expect("Failed to insert sell order");
        
        TestDataSet {
            location,
            account_id,
            buy_order_id,
            sell_order_id,
        }
    }
}

/// Test data set for consistent testing
pub struct TestDataSet {
    pub location: GridLocation,
    pub account_id: AccountId,
    pub buy_order_id: Uuid,
    pub sell_order_id: Uuid,
}

/// Database performance metrics
#[derive(Debug)]
pub struct PerformanceMetrics {
    pub query_time_ms: u64,
    pub rows_affected: u64,
    pub connection_time_ms: u64,
}

/// Database test utilities
pub struct DatabaseTestUtils;

impl DatabaseTestUtils {
    /// Count rows in a table
    pub async fn count_rows(pool: &PgPool, table: &str) -> i64 {
        sqlx::query(&format!("SELECT COUNT(*) as count FROM {}", table))
            .fetch_one(pool)
            .await
            .unwrap()
            .get("count")
    }
    
    /// Measure query execution time
    pub async fn measure_query_time<F, Fut, T>(operation: F) -> (T, u64)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let start = std::time::Instant::now();
        let result = operation().await;
        let duration = start.elapsed().as_millis() as u64;
        (result, duration)
    }
    
    /// Check database constraints
    pub async fn verify_constraints(pool: &PgPool) -> bool {
        // Check foreign key constraints
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as violations FROM (
                SELECT 1 FROM trading.trades t 
                LEFT JOIN trading.orders bo ON t.buy_order_id = bo.id 
                LEFT JOIN trading.orders so ON t.sell_order_id = so.id
                WHERE bo.id IS NULL OR so.id IS NULL
            ) violations
            "#
        )
        .fetch_one(pool)
        .await;
        
        match result {
            Ok(row) => row.violations.unwrap_or(0) == 0,
            Err(_) => false,
        }
    }
    
    /// Generate test load data
    pub async fn generate_load_data(pool: &PgPool, order_count: i32) {
        let location = create_test_grid_location();
        let location_json = serde_json::to_value(&location).unwrap();
        
        for i in 0..order_count {
            let order_type = if i % 2 == 0 { "buy" } else { "sell" };
            let price = 50.0 + (i as f64) * 0.1;
            let amount = 100.0 + (i as f64) * 10.0;
            
            sqlx::query!(
                r#"
                INSERT INTO trading.orders (account_id, order_type, energy_amount, price_per_unit, total_price, grid_location, status)
                VALUES ($1, $2, $3, $4, $5, $6, 'pending')
                "#,
                create_test_account_id(),
                order_type,
                amount,
                price,
                amount * price,
                location_json
            )
            .execute(pool)
            .await
            .expect("Failed to insert load test order");
        }
    }
}
