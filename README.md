# GridTokenX POC Blockchain Library

A Rust library for building blockchain-based energy trading platforms, enabling peer-to-peer energy trading with a 1:1 token-energy ratio - Proof of Concept.

## Features

- **Pure Blockchain Architecture**: Direct blockchain interfaces without HTTP/REST API layers
- **Energy-Focused**: Specialized for energy trading with 1 kWh = 1 Token ratio
- **Modular Design**: Clean separation of concerns with layered architecture
- **Grid Integration**: Built for energy grid infrastructure integration
- **Governance System**: Decentralized governance for network decisions
- **Oracle Integration**: Real-time price feeds and weather data
- **Smart Contracts**: Energy-focused smart contract execution environment

## Architecture

The library is structured in layers:

- **Interface Layer**: Direct blockchain interactions (no HTTP)
- **Application Layer**: Business logic (Trading, Grid, Governance, Oracle)
- **Runtime Layer**: Blockchain runtime (Token system, Energy trading, Compliance)
- **Blockchain Layer**: Core blockchain (Consensus, Transaction pool, Storage, Network)
- **Infrastructure Layer**: External integrations (Database, Grid, Cloud, Security)

## Usage

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
thai-energy-trading-blockchain = "0.0.1"
```

### Basic Usage

```rust
use thai_energy_trading_blockchain::{ThaiEnergyTradingSystem, SystemConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = SystemConfig::load().await?;
    
    // Create and start the system
    let system = ThaiEnergyTradingSystem::new(config).await?;
    system.start().await?;
    
    // Use the system...
    let trading_service = system.trading_service();
    let blockchain_interface = system.blockchain_interface();
    
    // Stop the system
    system.stop().await?;
    
    Ok(())
}
```

### Using Individual Components

```rust
use thai_energy_trading_blockchain::{
    application::trading::TradingService,
    blockchain::node::BlockchainNode,
    config::SystemConfig,
    types::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SystemConfig::default();
    
    // Use individual components
    let blockchain_node = BlockchainNode::new(&config.blockchain).await?;
    let trading_service = TradingService::new_placeholder().await?;
    
    // Start services
    blockchain_node.start().await?;
    trading_service.start().await?;
    
    // Use services...
    
    Ok(())
}
```

## Running as a Binary

The library also provides a binary executable:

```bash
# Build the binary
cargo build --release

# Run the binary
./target/release/thai-energy-trading-blockchain

# Or run directly with cargo
cargo run --bin thai-energy-trading-blockchain
```

## Configuration

The system can be configured through environment variables:

```bash
# Database configuration
export DATABASE_URL="postgresql://user:password@localhost/thai_energy_trading"
export REDIS_URL="redis://localhost:6379"

# Blockchain configuration
export BLOCKCHAIN_NETWORK="development"
export BLOCKCHAIN_ENABLE_MINING="true"
export BLOCKCHAIN_MINING_DIFFICULTY="4"

# Grid configuration
export GRID_API_URL="https://api.grid.thailand.gov.th"
export GRID_API_KEY="your_api_key"

# Security configuration
export JWT_SECRET="your_jwt_secret_key"
export RATE_LIMIT_REQUESTS_PER_MINUTE="60"
```

## Examples

### Energy Trading

```rust
use thai_energy_trading_blockchain::{
    types::{EnergyOrder, OrderType, EnergySource, GridLocation},
    ThaiEnergyTradingSystem,
    SystemConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = ThaiEnergyTradingSystem::new(SystemConfig::default()).await?;
    system.start().await?;
    
    let trading_service = system.trading_service();
    
    // Create a buy order for solar energy
    let order = EnergyOrder {
        id: uuid::Uuid::new_v4(),
        account_id: "buyer_account".to_string(),
        order_type: OrderType::Buy,
        energy_source: Some(EnergySource::Solar),
        amount: 100.0, // 100 kWh
        price_per_kwh: 3.5, // 3.5 tokens per kWh
        location: GridLocation {
            province: "Bangkok".to_string(),
            district: "Pathum Wan".to_string(),
            substation: "Siam".to_string(),
        },
        // ... other fields
    };
    
    let order_id = trading_service.place_order(order).await?;
    println!("Order placed with ID: {}", order_id);
    
    system.stop().await?;
    Ok(())
}
```

### Blockchain Interaction

```rust
use thai_energy_trading_blockchain::{
    ThaiEnergyTradingSystem,
    SystemConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = ThaiEnergyTradingSystem::new(SystemConfig::default()).await?;
    system.start().await?;
    
    let blockchain_interface = system.blockchain_interface();
    
    // Get blockchain status
    let status = blockchain_interface.get_blockchain_status().await?;
    println!("Blockchain status: {:?}", status);
    
    system.stop().await?;
    Ok(())
}
```

## Development

### Building

```bash
# Build the library
cargo build

# Build with release optimizations
cargo build --release

# Build only the library (no binary)
cargo build --lib
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_system_creation
```

### Documentation

```bash
# Generate documentation
cargo doc

# Generate and open documentation
cargo doc --open
```

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Support

For questions and support, please open an issue on the GitHub repository.
