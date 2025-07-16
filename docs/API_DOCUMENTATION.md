# GridTokenX Blockchain - Public API Documentation

## Oracle Gateway Bridge for Token Generation

### Version: 1.0.0
### Base URL: `https://api.gridtokenx.com/api/v1`
### Protocol: REST API over HTTPS

---

## Table of Contents

1. [Overview](#overview)
2. [Authentication](#authentication)
3. [Oracle Gateway Bridge](#oracle-gateway-bridge)
4. [Token Generation APIs](#token-generation-apis)
5. [Trading APIs](#trading-apis)
6. [Grid Management APIs](#grid-management-apis)
7. [Governance APIs](#governance-apis)
8. [Error Handling](#error-handling)
9. [Rate Limiting](#rate-limiting)
10. [Examples](#examples)

---

## Overview

The GridTokenX Public API provides access to blockchain-based energy trading functionality through the Oracle Gateway Bridge. This system enables seamless token generation, energy trading, and grid management with real-time data integration.

### Key Features

- **Oracle Gateway Bridge**: Connects external energy data sources to blockchain
- **Token Generation**: Automated token minting from verified energy production
- **Real-time Price Feeds**: Live energy pricing through oracle network
- **Grid Integration**: Direct connection to Thailand's energy grid
- **Compliance**: Full regulatory compliance for Thai energy market

### Token Economics

- **1:1 Ratio**: 1 Token = 1 kWh of energy
- **Maximum Supply**: 10,000,000 tokens
- **Energy Sources**: Solar, Wind, Hydro, Biomass, Natural Gas, Nuclear, Coal
- **Exchange Rates**: Dynamic pricing based on energy source and market conditions

---

## Authentication

All API endpoints require JWT authentication.

### Generate Authentication Token

```http
POST /auth/login
Content-Type: application/json

{
  "account_id": "your_account_id",
  "signature": "signed_message",
  "timestamp": 1721865600
}
```

### Response

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": 1721952000,
  "account_id": "your_account_id"
}
```

### Authentication Header

```http
Authorization: Bearer <jwt_token>
```

---

## Oracle Gateway Bridge

The Oracle Gateway Bridge is a sophisticated middleware system that connects external energy infrastructure to the blockchain, enabling real-time data integration, automated token generation, and seamless energy trading. It serves as the critical link between Thailand's energy grid and the decentralized blockchain network.

### Bridge Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                          Oracle Gateway Bridge Architecture                         │
├─────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                     │
│  External Energy Grid          Oracle Gateway Bridge            Blockchain Network  │
│                                                                                     │
│  ┌─────────────────┐           ┌─────────────────────┐         ┌─────────────────┐  │
│  │ Smart Meters    │──────────→│ Data Aggregation    │────────→│ Token Generation│  │
│  │ - Solar Panels  │           │ - Real-time Ingress │         │ - Automated     │  │
│  │ - Wind Turbines │           │ - Data Validation   │         │ - Verified      │  │
│  │ - Hydro Plants  │           │ - Format Conversion │         │ - Traceable     │  │
│  └─────────────────┘           └─────────────────────┘         └─────────────────┘  │
│                                                                                     │
│  ┌─────────────────┐           ┌─────────────────────┐         ┌─────────────────┐  │
│  │ Grid Monitoring │──────────→│ Oracle Consensus    │────────→│ Smart Contracts │  │
│  │ - Load Balancing│           │ - Multi-Validator   │         │ - Trading Logic │  │
│  │ - Fault Detection│          │ - Reputation System │         │ - Pricing       │  │
│  │ - Capacity Mgmt │           │ - Dispute Resolution│         │ - Settlements   │  │
│  └─────────────────┘           └─────────────────────┘         └─────────────────┘  │
│                                                                                     │
│  ┌─────────────────┐           ┌─────────────────────┐         ┌─────────────────┐  │
│  │ Weather Stations│──────────→│ Price Feed Engine   │────────→│ Governance      │  │
│  │ - Temperature   │           │ - Dynamic Pricing   │         │ - Voting        │  │
│  │ - Solar Irrad.  │           │ - Market Analysis   │         │ - Proposals     │  │
│  │ - Wind Speed    │           │ - Demand Prediction │         │ - Upgrades      │  │
│  └─────────────────┘           └─────────────────────┘         └─────────────────┘  │
│                                                                                     │
│  ┌─────────────────┐           ┌─────────────────────┐         ┌─────────────────┐  │
│  │ Regulatory APIs │──────────→│ Compliance Engine   │────────→│ Audit Trail     │  │
│  │ - Certifications│           │ - Compliance Checks │         │ - Immutable     │  │
│  │ - Permits       │           │ - Regulatory Updates│         │ - Transparent   │  │
│  │ - Standards     │           │ - Violation Alerts  │         │ - Verifiable    │  │
│  └─────────────────┘           └─────────────────────┘         └─────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

### Oracle Data Sources & Validation

#### 1. **Energy Production Sources**
- **Smart Meters**: IoT-enabled meters with cryptographic signatures
- **Solar Installations**: Photovoltaic systems with real-time monitoring
- **Wind Turbines**: SCADA systems integrated with production tracking
- **Hydroelectric Plants**: Flow meters and turbine efficiency sensors
- **Biomass Facilities**: Fuel consumption and efficiency monitoring
- **Thermal Plants**: Heat rate and emission monitoring systems

#### 2. **Grid Infrastructure Monitoring**
- **Load Dispatch Centers**: Real-time grid load and capacity data
- **Transmission Lines**: Power flow and system stability monitoring
- **Substations**: Voltage levels and transformer health data
- **Distribution Networks**: Last-mile delivery and consumption patterns
- **Emergency Systems**: Fault detection and automatic switching

#### 3. **Environmental & Weather Data**
- **Meteorological Services**: Temperature, humidity, wind patterns
- **Solar Irradiance**: Real-time solar energy potential measurement
- **Weather Forecasts**: Predictive models for renewable energy planning
- **Seasonal Patterns**: Historical data for capacity planning
- **Climate Monitoring**: Long-term environmental impact assessment

#### 4. **Market & Price Intelligence**
- **Energy Exchanges**: Real-time trading prices from multiple markets
- **Demand Forecasting**: AI-powered consumption prediction models
- **Carbon Markets**: Carbon credit pricing and trading data
- **Commodity Prices**: Fuel costs affecting energy pricing
- **Economic Indicators**: GDP, inflation, and energy policy impacts

#### 5. **Regulatory & Compliance Systems**
- **Energy Regulatory Commission**: Licensing and compliance data
- **Environmental Agencies**: Emission standards and monitoring
- **Grid Codes**: Technical standards and operational requirements
- **International Standards**: ISO 50001, IEC standards compliance
- **Audit Systems**: Third-party verification and certification

### Oracle Consensus & Validation Mechanism

#### Multi-Validator Architecture
```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Oracle Consensus Network                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  Validator Node 1        Validator Node 2        Validator Node 3          │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐        │
│  │ Data Validation │    │ Data Validation │    │ Data Validation │        │
│  │ - Source Check  │    │ - Source Check  │    │ - Source Check  │        │
│  │ - Signature Ver.│    │ - Signature Ver.│    │ - Signature Ver.│        │
│  │ - Range Check   │    │ - Range Check   │    │ - Range Check   │        │
│  └─────────────────┘    └─────────────────┘    └─────────────────┘        │
│          │                       │                       │                 │
│          └───────────────────────┼───────────────────────┘                 │
│                                  │                                         │
│                    ┌─────────────────────┐                                 │
│                    │ Consensus Algorithm │                                 │
│                    │ - 2/3 Majority Rule │                                 │
│                    │ - Weighted Voting   │                                 │
│                    │ - Reputation Score  │                                 │
│                    └─────────────────────┘                                 │
│                                  │                                         │
│                    ┌─────────────────────┐                                 │
│                    │ Blockchain Submit   │                                 │
│                    │ - Aggregated Data   │                                 │
│                    │ - Consensus Proof   │                                 │
│                    │ - Validator Sigs    │                                 │
│                    └─────────────────────┘                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Validator Selection Criteria
- **Technical Expertise**: Proven track record in energy systems
- **Infrastructure Requirements**: High-availability, low-latency systems
- **Stake Requirements**: Minimum 100,000 tokens staked
- **Reputation Score**: Community-based rating system
- **Geographic Distribution**: Validators distributed across Thailand
- **Regulatory Compliance**: Licensed energy market participants

### Data Security & Integrity

#### Cryptographic Protection
- **End-to-End Encryption**: TLS 1.3 for all data transmission
- **Digital Signatures**: Ed25519 signatures for all data sources
- **Merkle Trees**: Efficient data integrity verification
- **Zero-Knowledge Proofs**: Privacy-preserving validation
- **Hardware Security Modules**: Secure key management

#### Data Validation Pipeline
1. **Input Validation**: Schema validation and range checks
2. **Source Verification**: Cryptographic signature validation
3. **Cross-Validation**: Multi-source data correlation
4. **Anomaly Detection**: AI-powered outlier identification
5. **Consensus Verification**: Multi-validator agreement
6. **Blockchain Commitment**: Immutable data recording

---

## Token Generation APIs

### 1. Generate Tokens from Energy Production

**Endpoint**: `POST /oracle/tokens/generate`

**Description**: Generate tokens based on verified energy production data.

**Request Body**:
```json
{
  "production_record": {
    "producer_id": "producer_12345",
    "energy_source": "Solar",
    "amount_kwh": 150.5,
    "timestamp": 1721865600,
    "location": {
      "province": "Bangkok",
      "district": "Pathumwan",
      "coordinates": [13.7563, 100.5018],
      "grid_code": "BKK-001",
      "meter_id": "MTR-789123"
    },
    "validation_signatures": [
      {
        "validator_id": "validator_001",
        "signature": "0x1234567890abcdef..."
      }
    ]
  },
  "exchange_rate": 1.0,
  "carbon_credits": 12
}
```

**Response**:
```json
{
  "success": true,
  "transaction_id": "tx_a1b2c3d4e5f6",
  "tokens_generated": 150.5,
  "token_type": "Solar",
  "grid_location": {
    "province": "Bangkok",
    "district": "Pathumwan",
    "grid_code": "BKK-001"
  },
  "carbon_credits_earned": 12,
  "timestamp": 1721865600,
  "block_hash": "0xabcdef123456...",
  "gas_used": 45000
}
```

### 2. Get Token Generation Status

**Endpoint**: `GET /oracle/tokens/status/{transaction_id}`

**Response**:
```json
{
  "transaction_id": "tx_a1b2c3d4e5f6",
  "status": "confirmed",
  "tokens_generated": 150.5,
  "confirmations": 12,
  "block_number": 1250,
  "timestamp": 1721865600
}
```

### 3. Get Exchange Rates

**Endpoint**: `GET /oracle/exchange-rates`

**Response**:
```json
{
  "rates": {
    "Solar": 1.0,
    "Wind": 1.0,
    "Hydro": 1.1,
    "Biomass": 0.9,
    "NaturalGas": 0.8,
    "Nuclear": 0.95,
    "Coal": 0.7
  },
  "last_updated": 1721865600,
  "next_update": 1721866200
}
```

### 4. Validate Energy Production

**Endpoint**: `POST /oracle/validate/production`

**Request Body**:
```json
{
  "producer_id": "producer_12345",
  "energy_source": "Solar",
  "amount_kwh": 150.5,
  "timestamp": 1721865600,
  "location": {
    "province": "Bangkok",
    "district": "Pathumwan",
    "grid_code": "BKK-001",
    "meter_id": "MTR-789123"
  },
  "meter_signature": "0x987654321fedcba..."
}
```

**Response**:
```json
{
  "valid": true,
  "validation_id": "val_xyz789",
  "validator_signatures": [
    {
      "validator_id": "validator_001",
      "signature": "0x1234567890abcdef...",
      "timestamp": 1721865600
    }
  ],
  "confidence_score": 0.98,
  "ready_for_token_generation": true
}
```

---

## Trading APIs

### 1. Place Energy Order

**Endpoint**: `POST /trading/orders`

**Request Body**:
```json
{
  "order_type": "sell",
  "energy_source": "Solar",
  "amount_kwh": 100.0,
  "price_per_kwh": 3.50,
  "location": {
    "province": "Bangkok",
    "district": "Pathumwan",
    "grid_code": "BKK-001"
  },
  "expiry_time": 1721952000,
  "signature": "0xabcdef123456..."
}
```

**Response**:
```json
{
  "order_id": "order_abc123",
  "status": "pending",
  "transaction_id": "tx_def456",
  "estimated_gas": 35000,
  "order_details": {
    "type": "sell",
    "energy_source": "Solar",
    "amount_kwh": 100.0,
    "price_per_kwh": 3.50,
    "total_value": 350.0,
    "expiry_time": 1721952000
  }
}
```

### 2. Get Market Data

**Endpoint**: `GET /trading/market`

**Query Parameters**:
- `location`: Grid location (optional)
- `energy_source`: Energy source type (optional)

**Response**:
```json
{
  "market_data": {
    "current_price": 3.45,
    "24h_volume": 12500.0,
    "24h_change": 0.15,
    "buy_orders": 145,
    "sell_orders": 132,
    "last_trade": {
      "price": 3.45,
      "amount": 25.0,
      "timestamp": 1721865400
    }
  },
  "price_history": [
    {
      "timestamp": 1721865000,
      "price": 3.40,
      "volume": 150.0
    }
  ]
}
```

---

## Grid Management APIs

### 1. Get Grid Status

**Endpoint**: `GET /grid/status`

**Query Parameters**:
- `location`: Grid location code
- `include_forecast`: Include forecast data (true/false)

**Response**:
```json
{
  "grid_status": {
    "location": "BKK-001",
    "current_load": 85.2,
    "capacity": 100.0,
    "renewable_percentage": 45.8,
    "peak_demand_forecast": 92.5,
    "grid_stability": "stable",
    "energy_mix": {
      "Solar": 25.3,
      "Wind": 12.8,
      "Hydro": 7.7,
      "NaturalGas": 35.2,
      "Nuclear": 19.0
    }
  },
  "timestamp": 1721865600
}
```

### 2. Get Grid Monitoring Data

**Endpoint**: `GET /grid/monitoring`

**Response**:
```json
{
  "monitoring_data": {
    "active_producers": 1250,
    "active_consumers": 5680,
    "total_generation": 12500.0,
    "total_consumption": 11800.0,
    "grid_efficiency": 94.4,
    "carbon_emissions": 2.3,
    "renewable_contribution": 45.8,
    "grid_incidents": 0,
    "maintenance_scheduled": 2
  },
  "regional_data": [
    {
      "region": "Bangkok",
      "generation": 2500.0,
      "consumption": 2350.0,
      "efficiency": 94.0
    }
  ]
}
```

---

## Governance APIs

### 1. Get Proposals

**Endpoint**: `GET /governance/proposals`

**Query Parameters**:
- `status`: Proposal status (pending/active/completed)
- `category`: Proposal category (energy/grid/economic)

**Response**:
```json
{
  "proposals": [
    {
      "id": "prop_123",
      "title": "Increase Solar Energy Incentives",
      "description": "Proposal to increase token generation rate for solar energy",
      "category": "energy",
      "status": "active",
      "proposer": "gov_user_456",
      "created_at": 1721865600,
      "voting_deadline": 1722470400,
      "votes": {
        "for": 12500,
        "against": 3200,
        "abstain": 800
      },
      "required_threshold": 10000
    }
  ],
  "total": 5,
  "page": 1,
  "limit": 10
}
```

### 2. Submit Proposal

**Endpoint**: `POST /governance/proposals`

**Request Body**:
```json
{
  "title": "Update Grid Fee Structure",
  "description": "Proposal to modify grid transaction fees",
  "category": "economic",
  "proposal_data": {
    "new_grid_fee": 0.02,
    "effective_date": 1722470400
  },
  "stake_amount": 1000,
  "signature": "0x123456789abcdef..."
}
```

**Response**:
```json
{
  "proposal_id": "prop_789",
  "status": "submitted",
  "transaction_id": "tx_ghi789",
  "voting_period": 604800,
  "voting_deadline": 1722470400,
  "stake_locked": 1000
}
```

---

## Oracle Data APIs

### 1. Get Price Data

**Endpoint**: `GET /oracle/prices`

**Query Parameters**:
- `energy_source`: Energy source type
- `location`: Grid location
- `timeframe`: Time period (1h/24h/7d/30d)

**Response**:
```json
{
  "prices": {
    "Solar": {
      "current": 3.45,
      "24h_change": 0.15,
      "24h_high": 3.60,
      "24h_low": 3.30,
      "volume": 2500.0
    },
    "Wind": {
      "current": 3.40,
      "24h_change": 0.10,
      "24h_high": 3.55,
      "24h_low": 3.25,
      "volume": 1800.0
    }
  },
  "timestamp": 1721865600
}
```

### 2. Get Weather Data

**Endpoint**: `GET /oracle/weather`

**Query Parameters**:
- `location`: Grid location coordinates
- `forecast_hours`: Forecast period (1-72 hours)

**Response**:
```json
{
  "weather_data": {
    "current": {
      "temperature": 32.5,
      "humidity": 68,
      "wind_speed": 12.5,
      "solar_irradiance": 850,
      "precipitation": 0
    },
    "forecast": [
      {
        "timestamp": 1721869200,
        "temperature": 31.0,
        "humidity": 70,
        "wind_speed": 15.0,
        "solar_irradiance": 920,
        "precipitation": 0
      }
    ]
  },
  "location": {
    "province": "Bangkok",
    "coordinates": [13.7563, 100.5018]
  }
}
```

---

## Error Handling

### Standard Error Response

```json
{
  "error": {
    "code": "INVALID_ENERGY_SOURCE",
    "message": "The specified energy source is not supported",
    "details": {
      "supported_sources": ["Solar", "Wind", "Hydro", "Biomass", "NaturalGas", "Nuclear", "Coal"],
      "provided": "Geothermal"
    },
    "timestamp": 1721865600,
    "request_id": "req_abc123"
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_TOKEN` | 401 | Invalid or expired authentication token |
| `INSUFFICIENT_BALANCE` | 400 | Insufficient token balance for operation |
| `INVALID_ENERGY_SOURCE` | 400 | Unsupported energy source type |
| `GRID_CONGESTION` | 503 | Grid congestion preventing transaction |
| `VALIDATION_FAILED` | 400 | Energy production validation failed |
| `RATE_LIMIT_EXCEEDED` | 429 | API rate limit exceeded |
| `INTERNAL_ERROR` | 500 | Internal server error |

---

## Rate Limiting

### Limits

- **Public endpoints**: 100 requests per minute
- **Authenticated endpoints**: 1000 requests per minute
- **Token generation**: 10 requests per minute
- **Trading operations**: 100 requests per minute

### Rate Limit Headers

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1721865660
```

---

## Examples

### Complete Token Generation Flow

```bash
# 1. Authenticate
curl -X POST "https://api.gridtokenx.com/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "account_id": "producer_12345",
    "signature": "0x123456789abcdef...",
    "timestamp": 1721865600
  }'

# Response: {"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."}

# 2. Validate energy production
curl -X POST "https://api.gridtokenx.com/api/v1/oracle/validate/production" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "producer_id": "producer_12345",
    "energy_source": "Solar",
    "amount_kwh": 150.5,
    "timestamp": 1721865600,
    "location": {
      "province": "Bangkok",
      "district": "Pathumwan",
      "grid_code": "BKK-001",
      "meter_id": "MTR-789123"
    },
    "meter_signature": "0x987654321fedcba..."
  }'

# 3. Generate tokens
curl -X POST "https://api.gridtokenx.com/api/v1/oracle/tokens/generate" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "production_record": {
      "producer_id": "producer_12345",
      "energy_source": "Solar",
      "amount_kwh": 150.5,
      "timestamp": 1721865600,
      "location": {
        "province": "Bangkok",
        "district": "Pathumwan",
        "coordinates": [13.7563, 100.5018],
        "grid_code": "BKK-001",
        "meter_id": "MTR-789123"
      },
      "validation_signatures": [
        {
          "validator_id": "validator_001",
          "signature": "0x1234567890abcdef..."
        }
      ]
    },
    "exchange_rate": 1.0,
    "carbon_credits": 12
  }'

# 4. Check token generation status
curl -X GET "https://api.gridtokenx.com/api/v1/oracle/tokens/status/tx_a1b2c3d4e5f6" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### Energy Trading Example

```bash
# 1. Get current market data
curl -X GET "https://api.gridtokenx.com/api/v1/trading/market?location=BKK-001&energy_source=Solar" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

# 2. Place sell order
curl -X POST "https://api.gridtokenx.com/api/v1/trading/orders" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "order_type": "sell",
    "energy_source": "Solar",
    "amount_kwh": 100.0,
    "price_per_kwh": 3.50,
    "location": {
      "province": "Bangkok",
      "district": "Pathumwan",
      "grid_code": "BKK-001"
    },
    "expiry_time": 1721952000,
    "signature": "0xabcdef123456..."
  }'
```

---

## WebSocket API

### Real-time Data Feeds

**Endpoint**: `wss://api.gridtokenx.com/ws`

**Authentication**: Send JWT token in first message

```javascript
const ws = new WebSocket('wss://api.gridtokenx.com/ws');

ws.onopen = function() {
  // Authenticate
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
  }));
  
  // Subscribe to price feeds
  ws.send(JSON.stringify({
    type: 'subscribe',
    channels: ['prices', 'grid_status', 'token_generation']
  }));
};

ws.onmessage = function(event) {
  const data = JSON.parse(event.data);
  
  if (data.channel === 'prices') {
    console.log('Price update:', data.data);
  } else if (data.channel === 'token_generation') {
    console.log('Token generated:', data.data);
  }
};
```

---

## SDK Examples

### JavaScript SDK

```javascript
import { GridTokenXAPI } from '@gridtokenx/sdk';

const api = new GridTokenXAPI({
  baseUrl: 'https://api.gridtokenx.com',
  apiKey: 'your_api_key'
});

// Generate tokens from energy production
const result = await api.oracle.generateTokens({
  producer_id: 'producer_12345',
  energy_source: 'Solar',
  amount_kwh: 150.5,
  location: {
    province: 'Bangkok',
    grid_code: 'BKK-001'
  }
});

console.log('Tokens generated:', result.tokens_generated);
```

### Python SDK

```python
from gridtokenx import GridTokenXAPI

api = GridTokenXAPI(
    base_url='https://api.gridtokenx.com',
    api_key='your_api_key'
)

# Generate tokens from energy production
result = api.oracle.generate_tokens(
    producer_id='producer_12345',
    energy_source='Solar',
    amount_kwh=150.5,
    location={
        'province': 'Bangkok',
        'grid_code': 'BKK-001'
    }
)

print(f"Tokens generated: {result['tokens_generated']}")
```

---

## Security Considerations

### API Security

1. **TLS 1.3**: All communications encrypted
2. **JWT Tokens**: Secure authentication with expiration
3. **Rate Limiting**: Prevent abuse and DoS attacks
4. **Signature Verification**: All transactions cryptographically signed
5. **IP Whitelisting**: Optional IP restriction for sensitive operations

### Blockchain Security

1. **Consensus Mechanism**: Proof-of-Work for transaction validation
2. **Smart Contract Auditing**: All contracts formally verified
3. **Multi-signature**: Critical operations require multiple signatures
4. **Oracle Validation**: Multiple validators for data integrity

---

## Support

### Documentation
- **API Reference**: https://docs.gridtokenx.com/api
- **Developer Guide**: https://docs.gridtokenx.com/guide
- **SDK Documentation**: https://docs.gridtokenx.com/sdk

### Support Channels
- **Email**: api-support@gridtokenx.com
- **Discord**: https://discord.gg/gridtokenx
- **GitHub**: https://github.com/gridtokenx/public-api

### SLA
- **Uptime**: 99.9% guaranteed
- **Response Time**: < 200ms for most endpoints
- **Support Response**: < 24 hours for technical issues

---

*Last Updated: July 15, 2025*
*Version: 1.0.0*

---

## Oracle Gateway Bridge APIs

### 1. Data Ingestion APIs

#### Submit Energy Production Data

**Endpoint**: `POST /oracle/bridge/data/production`

**Description**: Submit real-time energy production data from external systems to the Oracle Gateway Bridge.

**Request Body**:
```json
{
  "data_source": {
    "source_id": "smart_meter_001",
    "source_type": "smart_meter",
    "location": {
      "province": "Bangkok",
      "district": "Pathumwan",
      "grid_code": "BKK-001",
      "coordinates": [13.7563, 100.5018]
    },
    "certification": {
      "cert_id": "CERT-SM-001",
      "issuer": "Thai Energy Authority",
      "valid_until": 1753401600
    }
  },
  "production_data": {
    "timestamp": 1721865600,
    "energy_source": "Solar",
    "amount_kwh": 150.5,
    "efficiency": 0.94,
    "quality_score": 0.98,
    "carbon_intensity": 0.02
  },
  "metadata": {
    "measurement_interval": 300,
    "data_quality": "high",
    "calibration_date": 1721779200,
    "firmware_version": "v2.1.3"
  },
  "cryptographic_proof": {
    "signature": "0x1234567890abcdef...",
    "public_key": "0xabcdef1234567890...",
    "signature_algorithm": "Ed25519",
    "hash": "0x9876543210fedcba..."
  }
}
```

**Response**:
```json
{
  "success": true,
  "ingestion_id": "ing_a1b2c3d4e5f6",
  "status": "accepted",
  "validation_pending": true,
  "estimated_processing_time": 30,
  "validator_nodes": [
    "validator_001",
    "validator_002",
    "validator_003"
  ],
  "quality_checks": {
    "schema_validation": "passed",
    "signature_verification": "passed",
    "range_validation": "passed",
    "anomaly_detection": "passed"
  },
  "next_steps": [
    "multi_validator_consensus",
    "blockchain_commitment",
    "token_generation"
  ]
}
```

#### Submit Grid Status Data

**Endpoint**: `POST /oracle/bridge/data/grid`

**Request Body**:
```json
{
  "data_source": {
    "source_id": "grid_monitor_bkk_001",
    "source_type": "grid_monitor",
    "authority": "Metropolitan Electricity Authority",
    "location": {
      "region": "Bangkok Metropolitan",
      "grid_code": "BKK-001",
      "voltage_level": "22kV"
    }
  },
  "grid_data": {
    "timestamp": 1721865600,
    "current_load": 85.2,
    "capacity": 100.0,
    "frequency": 50.0,
    "voltage_stability": 0.98,
    "power_factor": 0.95,
    "fault_status": "normal",
    "maintenance_mode": false
  },
  "demand_forecast": {
    "next_hour": 87.5,
    "next_4_hours": [89.2, 92.1, 88.7, 85.3],
    "peak_today": 95.8,
    "confidence_interval": 0.92
  },
  "cryptographic_proof": {
    "signature": "0x2345678901bcdef0...",
    "public_key": "0xbcdef01234567890...",
    "signature_algorithm": "Ed25519"
  }
}
```

#### Submit Weather Data

**Endpoint**: `POST /oracle/bridge/data/weather`

**Request Body**:
```json
{
  "data_source": {
    "source_id": "weather_station_001",
    "source_type": "weather_station",
    "provider": "Thai Meteorological Department",
    "location": {
      "province": "Bangkok",
      "coordinates": [13.7563, 100.5018],
      "altitude": 15.0
    }
  },
  "weather_data": {
    "timestamp": 1721865600,
    "temperature": 32.5,
    "humidity": 68,
    "wind_speed": 12.5,
    "wind_direction": 180,
    "solar_irradiance": 850,
    "precipitation": 0,
    "cloud_cover": 0.3,
    "uv_index": 8.5
  },
  "forecast_data": {
    "next_24_hours": [
      {
        "timestamp": 1721869200,
        "temperature": 31.0,
        "solar_irradiance": 920,
        "wind_speed": 15.0
      }
    ],
    "renewable_energy_potential": {
      "solar_forecast": 0.92,
      "wind_forecast": 0.75,
      "confidence": 0.88
    }
  }
}
```

### 2. Validation & Consensus APIs

#### Get Validation Status

**Endpoint**: `GET /oracle/bridge/validation/{ingestion_id}`

**Response**:
```json
{
  "ingestion_id": "ing_a1b2c3d4e5f6",
  "validation_status": "in_progress",
  "validators": [
    {
      "validator_id": "validator_001",
      "status": "validated",
      "score": 0.98,
      "timestamp": 1721865630,
      "signature": "0x3456789012cdef01..."
    },
    {
      "validator_id": "validator_002",
      "status": "validated",
      "score": 0.96,
      "timestamp": 1721865635,
      "signature": "0x456789012cdef012..."
    },
    {
      "validator_id": "validator_003",
      "status": "pending",
      "expected_completion": 1721865680
    }
  ],
  "consensus_progress": {
    "required_validators": 3,
    "completed_validations": 2,
    "consensus_threshold": 0.67,
    "current_consensus": 0.97,
    "estimated_completion": 1721865680
  },
  "validation_results": {
    "data_integrity": "passed",
    "source_authenticity": "passed",
    "range_validation": "passed",
    "cross_validation": "passed",
    "anomaly_detection": "passed"
  }
}
```

#### Trigger Manual Validation

**Endpoint**: `POST /oracle/bridge/validation/manual`

**Request Body**:
```json
{
  "ingestion_id": "ing_a1b2c3d4e5f6",
  "validator_id": "validator_004",
  "reason": "additional_verification_requested",
  "priority": "high",
  "signature": "0x567890123def0123..."
}
```

#### Get Consensus Results

**Endpoint**: `GET /oracle/bridge/consensus/{ingestion_id}`

**Response**:
```json
{
  "ingestion_id": "ing_a1b2c3d4e5f6",
  "consensus_status": "reached",
  "final_consensus_score": 0.97,
  "participating_validators": 3,
  "consensus_timestamp": 1721865680,
  "aggregated_data": {
    "energy_source": "Solar",
    "verified_amount_kwh": 150.5,
    "quality_score": 0.98,
    "confidence_interval": [150.2, 150.8]
  },
  "validator_signatures": [
    {
      "validator_id": "validator_001",
      "signature": "0x3456789012cdef01...",
      "weight": 0.33
    },
    {
      "validator_id": "validator_002",
      "signature": "0x456789012cdef012...",
      "weight": 0.33
    },
    {
      "validator_id": "validator_003",
      "signature": "0x567890123def0123...",
      "weight": 0.34
    }
  ],
  "blockchain_commitment": {
    "transaction_id": "tx_blockchain_001",
    "block_number": 1251,
    "commitment_hash": "0x789012345def0123..."
  }
}
```

### 3. Oracle Network Management APIs

#### Get Oracle Network Status

**Endpoint**: `GET /oracle/bridge/network/status`

**Response**:
```json
{
  "network_health": {
    "status": "healthy",
    "uptime": 0.999,
    "last_downtime": 1721779200,
    "total_validators": 25,
    "active_validators": 24,
    "inactive_validators": 1
  },
  "data_throughput": {
    "current_tps": 150.5,
    "peak_tps": 250.0,
    "average_tps": 125.0,
    "total_transactions_24h": 10800000
  },
  "consensus_metrics": {
    "average_consensus_time": 28.5,
    "success_rate": 0.998,
    "disputed_transactions": 5,
    "resolved_disputes": 5
  },
  "data_sources": {
    "total_registered": 1250,
    "active_sources": 1180,
    "smart_meters": 850,
    "grid_monitors": 180,
    "weather_stations": 120,
    "regulatory_apis": 30
  }
}
```

#### Get Validator Information

**Endpoint**: `GET /oracle/bridge/validators`

**Response**:
```json
{
  "validators": [
    {
      "validator_id": "validator_001",
      "name": "Bangkok Energy Validator",
      "operator": "MEA Validator Services",
      "status": "active",
      "reputation_score": 0.98,
      "stake_amount": 150000,
      "location": {
        "province": "Bangkok",
        "grid_region": "Central"
      },
      "performance_metrics": {
        "uptime": 0.999,
        "response_time": 25.0,
        "validation_accuracy": 0.997,
        "disputes_resolved": 15
      },
      "specializations": [
        "smart_meter_validation",
        "grid_monitoring",
        "renewable_energy"
      ]
    }
  ],
  "validator_selection": {
    "current_rotation": "round_robin",
    "next_rotation": 1721869200,
    "minimum_validators": 3,
    "maximum_validators": 7
  }
}
```

### 4. Data Quality & Monitoring APIs

#### Get Data Quality Metrics

**Endpoint**: `GET /oracle/bridge/quality/metrics`

**Query Parameters**:
- `time_range`: Time period (1h/24h/7d/30d)
- `data_source`: Specific data source type
- `location`: Grid location filter

**Response**:
```json
{
  "quality_metrics": {
    "overall_quality_score": 0.96,
    "data_completeness": 0.98,
    "data_accuracy": 0.95,
    "data_timeliness": 0.97,
    "data_consistency": 0.94
  },
  "source_quality": {
    "smart_meters": {
      "quality_score": 0.97,
      "error_rate": 0.003,
      "missing_data_rate": 0.02,
      "latency_avg": 15.5
    },
    "grid_monitors": {
      "quality_score": 0.95,
      "error_rate": 0.005,
      "missing_data_rate": 0.01,
      "latency_avg": 12.0
    },
    "weather_stations": {
      "quality_score": 0.94,
      "error_rate": 0.006,
      "missing_data_rate": 0.03,
      "latency_avg": 18.2
    }
  },
  "anomaly_detection": {
    "anomalies_detected": 25,
    "false_positives": 2,
    "accuracy": 0.92,
    "top_anomaly_types": [
      "unusual_production_spike",
      "meter_calibration_drift",
      "weather_data_inconsistency"
    ]
  }
}
```

#### Get Real-time Data Streams

**Endpoint**: `GET /oracle/bridge/streams/realtime`

**Response**:
```json
{
  "active_streams": [
    {
      "stream_id": "stream_production_001",
      "data_type": "energy_production",
      "source_count": 850,
      "update_frequency": 300,
      "last_update": 1721865600,
      "quality_score": 0.97,
      "throughput": 2.83
    },
    {
      "stream_id": "stream_grid_001",
      "data_type": "grid_status",
      "source_count": 180,
      "update_frequency": 60,
      "last_update": 1721865580,
      "quality_score": 0.95,
      "throughput": 3.0
    }
  ],
  "aggregated_metrics": {
    "total_data_points": 1250000,
    "processing_rate": 4167.0,
    "error_rate": 0.004,
    "latency_p95": 28.5
  }
}
```

### 5. Bridge Configuration APIs

#### Update Bridge Configuration

**Endpoint**: `PUT /oracle/bridge/config`

**Request Body**:
```json
{
  "consensus_parameters": {
    "minimum_validators": 3,
    "consensus_threshold": 0.67,
    "timeout_seconds": 60,
    "retry_attempts": 3
  },
  "data_validation": {
    "enable_anomaly_detection": true,
    "anomaly_threshold": 0.95,
    "cross_validation_required": true,
    "signature_verification": true
  },
  "performance_settings": {
    "max_throughput": 1000,
    "batch_size": 50,
    "processing_timeout": 30,
    "queue_size": 10000
  },
  "security_settings": {
    "encryption_algorithm": "AES-256-GCM",
    "signature_algorithm": "Ed25519",
    "key_rotation_interval": 86400,
    "access_control": "rbac"
  }
}
```

#### Get Bridge Configuration

**Endpoint**: `GET /oracle/bridge/config`

**Response**:
```json
{
  "current_configuration": {
    "version": "2.1.0",
    "last_updated": 1721865600,
    "consensus_parameters": {
      "minimum_validators": 3,
      "consensus_threshold": 0.67,
      "timeout_seconds": 60,
      "retry_attempts": 3
    },
    "data_validation": {
      "enable_anomaly_detection": true,
      "anomaly_threshold": 0.95,
      "cross_validation_required": true,
      "signature_verification": true
    },
    "performance_settings": {
      "max_throughput": 1000,
      "batch_size": 50,
      "processing_timeout": 30,
      "queue_size": 10000
    }
  },
  "configuration_history": [
    {
      "version": "2.0.0",
      "changed_at": 1721779200,
      "changes": ["increased_consensus_threshold", "enabled_anomaly_detection"]
    }
  ]
}
```

### 6. Oracle Bridge Analytics APIs

#### Get Bridge Analytics Dashboard

**Endpoint**: `GET /oracle/bridge/analytics/dashboard`

**Response**:
```json
{
  "dashboard_data": {
    "overview": {
      "total_data_ingested": 50000000,
      "tokens_generated": 1250000,
      "active_data_sources": 1180,
      "uptime_percentage": 99.9
    },
    "performance_metrics": {
      "avg_processing_time": 28.5,
      "throughput_tps": 150.5,
      "error_rate": 0.004,
      "success_rate": 0.996
    },
    "data_breakdown": {
      "energy_production": {
        "percentage": 65.0,
        "volume": 32500000,
        "quality_score": 0.97
      },
      "grid_monitoring": {
        "percentage": 25.0,
        "volume": 12500000,
        "quality_score": 0.95
      },
      "weather_data": {
        "percentage": 10.0,
        "volume": 5000000,
        "quality_score": 0.94
      }
    },
    "geographical_distribution": {
      "Bangkok": 35.0,
      "Central": 25.0,
      "Northeast": 20.0,
      "North": 12.0,
      "South": 8.0
    }
  }
}
```

### 7. Advanced Oracle Bridge Features

#### Dispute Resolution System

**Endpoint**: `POST /oracle/bridge/disputes/create`

**Description**: Create a dispute for questionable validator decisions or data inconsistencies.

**Request Body**:
```json
{
  "dispute_type": "data_validation_error",
  "target_validation": {
    "ingestion_id": "ing_a1b2c3d4e5f6",
    "validator_id": "validator_002",
    "validation_timestamp": 1721865635
  },
  "evidence": {
    "description": "Validator approved production data outside normal range",
    "supporting_data": [
      {
        "type": "historical_comparison",
        "data": "Previous 30-day average: 120 kWh, Current: 150.5 kWh (+25%)",
        "source": "internal_analytics"
      },
      {
        "type": "cross_validator_disagreement",
        "data": "2 out of 3 validators flagged as anomalous",
        "source": "consensus_logs"
      }
    ],
    "attachments": [
      {
        "type": "data_log",
        "hash": "0x123456789abcdef...",
        "description": "Raw meter data for disputed period"
      }
    ]
  },
  "stake_amount": 10000,
  "disputant_id": "disputant_001",
  "signature": "0x789012345def0123..."
}
```

**Response**:
```json
{
  "dispute_id": "dispute_001",
  "status": "created",
  "stake_locked": 10000,
  "resolution_period": 604800,
  "assigned_arbitrators": [
    "arbitrator_001",
    "arbitrator_002",
    "arbitrator_003"
  ],
  "evidence_period_ends": 1721952000,
  "voting_period_starts": 1721952000,
  "expected_resolution": 1722470400
}
```

#### Machine Learning Integration

**Endpoint**: `POST /oracle/bridge/ml/train`

**Description**: Train machine learning models for improved data validation and anomaly detection.

**Request Body**:
```json
{
  "model_type": "anomaly_detection",
  "training_data": {
    "data_source": "historical_production",
    "time_range": {
      "start": 1719273600,
      "end": 1721865600
    },
    "features": [
      "energy_amount",
      "time_of_day",
      "weather_conditions",
      "grid_load",
      "seasonal_patterns"
    ]
  },
  "model_parameters": {
    "algorithm": "isolation_forest",
    "contamination_rate": 0.05,
    "random_state": 42,
    "n_estimators": 100
  },
  "validation_split": 0.2,
  "cross_validation_folds": 5
}
```

**Response**:
```json
{
  "training_job_id": "ml_job_001",
  "status": "started",
  "estimated_completion": 1721867400,
  "training_metrics": {
    "dataset_size": 1000000,
    "feature_count": 5,
    "training_samples": 800000,
    "validation_samples": 200000
  },
  "model_version": "v2.1.0"
}
```

#### Get ML Model Performance

**Endpoint**: `GET /oracle/bridge/ml/performance/{model_id}`

**Response**:
```json
{
  "model_id": "ml_job_001",
  "model_type": "anomaly_detection",
  "version": "v2.1.0",
  "performance_metrics": {
    "accuracy": 0.94,
    "precision": 0.92,
    "recall": 0.89,
    "f1_score": 0.90,
    "false_positive_rate": 0.03,
    "false_negative_rate": 0.11
  },
  "training_results": {
    "training_accuracy": 0.95,
    "validation_accuracy": 0.94,
    "overfitting_score": 0.01,
    "convergence_epochs": 45
  },
  "deployment_status": {
    "status": "deployed",
    "deployment_time": 1721867400,
    "production_performance": {
      "anomalies_detected": 125,
      "false_positives": 4,
      "accuracy_in_production": 0.93
    }
  }
}
```

### 8. Oracle Bridge Integration Examples

#### Smart Meter Integration SDK

**Java Example**:
```java
import com.gridtokenx.oracle.SmartMeterClient;
import com.gridtokenx.oracle.model.ProductionData;
import com.gridtokenx.oracle.model.CryptographicProof;

public class SmartMeterIntegration {
    private SmartMeterClient client;
    
    public SmartMeterIntegration() {
        this.client = new SmartMeterClient.Builder()
            .baseUrl("https://api.gridtokenx.com")
            .certificate("path/to/certificate.pem")
            .privateKey("path/to/private.key")
            .build();
    }
    
    public void submitProductionData(double amountKwh, String energySource) {
        ProductionData data = ProductionData.builder()
            .timestamp(System.currentTimeMillis() / 1000)
            .energySource(energySource)
            .amountKwh(amountKwh)
            .efficiency(0.94)
            .qualityScore(0.98)
            .build();
            
        CryptographicProof proof = client.createProof(data);
        
        IngestionResponse response = client.submitProductionData(
            DataSource.builder()
                .sourceId("smart_meter_001")
                .sourceType("smart_meter")
                .certification(getCertification())
                .build(),
            data,
            proof
        );
        
        if (response.isSuccess()) {
            System.out.println("Data submitted successfully: " + response.getIngestionId());
            monitorValidation(response.getIngestionId());
        }
    }
    
    private void monitorValidation(String ingestionId) {
        // Poll validation status
        ValidationStatus status = client.getValidationStatus(ingestionId);
        while (status.isInProgress()) {
            try {
                Thread.sleep(5000);
                status = client.getValidationStatus(ingestionId);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                break;
            }
        }
        
        if (status.isCompleted()) {
            System.out.println("Validation completed with score: " + status.getConsensusScore());
        }
    }
}
```

#### Grid Monitor Integration

**Python Example**:
```python
import asyncio
import aiohttp
from gridtokenx_oracle import GridMonitorClient
from cryptography.hazmat.primitives.asymmetric import ed25519

class GridMonitorIntegration:
    def __init__(self):
        self.client = GridMonitorClient(
            base_url="https://api.gridtokenx.com",
            cert_path="path/to/certificate.pem",
            private_key_path="path/to/private.key"
        )
        self.running = False
    
    async def start_monitoring(self):
        self.running = True
        
        while self.running:
            try:
                grid_data = await self.collect_grid_data()
                await self.submit_grid_data(grid_data)
                await asyncio.sleep(60)  # Submit every minute
            except Exception as e:
                print(f"Error in monitoring loop: {e}")
                await asyncio.sleep(10)
    
    async def collect_grid_data(self):
        # Collect from actual grid monitoring systems
        return {
            "timestamp": int(time.time()),
            "current_load": 85.2,
            "capacity": 100.0,
            "frequency": 50.0,
            "voltage_stability": 0.98,
            "power_factor": 0.95,
            "fault_status": "normal"
        }
    
    async def submit_grid_data(self, grid_data):
        data_source = {
            "source_id": "grid_monitor_bkk_001",
            "source_type": "grid_monitor",
            "authority": "Metropolitan Electricity Authority",
            "location": {
                "region": "Bangkok Metropolitan",
                "grid_code": "BKK-001",
                "voltage_level": "22kV"
            }
        }
        
        # Create cryptographic proof
        proof = self.client.create_proof(grid_data)
        
        response = await self.client.submit_grid_data(
            data_source=data_source,
            grid_data=grid_data,
            proof=proof
        )
        
        if response.success:
            print(f"Grid data submitted: {response.ingestion_id}")
            await self.monitor_validation(response.ingestion_id)
    
    async def monitor_validation(self, ingestion_id):
        for attempt in range(12):  # Wait up to 2 minutes
            status = await self.client.get_validation_status(ingestion_id)
            
            if status.validation_status == "completed":
                print(f"Validation completed: {status.consensus_progress.current_consensus}")
                break
            elif status.validation_status == "failed":
                print(f"Validation failed: {status.validation_results}")
                break
            
            await asyncio.sleep(10)

# Usage
async def main():
    monitor = GridMonitorIntegration()
    await monitor.start_monitoring()

if __name__ == "__main__":
    asyncio.run(main())
```

#### Weather Station Integration

**Node.js Example**:
```javascript
const { WeatherStationClient } = require('@gridtokenx/oracle-sdk');
const crypto = require('crypto');

class WeatherStationIntegration {
    constructor() {
        this.client = new WeatherStationClient({
            baseUrl: 'https://api.gridtokenx.com',
            certificatePath: 'path/to/certificate.pem',
            privateKeyPath: 'path/to/private.key'
        });
        this.isRunning = false;
    }
    
    async startDataCollection() {
        this.isRunning = true;
        
        while (this.isRunning) {
            try {
                const weatherData = await this.collectWeatherData();
                await this.submitWeatherData(weatherData);
                await this.sleep(300000); // Every 5 minutes
            } catch (error) {
                console.error('Error in weather data collection:', error);
                await this.sleep(60000); // Wait 1 minute before retry
            }
        }
    }
    
    async collectWeatherData() {
        // Simulate weather station data collection
        return {
            timestamp: Math.floor(Date.now() / 1000),
            temperature: 32.5,
            humidity: 68,
            wind_speed: 12.5,
            wind_direction: 180,
            solar_irradiance: 850,
            precipitation: 0,
            cloud_cover: 0.3,
            uv_index: 8.5
        };
    }
    
    async submitWeatherData(weatherData) {
        const dataSource = {
            source_id: 'weather_station_001',
            source_type: 'weather_station',
            provider: 'Thai Meteorological Department',
            location: {
                province: 'Bangkok',
                coordinates: [13.7563, 100.5018],
                altitude: 15.0
            }
        };
        
        // Generate forecast data
        const forecastData = await this.generateForecast(weatherData);
        
        const cryptographicProof = this.client.createProof(weatherData);
        
        const response = await this.client.submitWeatherData({
            data_source: dataSource,
            weather_data: weatherData,
            forecast_data: forecastData,
            cryptographic_proof: cryptographicProof
        });
        
        if (response.success) {
            console.log(`Weather data submitted: ${response.ingestion_id}`);
            this.monitorValidation(response.ingestion_id);
        } else {
            console.error('Failed to submit weather data:', response.error);
        }
    }
    
    async generateForecast(currentData) {
        // Simple forecast generation (in production, use meteorological models)
        const forecast = [];
        
        for (let i = 1; i <= 24; i++) {
            const futureTime = Math.floor(Date.now() / 1000) + (i * 3600);
            forecast.push({
                timestamp: futureTime,
                temperature: currentData.temperature + (Math.random() - 0.5) * 4,
                solar_irradiance: Math.max(0, currentData.solar_irradiance + (Math.random() - 0.5) * 200),
                wind_speed: Math.max(0, currentData.wind_speed + (Math.random() - 0.5) * 5)
            });
        }
        
        return {
            next_24_hours: forecast,
            renewable_energy_potential: {
                solar_forecast: 0.92,
                wind_forecast: 0.75,
                confidence: 0.88
            }
        };
    }
    
    async monitorValidation(ingestionId) {
        const maxAttempts = 12;
        let attempts = 0;
        
        while (attempts < maxAttempts) {
            try {
                const status = await this.client.getValidationStatus(ingestionId);
                
                if (status.validation_status === 'completed') {
                    console.log(`Validation completed with score: ${status.consensus_progress.current_consensus}`);
                    break;
                } else if (status.validation_status === 'failed') {
                    console.error('Validation failed:', status.validation_results);
                    break;
                }
                
                await this.sleep(10000); // Wait 10 seconds
                attempts++;
            } catch (error) {
                console.error('Error checking validation status:', error);
                break;
            }
        }
    }
    
    sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
    
    stop() {
        this.isRunning = false;
    }
}

// Usage
const weatherStation = new WeatherStationIntegration();
weatherStation.startDataCollection().catch(console.error);

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('Shutting down weather station integration...');
    weatherStation.stop();
    process.exit(0);
});
```

### 9. Oracle Bridge Monitoring & Alerting

#### Real-time Monitoring Dashboard

**Endpoint**: `GET /oracle/bridge/monitoring/realtime`

**Response**:
```json
{
  "system_health": {
    "status": "healthy",
    "uptime": 99.9,
    "current_load": 75.2,
    "memory_usage": 68.5,
    "cpu_usage": 45.2,
    "disk_usage": 32.1
  },
  "data_pipeline": {
    "ingestion_rate": 150.5,
    "processing_queue": 125,
    "validation_queue": 45,
    "consensus_queue": 12,
    "error_queue": 2
  },
  "validator_network": {
    "active_validators": 24,
    "consensus_performance": {
      "avg_time": 28.5,
      "success_rate": 99.8,
      "current_disputes": 1
    }
  },
  "alerts": [
    {
      "id": "alert_001",
      "level": "warning",
      "message": "High processing queue detected",
      "timestamp": 1721865600,
      "auto_resolve": true
    }
  ]
}
```

#### Alert Configuration

**Endpoint**: `POST /oracle/bridge/alerts/config`

**Request Body**:
```json
{
  "alert_rules": [
    {
      "name": "high_processing_queue",
      "condition": "processing_queue > 1000",
      "severity": "warning",
      "notification_channels": ["email", "slack", "webhook"],
      "auto_resolve": true,
      "cooldown_period": 300
    },
    {
      "name": "validator_offline",
      "condition": "active_validators < 20",
      "severity": "critical",
      "notification_channels": ["email", "pager", "webhook"],
      "auto_resolve": false,
      "cooldown_period": 60
    },
    {
      "name": "consensus_failure",
      "condition": "consensus_success_rate < 0.95",
      "severity": "critical",
      "notification_channels": ["email", "pager", "slack"],
      "auto_resolve": false,
      "cooldown_period": 120
    }
  ],
  "notification_settings": {
    "email": {
      "recipients": ["admin@gridtokenx.com", "ops@gridtokenx.com"],
      "smtp_server": "smtp.gridtokenx.com"
    },
    "slack": {
      "webhook_url": "https://hooks.slack.com/services/...",
      "channel": "#oracle-alerts"
    },
    "webhook": {
      "url": "https://monitoring.gridtokenx.com/alerts",
      "method": "POST",
      "headers": {
        "Authorization": "Bearer token123"
      }
    }
  }
}
```

### 10. Oracle Bridge Performance Optimization

#### Batch Processing Configuration

**Endpoint**: `POST /oracle/bridge/batch/config`

**Request Body**:
```json
{
  "batch_settings": {
    "max_batch_size": 100,
    "max_wait_time": 30,
    "priority_queue": true,
    "compression": "gzip",
    "parallel_processing": true,
    "worker_threads": 8
  },
  "data_type_configs": {
    "energy_production": {
      "batch_size": 50,
      "wait_time": 15,
      "priority": 1
    },
    "grid_monitoring": {
      "batch_size": 25,
      "wait_time": 10,
      "priority": 2
    },
    "weather_data": {
      "batch_size": 100,
      "wait_time": 60,
      "priority": 3
    }
  }
}
```

#### Performance Metrics

**Endpoint**: `GET /oracle/bridge/performance/metrics`

**Response**:
```json
{
  "performance_metrics": {
    "throughput": {
      "current_tps": 150.5,
      "peak_tps": 250.0,
      "avg_tps_24h": 125.0,
      "total_transactions": 50000000
    },
    "latency": {
      "avg_processing_time": 28.5,
      "p50_processing_time": 25.0,
      "p95_processing_time": 45.0,
      "p99_processing_time": 65.0
    },
    "resource_utilization": {
      "cpu_usage": 45.2,
      "memory_usage": 68.5,
      "disk_io": 32.1,
      "network_io": 28.7
    },
    "error_rates": {
      "total_error_rate": 0.004,
      "validation_error_rate": 0.002,
      "consensus_error_rate": 0.001,
      "network_error_rate": 0.001
    }
  },
  "optimization_recommendations": [
    {
      "type": "batch_size_increase",
      "description": "Increase batch size for weather data processing",
      "expected_improvement": "15% throughput increase",
      "implementation_effort": "low"
    },
    {
      "type": "worker_scaling",
      "description": "Add 2 additional worker threads",
      "expected_improvement": "25% processing speed increase",
      "implementation_effort": "medium"
    }
  ]
}
```
