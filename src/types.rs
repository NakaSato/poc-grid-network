//! # Core Types for Thai Energy Trading System
//! 
//! This module defines all the core data types used throughout the system.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Account identifier type - using string for simplicity
pub type AccountId = String;

/// Hash type used throughout the system
pub type Hash = String;

/// Balance type for token amounts (1 token = 1 kWh)
pub type Balance = u128;

/// Energy amount in kWh
pub type EnergyAmount = f64;

/// Token price per kWh
pub type TokenPrice = f64;

/// Trade status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TradeStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
    Failed,
}

/// Energy trade record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyTrade {
    /// Trade ID
    pub trade_id: String,
    /// Energy amount traded
    pub energy_amount: EnergyAmount,
    /// Price per unit
    pub price_per_unit: Balance,
    /// Buyer ID
    pub buyer_id: String,
    /// Seller ID
    pub seller_id: String,
    /// Trade timestamp
    pub timestamp: u64,
    /// Trade status
    pub status: TradeStatus,
    /// Grid location
    pub grid_location: GridLocation,
    /// Legacy field mappings for compatibility
    pub id: String,
    pub buy_order_id: String,
    pub sell_order_id: String,
    pub price_per_kwh: Balance,
    pub total_price: Balance,
    pub grid_fee: Balance,
    pub energy_source: EnergySource,
    pub carbon_offset: CarbonOffset,
}

/// Grid location for energy routing
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridLocation {
    /// Province
    pub province: String,
    /// District
    pub district: String,
    /// Grid coordinates
    pub coordinates: GridCoordinates,
    /// Region identifier
    pub region: String,
    /// Substation identifier
    pub substation: String,
    /// Grid code
    pub grid_code: String,
    /// Meter ID
    pub meter_id: String,
}

/// Grid coordinates structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridCoordinates {
    pub lat: f64,
    pub lng: f64,
}

impl Eq for GridLocation {}

impl std::hash::Hash for GridLocation {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.province.hash(state);
        self.district.hash(state);
        self.region.hash(state);
        self.substation.hash(state);
        self.grid_code.hash(state);
        self.meter_id.hash(state);
        // Hash coordinately as ordered bytes to handle f64
        self.coordinates.lat.to_bits().hash(state);
        self.coordinates.lng.to_bits().hash(state);
    }
}

/// Energy source type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnergySource {
    /// Solar energy
    Solar,
    /// Wind energy
    Wind,
    /// Hydroelectric energy
    Hydro,
    /// Biomass energy
    Biomass,
    /// Natural gas
    NaturalGas,
    /// Nuclear energy
    Nuclear,
    /// Coal energy
    Coal,
    /// Gas energy
    Gas,
    /// Mixed energy sources
    Mixed,
}

/// Producer type classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProducerType {
    Residential,      // Home solar panels
    Commercial,       // Business renewable energy
    Industrial,       // Large-scale production
    Utility,         // Traditional power plants
    Community,       // Community energy projects
}

/// Grid status for monitoring
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridStatus {
    /// Region identifier
    pub region: String,
    /// Current load
    pub current_load: f64,
    /// Maximum capacity
    pub max_capacity: f64,
    /// Grid health score
    pub health: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
    /// Additional fields for compatibility
    pub location: GridLocation,
    pub capacity: f64,
    pub congestion_level: CongestionLevel,
    pub stability_score: f64,
    pub outage_risk: f64,
    pub updated_at: DateTime<Utc>,
}

impl GridStatus {
    /// Check if grid can handle additional load
    pub fn can_handle_load(&self, additional_load: f64) -> bool {
        (self.current_load + additional_load) <= self.max_capacity
    }
}

/// Congestion level enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CongestionLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Energy order for trading
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyOrder {
    /// Order ID
    pub id: Uuid,
    /// Order type
    pub order_type: OrderType,
    /// Energy amount
    pub energy_amount: EnergyAmount,
    /// Price per unit
    pub price_per_unit: Balance,
    /// Location
    pub location: GridLocation,
    /// Energy source
    pub energy_source: Option<EnergySource>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Status
    pub status: OrderStatus,
    /// Account ID
    pub account_id: AccountId,
    /// Updated at timestamp
    pub updated_at: DateTime<Utc>,
}

/// Order type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

/// Order status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
    Expired,
}

/// Energy production record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProductionRecord {
    /// Production amount
    pub amount: EnergyAmount,
    /// Energy type
    pub energy_type: EnergySource,
    /// Location
    pub location: GridLocation,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Verified flag
    pub verified: bool,
    /// Producer ID
    pub producer_id: AccountId,
    /// Equipment ID
    pub equipment_id: String,
    /// Production efficiency
    pub efficiency: f64,
    /// Quality metrics
    pub quality_metrics: QualityMetrics,
    /// Energy source for compatibility
    pub energy_source: EnergySource,
}

/// Energy consumption record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConsumptionRecord {
    /// Consumption amount
    pub amount: EnergyAmount,
    /// Energy type
    pub energy_type: EnergySource,
    /// Location
    pub location: GridLocation,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Consumer ID
    pub consumer_id: AccountId,
    /// Consumer type
    pub consumer_type: ConsumerType,
    /// Appliance breakdown
    pub appliance_breakdown: HashMap<String, EnergyAmount>,
}

/// Consumer type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsumerType {
    Residential,
    Commercial,
    Industrial,
    Municipal,
}

/// Quality metrics for energy production
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Power quality score
    pub power_quality: f64,
    /// Reliability score
    pub reliability: f64,
    /// Efficiency score
    pub efficiency: f64,
    /// Environmental impact score
    pub environmental_impact: f64,
}

/// Carbon offset information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarbonOffset {
    /// Offset credits
    pub offset_credits: f64,
    /// Verification status
    pub verified: bool,
    /// Certification body
    pub certification_body: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Governance proposal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceProposal {
    /// Proposal ID
    pub id: Uuid,
    /// Proposal title
    pub title: String,
    /// Proposal description
    pub description: String,
    /// Proposer ID
    pub proposer: AccountId,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Voting deadline
    pub voting_deadline: DateTime<Utc>,
    /// Minimum voting power required
    pub minimum_voting_power: Balance,
    /// Current status
    pub status: ProposalStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Vote results
    pub vote_results: VotingResults,
}

/// Proposal type enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProposalType {
    GridUpgrade,
    PricingPolicy,
    RegulationChange,
    FeatureUpdate,
    Emergency,
}

/// Proposal status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProposalStatus {
    Draft,
    Active,
    Passed,
    Rejected,
    Expired,
    Implemented,
}

/// Voting results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VotingResults {
    /// Yes votes
    pub yes_votes: Balance,
    /// No votes
    pub no_votes: Balance,
    /// Abstain votes
    pub abstain_votes: Balance,
    /// Total voting power
    pub total_voting_power: Balance,
    /// Participation rate
    pub participation_rate: f64,
    /// Total eligible voters
    pub total_eligible: u32,
    /// Turnout percentage
    pub turnout_percentage: f64,
}

/// Vote choice enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

/// System configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    /// Blockchain configuration
    pub blockchain: BlockchainConfig,
    /// API configuration
    pub api: ApiConfig,
    /// Grid configuration
    pub grid: GridConfig,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database URL
    pub url: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout
    pub timeout: u64,
}

/// Blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Network ID
    pub network_id: String,
    /// Consensus algorithm
    pub consensus: ConsensusConfig,
    /// Transaction pool size
    pub transaction_pool_size: usize,
    /// Block time in seconds
    pub block_time: u64,
}

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// CORS configuration
    pub cors: CorsConfig,
    /// Rate limiting
    pub rate_limit: RateLimitConfig,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
}

/// Grid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    /// Grid capacity
    pub capacity: f64,
    /// Grid efficiency
    pub efficiency: f64,
    /// Grid regions
    pub regions: Vec<String>,
}

/// Consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Consensus type
    pub consensus_type: String,
    /// Block size limit
    pub block_size_limit: usize,
    /// Difficulty adjustment
    pub difficulty_adjustment: u64,
}

/// Price trend enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PriceTrend {
    Rising,
    Falling,
    Stable,
}

/// Market data for energy trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    /// Current price
    pub current_price: TokenPrice,
    /// 24h volume
    pub volume_24h: EnergyAmount,
    /// Price change 24h
    pub price_change_24h: f64,
    /// Number of trades
    pub trades_24h: u32,
    /// High price 24h
    pub high_24h: TokenPrice,
    /// Low price 24h
    pub low_24h: TokenPrice,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Energy source
    pub energy_source: EnergySource,
    /// Location
    pub location: GridLocation,
    /// Price trend
    pub price_trend: PriceTrend,
}

/// User portfolio information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPortfolio {
    /// Account ID
    pub account_id: AccountId,
    /// Token balance
    pub token_balance: Balance,
    /// Energy credits
    pub energy_credits: EnergyAmount,
    /// Active orders
    pub active_orders: Vec<EnergyOrder>,
    /// Recent trades
    pub recent_trades: Vec<EnergyTrade>,
    /// Carbon credits
    pub carbon_credits: f64,
    /// Reputation score
    pub reputation_score: f64,
}

/// Energy token representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyToken {
    /// Token ID
    pub id: String,
    /// Token amount
    pub amount: Balance,
    /// Energy type
    pub energy_type: EnergySource,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiry timestamp
    pub expires_at: DateTime<Utc>,
    /// Verification status
    pub verified: bool,
}

/// System status for monitoring and TPS testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// Memory usage in KB
    pub memory_usage_kb: u64,
    /// Active database connections
    pub active_connections: u32,
    /// Cache hit rate percentage
    pub cache_hit_rate: f64,
    /// Current blockchain height
    pub current_block_height: u64,
    /// Number of pending transactions
    pub pending_transactions: u32,
}
