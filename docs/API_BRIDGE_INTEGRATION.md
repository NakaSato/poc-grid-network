# GridTokenX API Bridge Integration Guide

## Overview

The GridTokenX API Bridge provides a comprehensive HTTP/REST interface for backend applications to interact with the blockchain. This guide covers the bridge architecture, available endpoints, integration patterns, and best practices for backend API development.

## Architecture Overview

```
┌─────────────────┐    HTTP/REST     ┌─────────────────┐    Direct Calls    ┌──────────────────┐
│   Backend API   │ ────────────────▶│   API Bridge    │ ──────────────────▶│   Blockchain     │
│   Applications  │                  │   (mod.rs)      │                    │   System         │
└─────────────────┘                  └─────────────────┘                    └──────────────────┘
                                             │
                                             ▼
                                     ┌─────────────────┐
                                     │   Services      │
                                     │  - Trading      │
                                     │  - Grid         │
                                     │  - Governance   │
                                     │  - Oracle       │
                                     └─────────────────┘
```

## Bridge Components

### 1. Core Bridge Module (`src/bridge/mod.rs`)

The main API bridge provides:
- **HTTP Server**: Warp-based async web server
- **CORS Support**: Cross-origin resource sharing for web applications
- **JSON APIs**: RESTful endpoints for blockchain interaction
- **Error Handling**: Consistent error responses
- **Configuration**: Flexible server configuration

### 2. Bridge Configuration

```rust
pub struct BridgeConfig {
    pub port: u16,                    // Server port (default: 8080)
    pub host: String,                 // Server host (default: "0.0.0.0")
    pub cors_origins: Vec<String>,    // CORS allowed origins
    pub debug: bool,                  // Enable debug mode
}
```

### 3. Available Endpoints

#### Health Check
- **Endpoint**: `GET /health`
- **Purpose**: Server health monitoring
- **Response**:
```json
{
  "status": "healthy",
  "service": "GridTokenX API Bridge",
  "version": "1.0.0"
}
```

#### System Status
- **Endpoint**: `GET /api/v1/system/status`
- **Purpose**: Blockchain system status
- **Response**:
```json
{
  "blockchain_status": "running",
  "consensus": "proof_of_authority",
  "network_id": "thai_energy_grid",
  "connected_nodes": 1,
  "current_block": 0,
  "gas_price": "0.001"
}
```

#### Market Data
- **Endpoint**: `GET /api/v1/trading/market`
- **Purpose**: Current market data and trading information
- **Response**:
```json
{
  "markets": [
    {
      "pair": "ETH/THB",
      "price": "85000.00",
      "volume_24h": "1250.50",
      "change_24h": "+2.5%"
    },
    {
      "pair": "ENERGY/THB",
      "price": "12.50",
      "volume_24h": "8750.25",
      "change_24h": "+1.2%"
    }
  ],
  "total_volume_24h": "10000.75",
  "active_orders": 145,
  "recent_trades": 89
}
```

## Backend API Integration Patterns

### 1. Direct Bridge Integration

For backend applications that need direct blockchain access:

```rust
use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem,
    SystemConfig,
    bridge::{ApiBridge, BridgeConfig}
};

// Initialize blockchain system
let system_config = SystemConfig::default();
let mut blockchain_system = ThaiEnergyTradingSystem::new(system_config).await?;

// Configure API bridge
let bridge_config = BridgeConfig {
    port: 8081,
    host: "0.0.0.0".to_string(),
    cors_origins: vec!["*".to_string()],
    debug: true,
};

// Enable and start bridge
blockchain_system.enable_api_bridge(bridge_config).await?;
blockchain_system.start_api_bridge().await?;
```

### 2. Microservices Architecture

For microservices that need to interact with the blockchain:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Trading API   │    │   Grid API      │    │  Analytics API  │
│   Service       │    │   Service       │    │   Service       │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          │              HTTP/REST Calls                │
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                         ┌───────▼──────┐
                         │  API Bridge  │
                         │  (Port 8081) │
                         └───────┬──────┘
                                 │
                         ┌───────▼──────┐
                         │  Blockchain  │
                         │   System     │
                         └──────────────┘
```

### 3. Proxy Pattern

For existing APIs that need blockchain integration:

```rust
// Example: Express.js backend with blockchain integration
const axios = require('axios');

const BLOCKCHAIN_API_BASE = 'http://localhost:8081/api/v1';

// Health check endpoint
app.get('/health', async (req, res) => {
    try {
        const response = await axios.get(`${BLOCKCHAIN_API_BASE}/../health`);
        res.json({
            api_status: 'healthy',
            blockchain_status: response.data.status,
            timestamp: new Date().toISOString()
        });
    } catch (error) {
        res.status(500).json({ error: 'Blockchain unavailable' });
    }
});

// Market data proxy
app.get('/api/market', async (req, res) => {
    try {
        const response = await axios.get(`${BLOCKCHAIN_API_BASE}/trading/market`);
        res.json(response.data);
    } catch (error) {
        res.status(500).json({ error: 'Failed to fetch market data' });
    }
});
```

## Extended API Development

### 1. Trading API Extensions

Create comprehensive trading endpoints by extending the bridge:

```rust
// In your bridge module, add trading routes
let trading_routes = warp::path("api")
    .and(warp::path("v1"))
    .and(warp::path("trading"))
    .and(
        // Place order
        warp::path("orders")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::place_order_handler)
        .or(
            // Get orders
            warp::path("orders")
                .and(warp::get())
                .and(warp::query::<OrderQuery>())
                .and_then(Self::get_orders_handler)
        )
        .or(
            // Cancel order
            warp::path!("orders" / String / "cancel")
                .and(warp::delete())
                .and_then(Self::cancel_order_handler)
        )
    );
```

### 2. Authentication & Authorization

Add security middleware to your bridge:

```rust
// JWT Authentication middleware
fn with_auth() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>("authorization")
        .and_then(|token: String| async move {
            if token.starts_with("Bearer ") {
                let jwt_token = &token[7..];
                // Validate JWT token here
                match validate_jwt_token(jwt_token).await {
                    Ok(user_id) => Ok(user_id),
                    Err(_) => Err(warp::reject::custom(UnauthorizedError))
                }
            } else {
                Err(warp::reject::custom(UnauthorizedError))
            }
        })
}

// Apply to protected routes
let protected_routes = trading_routes
    .and(with_auth())
    .and_then(|user_id: String| async move {
        // Handle authenticated request
    });
```

### 3. Request/Response Models

Define comprehensive data models for API interactions:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    pub order_type: OrderType,
    pub energy_amount: f64,
    pub price_per_unit: u128,
    pub location: GridLocation,
    pub energy_source: Option<EnergySource>,
    pub time_in_force: Option<String>, // "GTC", "IOC", "FOK"
    pub post_only: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PlaceOrderResponse {
    pub order_id: String,
    pub status: String,
    pub executions: Vec<TradeExecution>,
    pub remaining_quantity: f64,
    pub fees: u128,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
```

## WebSocket Integration

For real-time updates, extend the bridge with WebSocket support:

```rust
// WebSocket endpoint for real-time updates
let websocket = warp::path("ws")
    .and(warp::ws())
    .and(with_blockchain_system())
    .map(|ws: warp::ws::Ws, system: Arc<ThaiEnergyTradingSystem>| {
        ws.on_upgrade(move |socket| handle_websocket(socket, system))
    });

async fn handle_websocket(
    websocket: WebSocket, 
    system: Arc<ThaiEnergyTradingSystem>
) {
    let (mut ws_sender, mut ws_receiver) = websocket.split();
    
    // Subscribe to blockchain events
    let mut event_stream = system.subscribe_to_events().await;
    
    // Handle incoming messages
    let ws_receiver_task = tokio::spawn(async move {
        while let Some(result) = ws_receiver.next().await {
            if let Ok(msg) = result {
                // Handle client messages (subscribe/unsubscribe)
                handle_ws_message(msg).await;
            }
        }
    });
    
    // Send blockchain events to client
    let ws_sender_task = tokio::spawn(async move {
        while let Ok(event) = event_stream.recv().await {
            let message = serde_json::to_string(&event).unwrap();
            if ws_sender.send(Message::text(message)).await.is_err() {
                break;
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = ws_receiver_task => {},
        _ = ws_sender_task => {},
    }
}
```

## Database Integration

Integrate with external databases for enhanced functionality:

```rust
use sqlx::{PgPool, Row};

pub struct DatabaseBridge {
    pool: PgPool,
    blockchain_system: Arc<ThaiEnergyTradingSystem>,
}

impl DatabaseBridge {
    // Store order in database
    pub async fn store_order(&self, order: &EnergyOrder) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO orders (id, order_type, energy_amount, price, location, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            order.id,
            order.order_type as _,
            order.energy_amount,
            order.price_per_unit as i64,
            serde_json::to_string(&order.location).unwrap(),
            "active"
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // Get order history
    pub async fn get_order_history(&self, account_id: &str) -> Result<Vec<OrderRecord>, sqlx::Error> {
        let records = sqlx::query_as!(
            OrderRecord,
            "SELECT * FROM orders WHERE account_id = $1 ORDER BY created_at DESC LIMIT 100",
            account_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(records)
    }
}
```

## Monitoring & Metrics

Add comprehensive monitoring to your API bridge:

```rust
use prometheus::{Counter, Histogram, Gauge};

pub struct BridgeMetrics {
    pub requests_total: Counter,
    pub request_duration: Histogram,
    pub active_connections: Gauge,
    pub blockchain_status: Gauge,
}

impl BridgeMetrics {
    pub fn new() -> Self {
        Self {
            requests_total: Counter::new("api_requests_total", "Total API requests").unwrap(),
            request_duration: Histogram::new("api_request_duration_seconds", "API request duration").unwrap(),
            active_connections: Gauge::new("api_active_connections", "Active connections").unwrap(),
            blockchain_status: Gauge::new("blockchain_status", "Blockchain status").unwrap(),
        }
    }
}

// Middleware for metrics collection
fn with_metrics(metrics: Arc<BridgeMetrics>) -> impl Filter<Extract = (), Error = std::convert::Infallible> + Clone {
    warp::any()
        .and_then(move || {
            let metrics = metrics.clone();
            async move {
                metrics.requests_total.inc();
                Ok(())
            }
        })
        .untuple_one()
}
```

## Testing Your API Bridge

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_bridge_creation() {
        let config = SystemConfig::default();
        let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
        let bridge_config = BridgeConfig::default();
        
        let bridge = ApiBridge::new(Arc::new(system), bridge_config).await;
        assert!(bridge.is_ok());
    }
    
    #[tokio::test]
    async fn test_health_endpoint() {
        // Setup test server
        let system = setup_test_system().await;
        let bridge = ApiBridge::new(Arc::new(system), BridgeConfig::default()).await.unwrap();
        
        // Test health endpoint
        let response = warp::test::request()
            .method("GET")
            .path("/health")
            .reply(&bridge.create_routes())
            .await;
            
        assert_eq!(response.status(), 200);
    }
}
```

### 2. Integration Tests

```bash
# Test health endpoint
curl -X GET http://localhost:8081/health

# Test system status
curl -X GET http://localhost:8081/api/v1/system/status

# Test market data
curl -X GET http://localhost:8081/api/v1/trading/market

# Test with authentication
curl -X GET http://localhost:8081/api/v1/orders \
  -H "Authorization: Bearer <your-jwt-token>"
```

### 3. Load Testing

```bash
# Using Apache Bench
ab -n 1000 -c 10 http://localhost:8081/health

# Using wrk
wrk -t12 -c400 -d30s http://localhost:8081/api/v1/system/status
```

## Production Deployment

### 1. Docker Integration

Add the bridge to your `docker-compose.yml`:

```yaml
services:
  api-bridge:
    build: .
    command: ["cargo", "run", "--example", "api_bridge_example"]
    ports:
      - "8081:8081"
    environment:
      - RUST_LOG=info
      - BRIDGE_PORT=8081
      - BRIDGE_HOST=0.0.0.0
    depends_on:
      - blockchain
      - postgres
      - redis
    networks:
      - blockchain-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - api-bridge
```

### 2. Nginx Configuration

```nginx
upstream api_bridge {
    server api-bridge:8081;
}

server {
    listen 443 ssl http2;
    server_name api.gridtokenx.com;

    ssl_certificate /etc/nginx/ssl/cert.pem;
    ssl_certificate_key /etc/nginx/ssl/key.pem;

    location /api/ {
        proxy_pass http://api_bridge;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    location /health {
        proxy_pass http://api_bridge;
    }
}
```

### 3. Environment Configuration

```bash
# Production environment variables
RUST_LOG=info
BRIDGE_PORT=8081
BRIDGE_HOST=0.0.0.0
BRIDGE_CORS_ORIGINS=https://app.gridtokenx.com,https://admin.gridtokenx.com
BRIDGE_DEBUG=false
DATABASE_URL=postgresql://user:pass@postgres:5432/gridtokenx
REDIS_URL=redis://redis:6379
JWT_SECRET=your-jwt-secret
```

## Best Practices

### 1. Error Handling

- Use consistent error response formats
- Log all errors with appropriate context
- Return appropriate HTTP status codes
- Never expose internal system details in error messages

### 2. Security

- Always validate input data
- Use authentication for sensitive endpoints
- Implement rate limiting
- Enable HTTPS in production
- Validate CORS origins properly

### 3. Performance

- Use connection pooling for databases
- Implement caching strategies
- Monitor response times
- Use async/await throughout
- Optimize database queries

### 4. Monitoring

- Track API usage metrics
- Monitor blockchain connectivity
- Set up alerts for failures
- Log important business events
- Use structured logging

## Advanced Features

### 1. Rate Limiting

```rust
use tokio::time::{Duration, Instant};
use std::collections::HashMap;

pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, (u32, Instant)>>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub async fn check_rate_limit(&self, client_id: &str) -> bool {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        match requests.get_mut(client_id) {
            Some((count, last_reset)) => {
                if now.duration_since(*last_reset) > self.window_duration {
                    *count = 1;
                    *last_reset = now;
                    true
                } else if *count < self.max_requests {
                    *count += 1;
                    true
                } else {
                    false
                }
            }
            None => {
                requests.insert(client_id.to_string(), (1, now));
                true
            }
        }
    }
}
```

### 2. Circuit Breaker

```rust
use std::sync::atomic::{AtomicU32, AtomicBool, Ordering};

pub struct CircuitBreaker {
    failure_count: AtomicU32,
    failure_threshold: u32,
    is_open: AtomicBool,
}

impl CircuitBreaker {
    pub fn call<T, E>(&self, operation: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
        if self.is_open.load(Ordering::Acquire) {
            return Err(/* CircuitOpenError */);
        }
        
        match operation() {
            Ok(result) => {
                self.failure_count.store(0, Ordering::Release);
                Ok(result)
            }
            Err(error) => {
                let failures = self.failure_count.fetch_add(1, Ordering::AcqRel);
                if failures >= self.failure_threshold {
                    self.is_open.store(true, Ordering::Release);
                }
                Err(error)
            }
        }
    }
}
```

This comprehensive guide provides everything needed to integrate backend APIs with the GridTokenX blockchain through the API bridge, from basic setup to advanced production features.
