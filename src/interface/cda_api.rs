//! # CDA Trading API Endpoints
//! 
//! This module provides REST API endpoints for the Continuous Double Auction
//! energy trading system, including order placement, market data, and real-time updates.

use crate::application::enhanced_trading::EnhancedTradingService;
use crate::runtime::continuous_double_auction::{MarketDepth, OrderBookEvent, TradeExecution};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;
use warp::{Filter, Reply};

/// API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: crate::utils::now().to_rfc3339(),
        }
    }
    
    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message.to_string()),
            timestamp: crate::utils::now().to_rfc3339(),
        }
    }
}

/// Order placement request
#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    pub order_type: OrderType,
    pub energy_amount: EnergyAmount,
    pub price_per_unit: Balance,
    pub location: GridLocation,
    pub energy_source: Option<EnergySource>,
    pub account_id: AccountId,
    pub time_in_force: Option<String>, // "GTC", "IOC", "FOK", "DAY"
    pub post_only: Option<bool>,
}

/// Order cancellation request
#[derive(Debug, Deserialize)]
pub struct CancelOrderRequest {
    pub order_id: Uuid,
    pub account_id: AccountId,
}

/// Market data request
#[derive(Debug, Deserialize)]
pub struct MarketDataRequest {
    pub location: GridLocation,
    pub energy_source: Option<EnergySource>,
}

/// Market depth request
#[derive(Debug, Deserialize)]
pub struct MarketDepthRequest {
    pub location: GridLocation,
    pub levels: Option<usize>,
}

/// Trade history request
#[derive(Debug, Deserialize)]
pub struct TradeHistoryRequest {
    pub account_id: Option<AccountId>,
    pub location: Option<GridLocation>,
    pub limit: Option<usize>,
    pub from_timestamp: Option<String>,
    pub to_timestamp: Option<String>,
}

/// Order placement response
#[derive(Debug, Serialize)]
pub struct PlaceOrderResponse {
    pub order_id: Uuid,
    pub executions: Vec<TradeExecutionInfo>,
    pub status: OrderStatus,
    pub message: String,
}

/// Trade execution info for API response
#[derive(Debug, Serialize)]
pub struct TradeExecutionInfo {
    pub trade_id: Uuid,
    pub price: TokenPrice,
    pub quantity: EnergyAmount,
    pub total_value: TokenPrice,
    pub fees: TokenPrice,
    pub counterparty: String, // Anonymized counterparty info
    pub timestamp: String,
}

impl From<&TradeExecution> for TradeExecutionInfo {
    fn from(execution: &TradeExecution) -> Self {
        Self {
            trade_id: execution.trade_id,
            price: execution.price,
            quantity: execution.quantity,
            total_value: execution.price * execution.quantity,
            fees: execution.fees.total_fee,
            counterparty: "***".to_string(), // Anonymized for privacy
            timestamp: execution.execution_time.to_rfc3339(),
        }
    }
}

/// CDA Trading API
pub struct CDAApiService {
    trading_service: Arc<EnhancedTradingService>,
}

impl CDAApiService {
    pub fn new(trading_service: Arc<EnhancedTradingService>) -> Self {
        Self { trading_service }
    }
    
    /// Create API routes
    pub fn routes(
        &self,
    ) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let trading_service = Arc::clone(&self.trading_service);
        
        // Place order endpoint
        let place_order = warp::path!("api" / "v1" / "orders")
            .and(warp::post())
            .and(warp::body::json())
            .and(self.with_trading_service(trading_service.clone()))
            .and_then(Self::place_order_handler);
        
        // Cancel order endpoint
        let cancel_order = warp::path!("api" / "v1" / "orders" / "cancel")
            .and(warp::post())
            .and(warp::body::json())
            .and(self.with_trading_service(trading_service.clone()))
            .and_then(Self::cancel_order_handler);
        
        // Get market depth
        let market_depth = warp::path!("api" / "v1" / "market" / "depth")
            .and(warp::get())
            .and(warp::query::<MarketDepthRequest>())
            .and(self.with_trading_service(trading_service.clone()))
            .and_then(Self::market_depth_handler);
        
        // Get market data
        let market_data = warp::path!("api" / "v1" / "market" / "data")
            .and(warp::get())
            .and(warp::query::<MarketDataRequest>())
            .and(self.with_trading_service(trading_service.clone()))
            .and_then(Self::market_data_handler);
        
        // Get trade history
        let trade_history = warp::path!("api" / "v1" / "trades")
            .and(warp::get())
            .and(warp::query::<TradeHistoryRequest>())
            .and(self.with_trading_service(trading_service.clone()))
            .and_then(Self::trade_history_handler);
        
        // Get user orders
        let user_orders = warp::path!("api" / "v1" / "orders" / String)
            .and(warp::get())
            .and(self.with_trading_service(trading_service.clone()))
            .and_then(Self::user_orders_handler);
        
        // WebSocket endpoint for real-time updates
        let websocket = warp::path!("api" / "v1" / "ws")
            .and(warp::ws())
            .and(self.with_trading_service(trading_service))
            .map(|ws: warp::ws::Ws, trading_service| {
                ws.on_upgrade(move |websocket| {
                    Self::websocket_handler(websocket, trading_service)
                })
            });
        
        place_order
            .or(cancel_order)
            .or(market_depth)
            .or(market_data)
            .or(trade_history)
            .or(user_orders)
            .or(websocket)
    }
    
    /// Helper to inject trading service into handlers
    fn with_trading_service(
        &self,
        trading_service: Arc<EnhancedTradingService>,
    ) -> impl Filter<Extract = (Arc<EnhancedTradingService>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || trading_service.clone())
    }
    
    /// Place order handler
    async fn place_order_handler(
        request: PlaceOrderRequest,
        trading_service: Arc<EnhancedTradingService>,
    ) -> Result<impl Reply, warp::Rejection> {
        let order = EnergyOrder {
            id: Uuid::new_v4(),
            order_type: request.order_type,
            energy_amount: request.energy_amount,
            price_per_unit: request.price_per_unit,
            location: request.location,
            energy_source: request.energy_source,
            timestamp: crate::utils::now(),
            status: OrderStatus::Pending,
            account_id: request.account_id,
            updated_at: crate::utils::now(),
        };
        
        match trading_service.place_order(order).await {
            Ok(result) => {
                let executions: Vec<TradeExecutionInfo> = result.executions.iter()
                    .map(|trade| TradeExecutionInfo {
                        trade_id: Uuid::parse_str(&trade.trade_id).unwrap_or_default(),
                        price: trade.price_per_unit as f64,
                        quantity: trade.energy_amount,
                        total_value: trade.total_price as f64,
                        fees: trade.grid_fee as f64,
                        counterparty: "***".to_string(),
                        timestamp: crate::utils::now().to_rfc3339(),
                    })
                    .collect();
                
                let response = PlaceOrderResponse {
                    order_id: result.order_id,
                    executions,
                    status: result.status,
                    message: "Order placed successfully".to_string(),
                };
                
                Ok(warp::reply::json(&ApiResponse::success(response)))
            }
            Err(e) => {
                Ok(warp::reply::json(&ApiResponse::<()>::error(&e.to_string())))
            }
        }
    }
    
    /// Cancel order handler
    async fn cancel_order_handler(
        request: CancelOrderRequest,
        trading_service: Arc<EnhancedTradingService>,
    ) -> Result<impl Reply, warp::Rejection> {
        match trading_service.cancel_order(request.order_id, request.account_id).await {
            Ok(()) => {
                Ok(warp::reply::json(&ApiResponse::success("Order cancelled successfully")))
            }
            Err(e) => {
                Ok(warp::reply::json(&ApiResponse::<()>::error(&e.to_string())))
            }
        }
    }
    
    /// Market depth handler
    async fn market_depth_handler(
        request: MarketDepthRequest,
        trading_service: Arc<EnhancedTradingService>,
    ) -> Result<impl Reply, warp::Rejection> {
        let levels = request.levels.unwrap_or(10);
        
        match trading_service.get_market_depth(&request.location, levels).await {
            Ok(depth) => {
                Ok(warp::reply::json(&ApiResponse::success(depth)))
            }
            Err(e) => {
                Ok(warp::reply::json(&ApiResponse::<()>::error(&e.to_string())))
            }
        }
    }
    
    /// Market data handler
    async fn market_data_handler(
        request: MarketDataRequest,
        trading_service: Arc<EnhancedTradingService>,
    ) -> Result<impl Reply, warp::Rejection> {
        match trading_service.get_market_data(&request.location).await {
            Ok(data) => {
                Ok(warp::reply::json(&ApiResponse::success(data)))
            }
            Err(e) => {
                Ok(warp::reply::json(&ApiResponse::<()>::error(&e.to_string())))
            }
        }
    }
    
    /// Trade history handler
    async fn trade_history_handler(
        request: TradeHistoryRequest,
        trading_service: Arc<EnhancedTradingService>,
    ) -> Result<impl Reply, warp::Rejection> {
        if let Some(account_id) = request.account_id {
            match trading_service.get_user_trades(&account_id).await {
                Ok(trades) => {
                    let limit = request.limit.unwrap_or(trades.len());
                    let limited_trades: Vec<_> = trades.into_iter().take(limit).collect();
                    Ok(warp::reply::json(&ApiResponse::success(limited_trades)))
                }
                Err(e) => {
                    Ok(warp::reply::json(&ApiResponse::<()>::error(&e.to_string())))
                }
            }
        } else {
            // Return general trade history
            if let Some(location) = request.location {
                match trading_service.get_recent_trades(&location, request.limit).await {
                    Ok(executions) => {
                        let trade_info: Vec<TradeExecutionInfo> = executions.iter()
                            .map(TradeExecutionInfo::from)
                            .collect();
                        Ok(warp::reply::json(&ApiResponse::success(trade_info)))
                    }
                    Err(e) => {
                        Ok(warp::reply::json(&ApiResponse::<()>::error(&e.to_string())))
                    }
                }
            } else {
                Ok(warp::reply::json(&ApiResponse::<()>::error("Either account_id or location must be provided")))
            }
        }
    }
    
    /// User orders handler
    async fn user_orders_handler(
        account_id: String,
        trading_service: Arc<EnhancedTradingService>,
    ) -> Result<impl Reply, warp::Rejection> {
        // This would typically get user's active orders
        // For now, return empty list as placeholder
        let orders: Vec<EnergyOrder> = Vec::new();
        Ok(warp::reply::json(&ApiResponse::success(orders)))
    }
    
    /// WebSocket handler for real-time updates
    async fn websocket_handler(
        websocket: warp::ws::WebSocket,
        trading_service: Arc<EnhancedTradingService>,
    ) {
        let mut event_receiver = trading_service.subscribe_to_market_events();
        let (mut ws_sender, mut ws_receiver) = websocket.split();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        
        // Spawn task to forward websocket messages
        let ws_sender_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if ws_sender.send(message).await.is_err() {
                    break;
                }
            }
        });
        
        // Spawn task to handle incoming websocket messages
        let ws_receiver_task = tokio::spawn(async move {
            while let Some(result) = ws_receiver.next().await {
                match result {
                    Ok(msg) => {
                        if msg.is_close() {
                            break;
                        }
                        // Handle incoming messages if needed
                    }
                    Err(_) => break,
                }
            }
        });
        
        // Forward order book events to websocket
        let event_forward_task = tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                let message = match serde_json::to_string(&event) {
                    Ok(json) => warp::ws::Message::text(json),
                    Err(_) => continue,
                };
                
                if tx.send(message).is_err() {
                    break;
                }
            }
        });
        
        // Wait for any task to complete
        tokio::select! {
            _ = ws_sender_task => {},
            _ = ws_receiver_task => {},
            _ = event_forward_task => {},
        }
    }
}

/// Create and start the CDA API server
pub async fn start_cda_api_server(
    trading_service: Arc<EnhancedTradingService>,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api = CDAApiService::new(trading_service);
    let routes = api.routes();
    
    // Add CORS support
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]);
    
    let routes = routes.with(cors);
    
    crate::utils::logging::log_info("CDAApiService", &format!("Starting CDA API server on port {}", port));
    
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
    
    Ok(())
}

// Re-export the futures_util trait for WebSocket handling
use futures_util::{SinkExt, StreamExt};
use warp::ws::Message;
