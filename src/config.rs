//! # Configuration Module for Thai Energy Trading Blockchain
//! 
//! This module handles all system configuration including database, blockchain,
//! grid integration, security, trading, governance, and oracle settings.

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::env;

/// Main system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub test_mode: bool,
    pub database: DatabaseConfig,
    pub blockchain: BlockchainConfig,
    pub grid: GridConfig,
    pub security: SecurityConfig,
    pub trading: TradingConfig,
    pub governance: GovernanceConfig,
    pub oracle: OracleConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub redis_url: String,
    pub redis_max_connections: u32,
}

/// Blockchain configuration (Proof-of-Authority only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    pub network: String,
    pub node_key: String,
    pub bootnodes: Vec<String>,
    pub validator: bool,
    pub rpc_port: u16,
    pub ws_port: u16,
    pub p2p_port: u16,
    pub chain_spec: String,
    /// Consensus algorithm (always PoA)
    pub consensus_algorithm: String,
    /// Block production time in seconds
    pub block_time: u64,
    /// Validator rotation interval in blocks
    pub validator_rotation_interval: u64,
    /// Maximum number of validators
    pub max_validators: u32,
    /// Authority threshold for consensus
    pub authority_threshold: u32,
    /// Gas limit for transactions
    pub gas_limit: u64,
    /// Gas price in tokens
    pub gas_price: u128,
    /// Smart contract VM settings
    pub smart_contract_vm: SmartContractConfig,
}

/// Smart contract configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContractConfig {
    /// Maximum gas limit for contract execution
    pub max_gas_limit: u64,
    /// Maximum contract size in bytes
    pub max_contract_size: usize,
    /// Enable WebAssembly execution
    pub enable_wasm: bool,
    /// Contract storage size limit
    pub max_storage_size: usize,
}

/// Grid integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    pub endpoint: String,
    pub api_key: String,
    pub polling_interval: u64,
    pub timeout: u64,
    pub backup_endpoints: Vec<String>,
    pub grid_fee_rate: f32,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiry: u64,
    pub rate_limit: RateLimitConfig,
    pub encryption: EncryptionConfig,
    pub firewall: FirewallConfig,
}

/// Trading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    pub order_expiry: u64,
    pub min_trade_amount: u64,
    pub max_trade_amount: u64,
    pub price_precision: u32,
    pub market_hours: MarketHours,
    pub fees: TradingFees,
}

/// Governance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceConfig {
    pub proposal_deposit: u128,
    pub voting_period: u32,
    pub execution_delay: u32,
    pub quorum_threshold: u32,
    pub approval_threshold: u32,
}

/// Oracle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    pub price_feed_url: String,
    pub weather_api_url: String,
    pub grid_data_url: String,
    pub update_interval: u64,
    pub price_deviation_threshold: f32,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_limit: u32,
    pub whitelist: Vec<String>,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub rotation_interval: u64,
}

/// Firewall configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub allowed_ips: Vec<String>,
    pub blocked_ips: Vec<String>,
    pub geo_restrictions: Vec<String>,
}

/// Market hours configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketHours {
    pub open_hour: u8,
    pub close_hour: u8,
    pub timezone: String,
    pub weekend_trading: bool,
}

/// Trading fees configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingFees {
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub grid_fee: f32,
    pub validator_fee: f32,
}

impl SystemConfig {
    /// Load configuration from environment variables and defaults
    pub async fn load() -> Result<Self> {
        let config = Self {
            test_mode: env::var("TEST_MODE").unwrap_or_default() == "true",
            database: DatabaseConfig::load()?,
            blockchain: BlockchainConfig::load()?,
            grid: GridConfig::load()?,
            security: SecurityConfig::load()?,
            trading: TradingConfig::load()?,
            governance: GovernanceConfig::load()?,
            oracle: OracleConfig::load()?,
        };
        
        config.validate()?;
        Ok(config)
    }
    
    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        // Validate database configuration
        if self.database.url.is_empty() {
            return Err(anyhow::anyhow!("Database URL cannot be empty"));
        }
        
        // Validate blockchain configuration
        if self.blockchain.network.is_empty() {
            return Err(anyhow::anyhow!("Blockchain network cannot be empty"));
        }
        
        // Validate PoA-only configuration
        if self.blockchain.consensus_algorithm != "proof_of_authority" {
            return Err(anyhow::anyhow!(
                "Only Proof-of-Authority consensus is supported. Found: {}", 
                self.blockchain.consensus_algorithm
            ));
        }
        
        // Validate PoA-specific parameters
        if self.blockchain.block_time == 0 {
            return Err(anyhow::anyhow!("Block time must be greater than 0 seconds"));
        }
        
        if self.blockchain.max_validators == 0 {
            return Err(anyhow::anyhow!("Max validators must be greater than 0"));
        }
        
        if self.blockchain.authority_threshold == 0 || 
           self.blockchain.authority_threshold > self.blockchain.max_validators {
            return Err(anyhow::anyhow!(
                "Authority threshold must be between 1 and max validators ({})", 
                self.blockchain.max_validators
            ));
        }
        
        // Validate API configuration
        // if self.api.port < 1024 || self.api.port > 65535 {
        //     return Err(anyhow::anyhow!("API port must be between 1024 and 65535"));
        // }
        
        // Validate trading configuration
        if self.trading.min_trade_amount >= self.trading.max_trade_amount {
            return Err(anyhow::anyhow!("Min trade amount must be less than max trade amount"));
        }
        
        Ok(())
    }
}

impl DatabaseConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            url: env::var("DATABASE_URL").unwrap_or_else(|_| 
                "postgresql://thai_energy:password@localhost/thai_energy_db".to_string()
            ),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            connection_timeout: env::var("DATABASE_CONNECTION_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| 
                "redis://localhost:6379".to_string()
            ),
            redis_max_connections: env::var("REDIS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
        })
    }
}

impl BlockchainConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            network: env::var("BLOCKCHAIN_NETWORK").unwrap_or_else(|_| 
                "thai-energy-testnet".to_string()
            ),
            node_key: env::var("BLOCKCHAIN_NODE_KEY").unwrap_or_else(|_| 
                "0x1234567890abcdef".to_string()
            ),
            bootnodes: env::var("BLOCKCHAIN_BOOTNODES")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
            validator: env::var("BLOCKCHAIN_VALIDATOR")
                .unwrap_or_else(|_| "false".to_string())
                .parse()?,
            rpc_port: env::var("BLOCKCHAIN_RPC_PORT")
                .unwrap_or_else(|_| "9933".to_string())
                .parse()?,
            ws_port: env::var("BLOCKCHAIN_WS_PORT")
                .unwrap_or_else(|_| "9944".to_string())
                .parse()?,
            p2p_port: env::var("BLOCKCHAIN_P2P_PORT")
                .unwrap_or_else(|_| "30333".to_string())
                .parse()?,
            chain_spec: env::var("BLOCKCHAIN_CHAIN_SPEC").unwrap_or_else(|_| 
                "thai-energy-testnet.json".to_string()
            ),
            consensus_algorithm: env::var("CONSENSUS_ALGORITHM").unwrap_or_else(|_| 
                "proof_of_authority".to_string()
            ),
            block_time: env::var("BLOCK_TIME")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            validator_rotation_interval: env::var("VALIDATOR_ROTATION_INTERVAL")
                .unwrap_or_else(|_| "15".to_string())
                .parse()?,
            max_validators: env::var("MAX_VALIDATORS")
                .unwrap_or_else(|_| "21".to_string())
                .parse()?,
            authority_threshold: env::var("AUTHORITY_THRESHOLD")
                .unwrap_or_else(|_| "3".to_string())
                .parse()?,
            gas_limit: env::var("BLOCKCHAIN_GAS_LIMIT")
                .unwrap_or_else(|_| "1000000".to_string())
                .parse()?,
            gas_price: env::var("BLOCKCHAIN_GAS_PRICE")
                .unwrap_or_else(|_| "1000000000".to_string())
                .parse()?,
            smart_contract_vm: SmartContractConfig {
                max_gas_limit: env::var("SMART_CONTRACT_MAX_GAS_LIMIT")
                    .unwrap_or_else(|_| "10000000".to_string())
                    .parse()?,
                max_contract_size: env::var("SMART_CONTRACT_MAX_SIZE")
                    .unwrap_or_else(|_| "1048576".to_string())
                    .parse()?,
                enable_wasm: env::var("SMART_CONTRACT_ENABLE_WASM")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
                max_storage_size: env::var("SMART_CONTRACT_MAX_STORAGE_SIZE")
                    .unwrap_or_else(|_| "10485760".to_string())
                    .parse()?,
            },
        })
    }
}

impl GridConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            endpoint: env::var("GRID_ENDPOINT").unwrap_or_else(|_| 
                "https://api.grid.thailand.gov.th".to_string()
            ),
            api_key: env::var("GRID_API_KEY").unwrap_or_else(|_| 
                "your-grid-api-key".to_string()
            ),
            polling_interval: env::var("GRID_POLLING_INTERVAL")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            timeout: env::var("GRID_TIMEOUT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()?,
            backup_endpoints: env::var("GRID_BACKUP_ENDPOINTS")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect(),
            grid_fee_rate: env::var("GRID_FEE_RATE")
                .unwrap_or_else(|_| "0.05".to_string())
                .parse()?,
        })
    }
}

impl SecurityConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| 
                "your-super-secret-jwt-key".to_string()
            ),
            jwt_expiry: env::var("JWT_EXPIRY")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
            rate_limit: RateLimitConfig {
                requests_per_minute: env::var("RATE_LIMIT_PER_MINUTE")
                    .unwrap_or_else(|_| "60".to_string())
                    .parse()?,
                burst_limit: env::var("RATE_LIMIT_BURST")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
                whitelist: env::var("RATE_LIMIT_WHITELIST")
                    .unwrap_or_else(|_| "".to_string())
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            },
            encryption: EncryptionConfig {
                algorithm: env::var("ENCRYPTION_ALGORITHM").unwrap_or_else(|_| 
                    "AES-256-GCM".to_string()
                ),
                key_size: env::var("ENCRYPTION_KEY_SIZE")
                    .unwrap_or_else(|_| "256".to_string())
                    .parse()?,
                rotation_interval: env::var("ENCRYPTION_ROTATION_INTERVAL")
                    .unwrap_or_else(|_| "86400".to_string())
                    .parse()?,
            },
            firewall: FirewallConfig {
                allowed_ips: env::var("FIREWALL_ALLOWED_IPS")
                    .unwrap_or_else(|_| "0.0.0.0/0".to_string())
                    .split(',')
                    .map(|s| s.to_string())
                    .collect(),
                blocked_ips: env::var("FIREWALL_BLOCKED_IPS")
                    .unwrap_or_else(|_| "".to_string())
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
                geo_restrictions: env::var("FIREWALL_GEO_RESTRICTIONS")
                    .unwrap_or_else(|_| "".to_string())
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect(),
            },
        })
    }
}

impl TradingConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            order_expiry: env::var("TRADING_ORDER_EXPIRY")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,
            min_trade_amount: env::var("TRADING_MIN_AMOUNT")
                .unwrap_or_else(|_| "1".to_string())
                .parse()?,
            max_trade_amount: env::var("TRADING_MAX_AMOUNT")
                .unwrap_or_else(|_| "10000".to_string())
                .parse()?,
            price_precision: env::var("TRADING_PRICE_PRECISION")
                .unwrap_or_else(|_| "4".to_string())
                .parse()?,
            market_hours: MarketHours {
                open_hour: env::var("MARKET_OPEN_HOUR")
                    .unwrap_or_else(|_| "0".to_string())
                    .parse()?,
                close_hour: env::var("MARKET_CLOSE_HOUR")
                    .unwrap_or_else(|_| "23".to_string())
                    .parse()?,
                timezone: env::var("MARKET_TIMEZONE").unwrap_or_else(|_| 
                    "Asia/Bangkok".to_string()
                ),
                weekend_trading: env::var("MARKET_WEEKEND_TRADING")
                    .unwrap_or_else(|_| "true".to_string())
                    .parse()?,
            },
            fees: TradingFees {
                maker_fee: env::var("TRADING_MAKER_FEE")
                    .unwrap_or_else(|_| "0.001".to_string())
                    .parse()?,
                taker_fee: env::var("TRADING_TAKER_FEE")
                    .unwrap_or_else(|_| "0.002".to_string())
                    .parse()?,
                grid_fee: env::var("TRADING_GRID_FEE")
                    .unwrap_or_else(|_| "0.05".to_string())
                    .parse()?,
                validator_fee: env::var("TRADING_VALIDATOR_FEE")
                    .unwrap_or_else(|_| "0.001".to_string())
                    .parse()?,
            },
        })
    }
}

impl GovernanceConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            proposal_deposit: env::var("GOVERNANCE_PROPOSAL_DEPOSIT")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            voting_period: env::var("GOVERNANCE_VOTING_PERIOD")
                .unwrap_or_else(|_| "168".to_string())
                .parse()?,
            execution_delay: env::var("GOVERNANCE_EXECUTION_DELAY")
                .unwrap_or_else(|_| "24".to_string())
                .parse()?,
            quorum_threshold: env::var("GOVERNANCE_QUORUM_THRESHOLD")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            approval_threshold: env::var("GOVERNANCE_APPROVAL_THRESHOLD")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,
        })
    }
}

impl OracleConfig {
    fn load() -> Result<Self> {
        Ok(Self {
            price_feed_url: env::var("ORACLE_PRICE_FEED_URL").unwrap_or_else(|_| 
                "https://api.coingecko.com/api/v3".to_string()
            ),
            weather_api_url: env::var("ORACLE_WEATHER_API_URL").unwrap_or_else(|_| 
                "https://api.openweathermap.org/data/2.5".to_string()
            ),
            grid_data_url: env::var("ORACLE_GRID_DATA_URL").unwrap_or_else(|_| 
                "https://api.grid.thailand.gov.th".to_string()
            ),
            update_interval: env::var("ORACLE_UPDATE_INTERVAL")
                .unwrap_or_else(|_| "300".to_string())
                .parse()?,
            price_deviation_threshold: env::var("ORACLE_PRICE_DEVIATION_THRESHOLD")
                .unwrap_or_else(|_| "0.05".to_string())
                .parse()?,
        })
    }
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            test_mode: true, // Default to test mode for development
            database: DatabaseConfig::default(),
            blockchain: BlockchainConfig::default(),
            grid: GridConfig::default(),
            security: SecurityConfig::default(),
            trading: TradingConfig::default(),
            governance: GovernanceConfig::default(),
            oracle: OracleConfig::default(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://postgres:password@localhost/thai_energy_trading".to_string(),
            max_connections: 100,
            connection_timeout: 30,
            redis_url: "redis://localhost:6379".to_string(),
            redis_max_connections: 50,
        }
    }
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            network: "development".to_string(),
            node_key: "default_node_key".to_string(),
            bootnodes: vec![],
            validator: true,
            rpc_port: 9933,
            ws_port: 9944,
            p2p_port: 30333,
            chain_spec: "dev".to_string(),
            enable_mining: true,
            mining_difficulty: 4,
            gas_limit: 1000000,
            gas_price: 1000000000,
            smart_contract_vm: SmartContractConfig::default(),
        }
    }
}

impl Default for SmartContractConfig {
    fn default() -> Self {
        Self {
            max_gas_limit: 1000000,
            max_contract_size: 1048576,
            enable_wasm: true,
            max_storage_size: 10485760,
        }
    }
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.grid.thailand.gov.th".to_string(),
            api_key: "your-grid-api-key".to_string(),
            polling_interval: 30,
            timeout: 10,
            backup_endpoints: vec![],
            grid_fee_rate: 0.05,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "default_jwt_secret_key_change_in_production".to_string(),
            jwt_expiry: 3600,
            rate_limit: RateLimitConfig::default(),
            encryption: EncryptionConfig::default(),
            firewall: FirewallConfig::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_limit: 10,
            whitelist: vec![],
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
            rotation_interval: 86400,
        }
    }
}

impl Default for FirewallConfig {
    fn default() -> Self {
        Self {
            allowed_ips: vec!["0.0.0.0/0".to_string()],
            blocked_ips: vec![],
            geo_restrictions: vec![],
        }
    }
}

impl Default for TradingConfig {
    fn default() -> Self {
        Self {
            order_expiry: 3600,
            min_trade_amount: 1,
            max_trade_amount: 10000,
            price_precision: 4,
            market_hours: MarketHours::default(),
            fees: TradingFees::default(),
        }
    }
}

impl Default for MarketHours {
    fn default() -> Self {
        Self {
            open_hour: 0,
            close_hour: 23,
            timezone: "Asia/Bangkok".to_string(),
            weekend_trading: true,
        }
    }
}

impl Default for TradingFees {
    fn default() -> Self {
        Self {
            maker_fee: 0.001,
            taker_fee: 0.002,
            grid_fee: 0.05,
            validator_fee: 0.001,
        }
    }
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            proposal_deposit: 1000,
            voting_period: 168,
            execution_delay: 24,
            quorum_threshold: 30,
            approval_threshold: 50,
        }
    }
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            price_feed_url: "https://api.coingecko.com/api/v3".to_string(),
            weather_api_url: "https://api.openweathermap.org/data/2.5".to_string(),
            grid_data_url: "https://api.grid.thailand.gov.th".to_string(),
            update_interval: 300,
            price_deviation_threshold: 0.05,
        }
    }
}
