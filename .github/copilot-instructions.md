# GridTokenX POC Blockchain - Copilot Instructions

## Project Overview

This is a Rust blockchain library for peer-to-peer energy trading POC, featuring a **pure blockchain architecture** (no HTTP/REST APIs) with 1:1 token-energy ratio (1 kWh = 1 Token). The system uses Proof-of-Authority consensus and modular layered architecture.

## Architecture Layers (Bottom-Up)

- **Infrastructure** (`src/infrastructure/`): Database, grid integration, cloud, security
- **Blockchain** (`src/blockchain/`): Core blockchain (consensus, transactions, storage, network) 
- **Runtime** (`src/runtime/`): Token system, energy trading pallet, CDA engine, compliance
- **Application** (`src/application/`): Business logic services (trading, grid, governance, oracle)
- **Interface** (`src/interface/`): Direct blockchain interfaces (no HTTP layer)

## Key Entry Points

- **Library**: `src/lib.rs` - Main `ThaiEnergyTradingSystem` struct
- **Binary**: `src/main.rs` - Standalone application with signal handling
- **Config**: `src/config.rs` - Comprehensive system configuration
- **Types**: `src/types.rs` - Core domain types (`EnergyTrade`, `GridLocation`, etc.)

## Critical Development Patterns

### System Initialization
```rust
let config = SystemConfig::load().await?; // or ::default() for testing
let system = ThaiEnergyTradingSystem::new(config).await?;
system.start().await?;
```

### Error Handling
- Use `SystemResult<T>` and `SystemError` from `utils.rs`
- All async operations return `Result` types
- Graceful shutdown with `system.stop().await?`

### Configuration Management
- Production config: `config/production.toml` 
- Environment overrides via `.env` file
- Test mode: `SystemConfig::default()` with `test_mode: true`

## Deployment & Operations

### Docker Development
```bash
# Build and run full stack
docker-compose up -d

# Development rebuild
docker-compose build --no-cache thai-energy-blockchain
```

### Production Deployment
```bash
# Use the comprehensive deployment script
./deploy.sh
```

### Key Environment Variables
- `DATABASE_URL`: PostgreSQL connection
- `REDIS_URL`: Redis cache connection  
- `BLOCKCHAIN_NETWORK`: Network type (development/production)
- `RUST_LOG=info`: Logging level

## Testing Approach

- **Unit tests**: `cargo test`
- **Integration tests**: `tests/integration_tests.rs`
- **Examples**: `examples/` directory for usage patterns
- **Test utilities**: `utils::testing` module for test data generation

## Domain-Specific Concepts

### Energy Trading
- All energy amounts in `f64` kWh
- Token balances in `u128` (1 token = 1 kWh)
- Orders processed via Continuous Double Auction (`runtime/cda/`)
- Grid location awareness for energy routing

### Blockchain Specifics
- **Consensus**: Proof-of-Authority only (no mining)
- **Block time**: Configurable (default 30 seconds)
- **Smart contracts**: Custom VM with energy-focused operations
- **Transaction types**: Energy transfers, grid operations, governance

### Thai Grid Integration
- Province/district-based routing
- Grid location with coordinates and meter IDs
- Energy source tracking (Solar, Wind, Hydro, etc.)
- Carbon offset calculations

## Common Workflows

### Adding New Features
1. Define types in `src/types.rs`
2. Implement in appropriate layer module
3. Add configuration in `src/config.rs` 
4. Create integration tests
5. Add usage example

### Debugging Issues
- Check logs with `RUST_LOG=debug`
- Use `system.get_status().await` for component health
- Database inspection via Docker exec into postgres container
- Monitor with included Grafana/Prometheus stack

## Important Files to Reference
- `examples/energy_trading.rs` - Complete trading workflow
- `src/runtime/continuous_double_auction.rs` - Core trading logic
- `docs/SYSTEM_DOCUMENT.md` - Comprehensive system design
- `deploy.sh` - Production deployment procedures
