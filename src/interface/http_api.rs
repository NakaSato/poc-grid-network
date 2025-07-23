//! # HTTP API Bridge
//! 
//! HTTP/REST API bridge for public access to GridTokenX blockchain functionality.
//! This layer bridges the blockchain system with standard web APIs.

use crate::application::trading::TradingService;
use crate::application::grid::GridService;
use crate::application::governance::GovernanceService;
use crate::application::oracle::OracleService;
use crate::application::enhanced_trading::EnhancedTradingService;
use crate::infrastructure::security::SecurityManager;
use crate::types::*;
use crate::utils::{SystemResult, SystemError};
use std::sync::Arc;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;
use warp::Filter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// HTTP API Server for public blockchain access
pub struct HttpApiServer {
    trading_service: Arc<TradingService>,
    enhanced_trading_service: Arc<EnhancedTradingService>,
    grid_service: Arc<GridService>,
    governance_service: Arc<GovernanceService>,
    oracle_service: Arc<OracleService>,
    security_manager: Arc<SecurityManager>,
    running: Arc<RwLock<bool>>,
    bind_address: SocketAddr,
}

impl HttpApiServer {
    pub async fn new(
        trading_service: Arc<TradingService>,
        enhanced_trading_service: Arc<EnhancedTradingService>,
        grid_service: Arc<GridService>,
        governance_service: Arc<GovernanceService>,
        oracle_service: Arc<OracleService>,
        security_manager: Arc<SecurityManager>,
        bind_address: SocketAddr,
    ) -> SystemResult<Self> {
        Ok(Self {
            trading_service,
            enhanced_trading_service,
            grid_service,
            governance_service,
            oracle_service,
            security_manager,
            running: Arc::new(RwLock::new(false)),
            bind_address,
        })
    }

    /// Start the HTTP API server
    pub async fn start(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = true;
        drop(running);

        log::info!("🌐 Starting GridTokenX HTTP API Server on {}", self.bind_address);

        // Create API routes
        let routes = self.create_routes().await;

        // Start the server
        let server = warp::serve(routes)
            .bind(self.bind_address);

        // Run server in background
        tokio::spawn(server);

        log::info!("✅ GridTokenX HTTP API Server started successfully");
        Ok(())
    }

    /// Stop the HTTP API server
    pub async fn stop(&self) -> SystemResult<()> {
        let mut running = self.running.write().await;
        *running = false;
        log::info!("🛑 GridTokenX HTTP API Server stopped");
        Ok(())
    }

    /// Create all API routes
    async fn create_routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Clone services for route handlers
        let trading_service = self.trading_service.clone();
        let enhanced_trading_service = self.enhanced_trading_service.clone();
        let grid_service = self.grid_service.clone();
        let governance_service = self.governance_service.clone();
        let oracle_service = self.oracle_service.clone();

        // Health check endpoint
        let health = warp::path("health")
            .and(warp::get())
            .map(|| {
                warp::reply::json(&ApiResponse::success(HealthStatus {
                    status: "healthy".to_string(),
                    service: "GridTokenX POC Blockchain".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    timestamp: chrono::Utc::now().timestamp() as u64,
                }))
            });

        // System info endpoint
        let info = warp::path("info")
            .and(warp::get())
            .map(|| {
                warp::reply::json(&ApiResponse::success(SystemInfo {
                    name: "GridTokenX POC Blockchain".to_string(),
                    description: "Blockchain-based peer-to-peer energy trading POC".to_string(),
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    consensus: "Proof-of-Authority".to_string(),
                    token_ratio: "1 kWh = 1 Token".to_string(),
                    network: "Thai Energy Trading Network".to_string(),
                }))
            });

        // Trading endpoints
        let trading_routes = self.create_trading_routes(trading_service.clone(), enhanced_trading_service.clone()).await;
        
        // Grid endpoints
        let grid_routes = self.create_grid_routes(grid_service.clone()).await;
        
        // Governance endpoints
        let governance_routes = self.create_governance_routes(governance_service.clone()).await;
        
        // Oracle endpoints
        let oracle_routes = self.create_oracle_routes(oracle_service.clone()).await;

        // API documentation endpoint
        let docs = warp::path("docs")
            .and(warp::get())
            .map(|| {
                warp::reply::html(include_str!("../../docs/api_docs.html").unwrap_or(
                    "<h1>GridTokenX API Documentation</h1><p>API documentation will be available here.</p>"
                ))
            });

        // Combine all routes with CORS
        let api = warp::path("api")
            .and(warp::path("v1"))
            .and(
                health
                    .or(info)
                    .or(trading_routes)
                    .or(grid_routes)
                    .or(governance_routes)
                    .or(oracle_routes)
            );

        let routes = api
            .or(docs)
            .with(warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["content-type", "authorization"])
                .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            )
            .with(warp::log("gridtokenx_api"))
            .recover(handle_rejection);

        routes
    }

    /// Create trading-related API routes
    async fn create_trading_routes(
        &self,
        trading_service: Arc<TradingService>,
        enhanced_trading_service: Arc<EnhancedTradingService>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Place energy order
        let place_order = warp::path("orders")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || trading_service.clone()))
            .and_then(|order: CreateOrderRequest, trading_service: Arc<TradingService>| async move {
                match Self::handle_place_order(order, trading_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        // Get orders by location
        let get_orders = warp::path("orders")
            .and(warp::get())
            .and(warp::query::<OrderQuery>())
            .and(warp::any().map(move || enhanced_trading_service.clone()))
            .and_then(|query: OrderQuery, enhanced_service: Arc<EnhancedTradingService>| async move {
                match Self::handle_get_orders(query, enhanced_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        // Cancel order
        let cancel_order = warp::path("orders")
            .and(warp::path::param::<String>())
            .and(warp::delete())
            .and(warp::any().map(move || trading_service.clone()))
            .and_then(|order_id: String, trading_service: Arc<TradingService>| async move {
                match Uuid::parse_str(&order_id) {
                    Ok(uuid) => {
                        match Self::handle_cancel_order(uuid, trading_service).await {
                            Ok(response) => Ok(warp::reply::json(&response)),
                            Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                        }
                    },
                    Err(_) => Ok(warp::reply::json(&ApiResponse::<()>::error("Invalid order ID format"))),
                }
            });

        // Get market data
        let market_data = warp::path("market")
            .and(warp::get())
            .and(warp::query::<MarketQuery>())
            .and(warp::any().map(move || enhanced_trading_service.clone()))
            .and_then(|query: MarketQuery, enhanced_service: Arc<EnhancedTradingService>| async move {
                match Self::handle_get_market_data(query, enhanced_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        place_order
            .or(get_orders)
            .or(cancel_order)
            .or(market_data)
    }

    /// Create grid-related API routes
    async fn create_grid_routes(
        &self,
        grid_service: Arc<GridService>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Get grid status
        let grid_status = warp::path("grid")
            .and(warp::path("status"))
            .and(warp::get())
            .and(warp::query::<GridQuery>())
            .and(warp::any().map(move || grid_service.clone()))
            .and_then(|query: GridQuery, grid_service: Arc<GridService>| async move {
                match Self::handle_get_grid_status(query, grid_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        grid_status
    }

    /// Create governance-related API routes
    async fn create_governance_routes(
        &self,
        governance_service: Arc<GovernanceService>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Create proposal
        let create_proposal = warp::path("governance")
            .and(warp::path("proposals"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || governance_service.clone()))
            .and_then(|proposal: CreateProposalRequest, governance_service: Arc<GovernanceService>| async move {
                match Self::handle_create_proposal(proposal, governance_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        // Cast vote
        let cast_vote = warp::path("governance")
            .and(warp::path("vote"))
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::any().map(move || governance_service.clone()))
            .and_then(|vote: CastVoteRequest, governance_service: Arc<GovernanceService>| async move {
                match Self::handle_cast_vote(vote, governance_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        create_proposal.or(cast_vote)
    }

    /// Create oracle-related API routes
    async fn create_oracle_routes(
        &self,
        oracle_service: Arc<OracleService>,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        // Get price feeds
        let price_feeds = warp::path("oracle")
            .and(warp::path("prices"))
            .and(warp::get())
            .and(warp::any().map(move || oracle_service.clone()))
            .and_then(|oracle_service: Arc<OracleService>| async move {
                match Self::handle_get_price_feeds(oracle_service).await {
                    Ok(response) => Ok(warp::reply::json(&response)),
                    Err(err) => Ok(warp::reply::json(&ApiResponse::<()>::error(&err.to_string()))),
                }
            });

        price_feeds
    }

    // Route handlers
    async fn handle_place_order(
        order: CreateOrderRequest,
        trading_service: Arc<TradingService>,
    ) -> SystemResult<ApiResponse<OrderResponse>> {
        let energy_order = EnergyOrder {
            id: Uuid::new_v4(),
            order_type: order.order_type,
            energy_amount: order.energy_amount,
            price_per_unit: order.price_per_unit,
            energy_source: order.energy_source,
            location: order.location,
            timestamp: chrono::Utc::now(),
            status: OrderStatus::Pending,
            account_id: order.account_id,
            updated_at: chrono::Utc::now(),
        };

        trading_service.place_order(energy_order.clone()).await?;

        Ok(ApiResponse::success(OrderResponse {
            order_id: energy_order.id,
            status: "placed".to_string(),
            message: "Order successfully placed in the energy trading system".to_string(),
        }))
    }

    async fn handle_get_orders(
        query: OrderQuery,
        enhanced_service: Arc<EnhancedTradingService>,
    ) -> SystemResult<ApiResponse<Vec<EnergyOrder>>> {
        let orders = enhanced_service
            .get_market_orders(&query.location, query.energy_source)
            .await?;

        Ok(ApiResponse::success(orders))
    }

    async fn handle_cancel_order(
        order_id: Uuid,
        trading_service: Arc<TradingService>,
    ) -> SystemResult<ApiResponse<CancelResponse>> {
        trading_service.cancel_order(order_id, &"api_user".to_string()).await?;

        Ok(ApiResponse::success(CancelResponse {
            order_id,
            status: "cancelled".to_string(),
            message: "Order successfully cancelled".to_string(),
        }))
    }

    async fn handle_get_market_data(
        query: MarketQuery,
        enhanced_service: Arc<EnhancedTradingService>,
    ) -> SystemResult<ApiResponse<MarketData>> {
        let current_price = enhanced_service
            .calculate_current_price(&query.location)
            .await?;

        let volume_24h = enhanced_service
            .calculate_volume_24h(&query.location)
            .await?;

        let trades_24h = enhanced_service
            .calculate_trades_24h(&query.location)
            .await?;

        Ok(ApiResponse::success(MarketData {
            location: query.location,
            current_price,
            volume_24h,
            trades_24h,
            timestamp: chrono::Utc::now(),
        }))
    }

    async fn handle_get_grid_status(
        query: GridQuery,
        grid_service: Arc<GridService>,
    ) -> SystemResult<ApiResponse<GridStatus>> {
        // This would call actual grid service methods
        Ok(ApiResponse::success(GridStatus {
            location: query.location,
            capacity_available: 1000.0, // Mock data
            load_current: 750.0,
            status: "operational".to_string(),
            timestamp: chrono::Utc::now(),
        }))
    }

    async fn handle_create_proposal(
        proposal: CreateProposalRequest,
        governance_service: Arc<GovernanceService>,
    ) -> SystemResult<ApiResponse<ProposalResponse>> {
        let governance_proposal = crate::application::governance::GovernanceProposal {
            id: Uuid::new_v4(),
            title: proposal.title,
            description: proposal.description,
            proposal_type: proposal.proposal_type,
            proposer: proposal.proposer,
            voting_start: chrono::Utc::now(),
            voting_end: chrono::Utc::now() + chrono::Duration::days(7),
            status: crate::application::governance::ProposalStatus::Active,
            required_threshold: 0.5,
            created_at: chrono::Utc::now(),
        };

        governance_service.create_proposal(governance_proposal.clone()).await?;

        Ok(ApiResponse::success(ProposalResponse {
            proposal_id: governance_proposal.id,
            status: "created".to_string(),
            message: "Proposal successfully created".to_string(),
        }))
    }

    async fn handle_cast_vote(
        vote: CastVoteRequest,
        governance_service: Arc<GovernanceService>,
    ) -> SystemResult<ApiResponse<VoteResponse>> {
        let governance_vote = crate::application::governance::GovernanceVote {
            proposal_id: vote.proposal_id.to_string(),
            voter_id: vote.voter_id,
            vote: vote.vote,
            voting_power: vote.voting_power.unwrap_or(100),
            timestamp: chrono::Utc::now(),
        };

        governance_service.cast_vote(governance_vote).await?;

        Ok(ApiResponse::success(VoteResponse {
            vote_id: Uuid::new_v4(),
            status: "recorded".to_string(),
            message: "Vote successfully recorded".to_string(),
        }))
    }

    async fn handle_get_price_feeds(
        oracle_service: Arc<OracleService>,
    ) -> SystemResult<ApiResponse<PriceFeeds>> {
        // This would call actual oracle service methods
        Ok(ApiResponse::success(PriceFeeds {
            energy_prices: vec![
                PriceFeed {
                    source: EnergySource::Solar,
                    price_per_kwh: 50000, // tokens
                    timestamp: chrono::Utc::now(),
                },
                PriceFeed {
                    source: EnergySource::Wind,
                    price_per_kwh: 45000,
                    timestamp: chrono::Utc::now(),
                },
            ],
            timestamp: chrono::Utc::now(),
        }))
    }
}

// API Data Structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub service: String,
    pub version: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub consensus: String,
    pub token_ratio: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub order_type: OrderType,
    pub energy_amount: f64,
    pub price_per_unit: u128,
    pub energy_source: Option<EnergySource>,
    pub location: GridLocation,
    pub account_id: String,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order_id: Uuid,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct CancelResponse {
    pub order_id: Uuid,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderQuery {
    pub location: GridLocation,
    pub energy_source: Option<EnergySource>,
}

#[derive(Debug, Deserialize)]
pub struct MarketQuery {
    pub location: GridLocation,
}

#[derive(Debug, Serialize)]
pub struct MarketData {
    pub location: GridLocation,
    pub current_price: u128,
    pub volume_24h: f64,
    pub trades_24h: u32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GridQuery {
    pub location: GridLocation,
}

#[derive(Debug, Serialize)]
pub struct GridStatus {
    pub location: GridLocation,
    pub capacity_available: f64,
    pub load_current: f64,
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub proposal_type: crate::application::governance::ProposalType,
    pub proposer: String,
}

#[derive(Debug, Serialize)]
pub struct ProposalResponse {
    pub proposal_id: Uuid,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CastVoteRequest {
    pub proposal_id: String,
    pub voter_id: String,
    pub vote: crate::application::governance::VoteType,
    pub voting_power: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct VoteResponse {
    pub vote_id: Uuid,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct PriceFeeds {
    pub energy_prices: Vec<PriceFeed>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct PriceFeed {
    pub source: EnergySource,
    pub price_per_kwh: u128,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// Error handling
async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    let response = if err.is_not_found() {
        ApiResponse::<()>::error("Not found")
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        ApiResponse::<()>::error("Invalid request body")
    } else {
        ApiResponse::<()>::error("Internal server error")
    };

    Ok(warp::reply::json(&response))
}
