# GridTokenX API Bridge

The **API Bridge** is a REST/WebSocket server that provides public HTTP access to the GridTokenX blockchain. This bridge enables web applications, mobile apps, and external systems to interact with the blockchain without requiring direct integration.

## Quick Start

### 1. Start with API Bridge

```bash
# Run the API bridge example
cargo run --example api_bridge
```

The server will start on:
- **HTTP API**: `http://localhost:8080`
- **WebSocket**: `ws://localhost:8081/ws`
- **Health Check**: `http://localhost:8080/health`

### 2. Test the API

```bash
# Health check
curl http://localhost:8080/health

# System status
curl http://localhost:8080/api/v1/system/status

# Market data
curl http://localhost:8080/api/v1/trading/market

# Place an energy order
curl -X POST http://localhost:8080/api/v1/trading/orders \
  -H "Content-Type: application/json" \
  -d '{
    "order_type": "buy",
    "energy_amount": 100.0,
    "price_per_unit": 150000,
    "location": {
      "province": "Bangkok",
      "district": "Chatuchak",
      "coordinates": {"lat": 13.8, "lng": 100.55}
    },
    "account_id": "user123"
  }'
```

## API Endpoints

### System Endpoints
- `GET /health` - Service health check
- `GET /api/v1/system/status` - Blockchain system status

### Trading Endpoints
- `POST /api/v1/trading/orders` - Place energy order
- `GET /api/v1/trading/orders` - List orders
- `GET /api/v1/trading/orders/:id` - Get order details
- `POST /api/v1/trading/orders/:id/cancel` - Cancel order
- `GET /api/v1/trading/market` - Market data & order book

### Token Endpoints
- `POST /api/v1/tokens/transfer` - Transfer tokens between accounts

### Governance Endpoints
- `POST /api/v1/governance/proposals` - Create governance proposal
- `GET /api/v1/governance/proposals` - List proposals
- `POST /api/v1/governance/vote` - Cast vote on proposal

### Blockchain Endpoints
- `GET /api/v1/blockchain/blocks` - List recent blocks
- `GET /api/v1/blockchain/blocks/:height` - Get block by height
- `GET /api/v1/blockchain/transactions/:hash` - Get transaction details

### Analytics Endpoints
- `GET /api/v1/analytics/tps` - TPS metrics and performance
- `GET /api/v1/analytics/volume` - Trading volume statistics

## WebSocket Channels

Connect to `ws://localhost:8081/ws` and subscribe to real-time updates:

### Available Channels
- `orderbook` - Order book updates
- `trades` - Trade execution notifications
- `blocks` - New block notifications
- `market_data` - Market data updates
- `system_status` - System status changes

### WebSocket Message Format

**Subscribe to channels:**
```json
{
  "type": "Subscribe",
  "channels": ["orderbook", "trades", "blocks"]
}
```

**Unsubscribe from channels:**
```json
{
  "type": "Unsubscribe", 
  "channels": ["orderbook"]
}
```

**Receive updates:**
```json
{
  "type": "TradeUpdate",
  "channel": "trades",
  "data": {
    "trade_id": "uuid",
    "price": 145000,
    "amount": 50.0,
    "side": "buy",
    "timestamp": "2024-01-01T12:00:00Z"
  },
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Configuration

### Bridge Configuration
```rust
use thai_energy_trading_blockchain::bridge::BridgeConfig;

let bridge_config = BridgeConfig {
    host: "0.0.0.0".to_string(),        // Bind address
    port: 8080,                         // HTTP port
    ws_port: 8081,                      // WebSocket port  
    enable_cors: true,                  // CORS support
    rate_limit: Some(100),              // Requests per minute
    api_key_required: false,            // API key authentication
    max_connections: 1000,              // Max WebSocket connections
    request_timeout_seconds: 30,        // Request timeout
    enable_metrics: true,               // Metrics collection
};
```

### Security Features

1. **Rate Limiting**: Token bucket algorithm (configurable per-IP limits)
2. **CORS**: Cross-origin resource sharing support
3. **Request Timeouts**: Configurable timeout protection  
4. **API Key Authentication**: Optional API key validation
5. **Security Headers**: Standard HTTP security headers
6. **Request Size Limits**: JSON body size limits (32KB)

### Middleware Stack

- **Rate Limiting**: Per-IP request throttling
- **CORS**: Cross-origin support
- **Logging**: Request/response logging
- **Error Handling**: Standardized error responses
- **Security Headers**: XSS, content-type protection
- **Request Tracing**: UUID-based request tracking

## Response Format

All API responses follow a consistent JSON format:

### Success Response
```json
{
  "success": true,
  "data": { /* response data */ },
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### Error Response  
```json
{
  "success": false,
  "error": {
    "code": 400,
    "message": "Invalid request",
    "details": "Additional error information"
  },
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Integration Examples

### JavaScript/TypeScript
```javascript
// HTTP API example
const response = await fetch('http://localhost:8080/api/v1/trading/market');
const marketData = await response.json();

// WebSocket example
const ws = new WebSocket('ws://localhost:8081/ws');
ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'Subscribe',
    channels: ['trades', 'blocks']
  }));
};
ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  console.log('Received update:', update);
};
```

### Python
```python
import requests
import websocket
import json

# HTTP API
response = requests.get('http://localhost:8080/api/v1/system/status')
print(response.json())

# WebSocket
def on_message(ws, message):
    data = json.loads(message)
    print("Received:", data)

ws = websocket.WebSocketApp("ws://localhost:8081/ws", on_message=on_message)
ws.run_forever()
```

### cURL Examples
```bash
# Place energy order
curl -X POST http://localhost:8080/api/v1/trading/orders \
  -H "Content-Type: application/json" \
  -d '{
    "order_type": "sell", 
    "energy_amount": 50.0,
    "price_per_unit": 140000,
    "energy_source": "solar",
    "location": {
      "province": "Chiang Mai",
      "district": "Mueang", 
      "coordinates": {"lat": 18.8, "lng": 98.98}
    },
    "account_id": "producer456"
  }'

# Get trading volume
curl "http://localhost:8080/api/v1/analytics/volume?timeframe=24h&location=Bangkok"

# Create governance proposal  
curl -X POST http://localhost:8080/api/v1/governance/proposals \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Reduce block time to 15 seconds",
    "description": "Proposal to improve transaction throughput",
    "proposal_type": "protocol_change",
    "proposer": "validator1",
    "voting_period_days": 7
  }'
```

## Docker Integration

The API bridge works seamlessly with the existing Docker deployment:

```bash
# Start the full stack with API bridge
docker-compose up -d

# The API will be available at:
# HTTP: http://localhost:8080  
# WebSocket: ws://localhost:8081/ws
```

## Monitoring & Metrics

The API bridge provides built-in monitoring:

- **Request Metrics**: Request count, response times, error rates
- **WebSocket Metrics**: Active connections, message rates  
- **Rate Limiting**: Per-IP request tracking
- **System Health**: Service health checks

Access metrics at:
- `GET /api/v1/middleware/health` - Middleware health status
- `GET /api/v1/analytics/tps` - Transaction performance metrics

## Development

### Enable API Bridge in Code
```rust
use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem,
    SystemConfig, 
    bridge::BridgeConfig
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize blockchain system
    let mut system = ThaiEnergyTradingSystem::new(SystemConfig::default()).await?;
    
    // Configure and enable API bridge
    let bridge_config = BridgeConfig::default();
    system.enable_api_bridge(bridge_config).await?;
    
    // Start blockchain
    system.start().await?;
    
    // Start API bridge server
    system.start_api_bridge().await?;
    
    Ok(())
}
```

### Custom Middleware
The bridge supports custom middleware for authentication, logging, and request processing. See `src/bridge/middleware.rs` for implementation details.

## Production Deployment

For production use:

1. **Enable API Key Authentication**:
   ```rust
   let bridge_config = BridgeConfig {
       api_key_required: true,
       // Configure valid API keys
       ..Default::default()
   };
   ```

2. **Configure Rate Limiting**:
   ```rust
   let bridge_config = BridgeConfig {
       rate_limit: Some(1000), // 1000 requests per minute
       ..Default::default()
   };
   ```

3. **Use HTTPS**: Configure reverse proxy (nginx) with SSL certificates

4. **Monitor Resources**: Set appropriate connection limits and timeouts

The API bridge provides a production-ready interface for integrating GridTokenX blockchain with web applications, mobile apps, and external systems.
