# GridTokenX API Bridge Documentation

## Overview

The API Bridge provides HTTP access to the GridTokenX blockchain system, enabling public access to blockchain functionality through REST endpoints.

## Quick Start

### 1. Basic Usage

```rust
use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem, 
    SystemConfig,
    bridge::{ApiBridge, BridgeConfig}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create system configuration
    let system_config = SystemConfig::default();
    
    // Create blockchain system
    let mut blockchain_system = ThaiEnergyTradingSystem::new(system_config).await?;
    
    // Create API bridge configuration
    let bridge_config = BridgeConfig {
        port: 8081, // Use different port if 8080 is busy
        host: "0.0.0.0".to_string(),
        cors_origins: vec!["*".to_string()],
        debug: true,
    };
    
    // Enable and start API bridge
    blockchain_system.enable_api_bridge(bridge_config).await?;
    blockchain_system.start_api_bridge().await?;
    
    Ok(())
}
```

### 2. Configuration Options

The `BridgeConfig` struct supports the following options:

- `port`: Server port (default: 8080)
- `host`: Server host (default: "0.0.0.0")
- `cors_origins`: CORS allowed origins (default: ["*"])
- `debug`: Enable debug mode (default: false)

## Available Endpoints

### Health Check
- **GET** `/health`
- Returns server health status

**Example Response:**
```json
{
    "status": "healthy",
    "service": "GridTokenX API Bridge",
    "version": "1.0.0"
}
```

### System Status
- **GET** `/api/v1/system/status`
- Returns blockchain system status

**Example Response:**
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

### Market Data
- **GET** `/api/v1/trading/market`
- Returns current market data and trading information

**Example Response:**
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

## Testing with Docker Deployment

When using with the existing Docker deployment (running on port 8080), use a different port:

1. **Start Docker stack** (if not already running):
   ```bash
   docker-compose up -d
   ```

2. **Run API bridge on different port**:
   ```bash
   # In your Rust code, use port 8081 instead of 8080
   let bridge_config = BridgeConfig {
       port: 8081,
       // ... other config
   };
   ```

3. **Test endpoints**:
   ```bash
   # Health check
   curl http://localhost:8081/health
   
   # System status
   curl http://localhost:8081/api/v1/system/status
   
   # Market data
   curl http://localhost:8081/api/v1/trading/market
   ```

## Integration with Docker

The API bridge can be integrated with the existing Docker deployment:

### Option 1: Separate Bridge Service
Add the bridge as a separate service in `docker-compose.yml`:

```yaml
services:
  api-bridge:
    build: .
    ports:
      - "8081:8081"
    environment:
      - BRIDGE_PORT=8081
      - BRIDGE_HOST=0.0.0.0
    depends_on:
      - blockchain
      - postgres
      - redis
```

### Option 2: Built-in Bridge
Enable the bridge in the main blockchain service:

```yaml
services:
  blockchain:
    # ... existing config
    ports:
      - "3030:3030"
      - "8081:8081"  # Add bridge port
    environment:
      - ENABLE_API_BRIDGE=true
      - API_BRIDGE_PORT=8081
```

## CORS Configuration

For web applications, configure CORS properly:

```rust
let bridge_config = BridgeConfig {
    port: 8081,
    host: "0.0.0.0".to_string(),
    cors_origins: vec![
        "http://localhost:3000".to_string(),  // React dev server
        "https://yourdomain.com".to_string(),  // Production frontend
    ],
    debug: false,
};
```

## Error Handling

The API bridge includes error handling and logging:

- Requests are logged with the `api_bridge` target
- Errors return appropriate HTTP status codes
- CORS is enabled for all endpoints
- JSON responses are consistent

## Performance Considerations

- The bridge is lightweight and uses async/await for performance
- Concurrent requests are handled efficiently
- Consider rate limiting for production deployments
- Monitor memory usage with high request volumes

## Security Notes

- Configure CORS origins appropriately for production
- Consider implementing authentication for sensitive endpoints
- Use HTTPS in production environments
- Monitor for abuse and implement rate limiting as needed

## Future Enhancements

The API bridge currently provides basic endpoints. Future versions may include:

- Authentication and authorization
- WebSocket support for real-time updates
- More comprehensive trading endpoints
- Admin endpoints for system management
- Rate limiting and quota management
- Metrics and monitoring endpoints
