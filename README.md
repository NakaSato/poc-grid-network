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
