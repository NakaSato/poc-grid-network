//! # Database Manager
//! 
//! Implements database connectivity and management for PostgreSQL and Redis.

use crate::config::DatabaseConfig;
use crate::utils::SystemResult;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Database manager for PostgreSQL and Redis
pub struct DatabaseManager {
    config: DatabaseConfig,
    postgres_pool: Option<PgPool>,
    redis_connection: Option<Arc<RwLock<redis::Connection>>>,
    running: Arc<RwLock<bool>>,
    test_mode: bool,
}

impl DatabaseManager {
    pub async fn new(config: &DatabaseConfig, test_mode: bool) -> SystemResult<Self> {
        Ok(Self {
            config: config.clone(),
            postgres_pool: None,
            redis_connection: None,
            running: Arc::new(RwLock::new(false)),
            test_mode,
        })
    }
    
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        
        crate::utils::logging::log_startup("Database Manager");
        
        if self.test_mode {
            // Skip database initialization in test mode
            return Ok(());
        }
        
        // Initialize PostgreSQL connection
        self.initialize_postgres().await?;
        
        // Initialize Redis connection
        self.initialize_redis().await?;
        
        Ok(())
    }
    
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        
        crate::utils::logging::log_shutdown("Database Manager");
        
        Ok(())
    }
    
    /// Get PostgreSQL connection pool
    pub fn get_postgres_pool(&self) -> Option<&PgPool> {
        self.postgres_pool.as_ref()
    }
    
    /// Get Redis connection
    pub fn get_redis_connection(&self) -> Option<Arc<RwLock<redis::Connection>>> {
        self.redis_connection.clone()
    }
    
    /// Initialize PostgreSQL connection
    async fn initialize_postgres(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("DatabaseManager", "Initializing PostgreSQL connection");
        
        // Create connection pool
        let pool = PgPool::connect(&self.config.url).await
            .map_err(|e| crate::utils::SystemError::Database(e))?;
        
        // Run migrations
        self.run_migrations(&pool).await?;
        
        // Store pool (this is a simplified version - in real implementation we'd use proper state management)
        crate::utils::logging::log_info("DatabaseManager", "PostgreSQL connection initialized");
        
        Ok(())
    }
    
    /// Initialize Redis connection
    async fn initialize_redis(&self) -> SystemResult<()> {
        crate::utils::logging::log_info("DatabaseManager", "Initializing Redis connection");
        
        let _client = redis::Client::open(self.config.redis_url.clone())
            .map_err(|e| crate::utils::SystemError::Database(sqlx::Error::Configuration(e.into())))?;
        
        // Store connection (this is a simplified version)
        crate::utils::logging::log_info("DatabaseManager", "Redis connection initialized");
        
        Ok(())
    }
    
    /// Run database migrations
    async fn run_migrations(&self, pool: &PgPool) -> SystemResult<()> {
        crate::utils::logging::log_info("DatabaseManager", "Running database migrations");
        
        // Example migration queries
        let migration_queries = vec![
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                account_id VARCHAR(64) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                name VARCHAR(255) NOT NULL,
                user_type VARCHAR(50) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS energy_orders (
                id UUID PRIMARY KEY,
                account_id VARCHAR(64) NOT NULL,
                order_type VARCHAR(10) NOT NULL,
                energy_amount BIGINT NOT NULL,
                price_per_kwh BIGINT NOT NULL,
                total_price BIGINT NOT NULL,
                energy_source VARCHAR(50),
                grid_location JSONB NOT NULL,
                delivery_time TIMESTAMP WITH TIME ZONE NOT NULL,
                expiry_time TIMESTAMP WITH TIME ZONE NOT NULL,
                status VARCHAR(20) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS energy_trades (
                id UUID PRIMARY KEY,
                buy_order_id UUID NOT NULL,
                sell_order_id UUID NOT NULL,
                buyer_id VARCHAR(64) NOT NULL,
                seller_id VARCHAR(64) NOT NULL,
                energy_amount BIGINT NOT NULL,
                price_per_kwh BIGINT NOT NULL,
                total_price BIGINT NOT NULL,
                grid_fee BIGINT NOT NULL,
                energy_source VARCHAR(50) NOT NULL,
                delivery_time TIMESTAMP WITH TIME ZONE NOT NULL,
                settlement_status VARCHAR(20) NOT NULL,
                executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS governance_proposals (
                id UUID PRIMARY KEY,
                proposer VARCHAR(64) NOT NULL,
                title VARCHAR(255) NOT NULL,
                description TEXT NOT NULL,
                proposal_type VARCHAR(50) NOT NULL,
                voting_period JSONB NOT NULL,
                required_threshold JSONB NOT NULL,
                current_votes JSONB NOT NULL,
                status VARCHAR(20) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                voting_ends_at TIMESTAMP WITH TIME ZONE NOT NULL
            )
            "#,
            r#"
            CREATE TABLE IF NOT EXISTS grid_status (
                id SERIAL PRIMARY KEY,
                location JSONB NOT NULL,
                capacity BIGINT NOT NULL,
                current_load BIGINT NOT NULL,
                congestion_level VARCHAR(20) NOT NULL,
                stability_score REAL NOT NULL,
                outage_risk REAL NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        ];
        
        for query in migration_queries {
            sqlx::query(query).execute(pool).await
                .map_err(|e| crate::utils::SystemError::Database(e))?;
        }
        
        crate::utils::logging::log_info("DatabaseManager", "Database migrations completed");
        
        Ok(())
    }
}
