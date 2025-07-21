use thai_energy_trading_blockchain::*;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_energy_trade_creation() {
    let trade = EnergyTrade {
        trade_id: "test_trade_123".to_string(),
        energy_amount: 100.0,
        price_per_unit: 120_000_000_000_000_000_000u128, // 0.12 THB in wei
        buyer_id: "buyer_123".to_string(),
        seller_id: "seller_123".to_string(),
        timestamp: Utc::now().timestamp() as u64,
        status: TradeStatus::Completed,
        grid_location: utils::testing::create_test_grid_location(),
        // Legacy fields
        id: "test_trade_123".to_string(),
        buy_order_id: "buy_123".to_string(),
        sell_order_id: "sell_123".to_string(),
        price_per_kwh: 120_000_000_000_000_000_000u128,
        total_price: 12_000_000_000_000_000_000_000u128,
        grid_fee: 500_000_000_000_000_000_000u128,
        energy_source: EnergySource::Solar,
        carbon_offset: CarbonOffset {
            offset_credits: 10.0,
            verified: true,
            certification_body: "carbon_issuer".to_string(),
            timestamp: Utc::now(),
        },
    };
    
    assert_eq!(trade.trade_id, "test_trade_123");
    assert_eq!(trade.energy_amount, 100.0);
    assert_eq!(trade.status, TradeStatus::Completed);
    assert_eq!(trade.energy_source, EnergySource::Solar);
}

#[test]
fn test_energy_order_creation() {
    let order = EnergyOrder {
        id: Uuid::new_v4(),
        order_type: OrderType::Buy,
        energy_amount: 150.0,
        price_per_unit: 150_000_000_000_000_000_000u128,
        location: utils::testing::create_test_grid_location(),
        energy_source: Some(EnergySource::Wind),
        timestamp: Utc::now(),
        status: OrderStatus::Pending,
        account_id: "test_account".to_string(),
        updated_at: Utc::now(),
    };
    
    assert_eq!(order.order_type, OrderType::Buy);
    assert_eq!(order.energy_amount, 150.0);
    assert_eq!(order.energy_source, Some(EnergySource::Wind));
    assert_eq!(order.status, OrderStatus::Pending);
}

#[test]
fn test_grid_status_functionality() {
    let grid_status = GridStatus {
        region: "Bangkok".to_string(),
        current_load: 5000.0,
        max_capacity: 10000.0,
        health: 0.95,
        last_updated: Utc::now(),
        location: utils::testing::create_test_grid_location(),
        capacity: 10000.0,
        congestion_level: CongestionLevel::Low,
        stability_score: 0.9,
        outage_risk: 0.05,
        updated_at: Utc::now(),
    };
    
    // Test can_handle_load method
    assert!(grid_status.can_handle_load(2000.0)); // 5000 + 2000 = 7000 < 10000
    assert!(grid_status.can_handle_load(5000.0)); // 5000 + 5000 = 10000 = 10000
    assert!(!grid_status.can_handle_load(6000.0)); // 5000 + 6000 = 11000 > 10000
    
    assert_eq!(grid_status.congestion_level, CongestionLevel::Low);
    assert_eq!(grid_status.health, 0.95);
}

#[test]
fn test_energy_production_record() {
    let production_record = EnergyProductionRecord {
        amount: 250.0,
        energy_type: EnergySource::Solar,
        location: utils::testing::create_test_grid_location(),
        timestamp: Utc::now(),
        verified: true,
        producer_id: "producer_123".to_string(),
        equipment_id: "solar_panel_001".to_string(),
        efficiency: 0.92,
        quality_metrics: QualityMetrics {
            power_quality: 0.95,
            reliability: 0.88,
            efficiency: 0.92,
            environmental_impact: 0.1,
        },
        energy_source: EnergySource::Solar, // Compatibility field
    };
    
    assert_eq!(production_record.amount, 250.0);
    assert_eq!(production_record.energy_type, EnergySource::Solar);
    assert!(production_record.verified);
    assert_eq!(production_record.efficiency, 0.92);
}

#[test]
fn test_governance_proposal() {
    let proposal = GovernanceProposal {
        id: Uuid::new_v4(),
        title: "Increase Grid Capacity".to_string(),
        description: "Proposal to increase grid capacity by 20%".to_string(),
        proposer: "proposer_123".to_string(),
        proposal_type: ProposalType::GridUpgrade,
        voting_deadline: Utc::now() + chrono::Duration::days(7),
        minimum_voting_power: 1000,
        status: ProposalStatus::Active,
        created_at: Utc::now(),
        vote_results: VotingResults {
            yes_votes: 5000,
            no_votes: 2000,
            abstain_votes: 500,
            total_voting_power: 10000,
            participation_rate: 0.75,
            total_eligible: 100,
            turnout_percentage: 75.0,
        },
    };
    
    assert_eq!(proposal.proposal_type, ProposalType::GridUpgrade);
    assert_eq!(proposal.status, ProposalStatus::Active);
    assert_eq!(proposal.vote_results.yes_votes, 5000);
    assert_eq!(proposal.vote_results.participation_rate, 0.75);
}

#[test]
fn test_voting_results() {
    let results = VotingResults {
        yes_votes: 7500,
        no_votes: 2000,
        abstain_votes: 500,
        total_voting_power: 10000,
        participation_rate: 0.80,
        total_eligible: 125,
        turnout_percentage: 80.0,
    };
    
    assert_eq!(results.yes_votes, 7500);
    assert_eq!(results.no_votes, 2000);
    assert_eq!(results.abstain_votes, 500);
    assert_eq!(results.total_voting_power, 10000);
    assert_eq!(results.participation_rate, 0.80);
}

#[test]
fn test_market_data() {
    let market_data = MarketData {
        current_price: 5500.0,
        volume_24h: 15000.0,
        price_change_24h: 2.5,
        trades_24h: 250,
        high_24h: 6000.0,
        low_24h: 5000.0,
        timestamp: Utc::now(),
        energy_source: EnergySource::Mixed,
        location: utils::testing::create_test_grid_location(),
        price_trend: PriceTrend::Rising,
    };
    
    assert_eq!(market_data.current_price, 5500.0);
    assert_eq!(market_data.volume_24h, 15000.0);
    assert_eq!(market_data.price_change_24h, 2.5);
    assert_eq!(market_data.trades_24h, 250);
    assert_eq!(market_data.energy_source, EnergySource::Mixed);
}

#[test]
fn test_energy_token() {
    let token = EnergyToken {
        id: "token_123".to_string(),
        amount: 500,
        energy_type: EnergySource::Wind,
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::days(30),
        verified: true,
    };
    
    assert_eq!(token.amount, 500);
    assert_eq!(token.energy_type, EnergySource::Wind);
    assert!(token.verified);
}

#[test]
fn test_enum_variants() {
    // Test OrderType
    assert_eq!(OrderType::Buy, OrderType::Buy);
    assert_ne!(OrderType::Buy, OrderType::Sell);
    
    // Test OrderStatus
    assert_eq!(OrderStatus::Pending, OrderStatus::Pending);
    assert_ne!(OrderStatus::Pending, OrderStatus::Filled);
    
    // Test TradeStatus
    assert_eq!(TradeStatus::Confirmed, TradeStatus::Confirmed);
    assert_ne!(TradeStatus::Confirmed, TradeStatus::Cancelled);
    
    // Test EnergySource
    assert_eq!(EnergySource::Solar, EnergySource::Solar);
    assert_ne!(EnergySource::Solar, EnergySource::Wind);
    
    // Test CongestionLevel
    assert_eq!(CongestionLevel::Low, CongestionLevel::Low);
    assert_ne!(CongestionLevel::Low, CongestionLevel::High);
    
    // Test ProposalType
    assert_eq!(ProposalType::GridUpgrade, ProposalType::GridUpgrade);
    assert_ne!(ProposalType::GridUpgrade, ProposalType::PricingPolicy);
    
    // Test ProposalStatus
    assert_eq!(ProposalStatus::Active, ProposalStatus::Active);
    assert_ne!(ProposalStatus::Active, ProposalStatus::Passed);
    
    // Test VoteChoice
    assert_eq!(VoteChoice::Yes, VoteChoice::Yes);
    assert_ne!(VoteChoice::Yes, VoteChoice::No);
    
    // Test PriceTrend
    assert_eq!(PriceTrend::Rising, PriceTrend::Rising);
    assert_ne!(PriceTrend::Rising, PriceTrend::Falling);
}

#[test]
fn test_grid_location() {
    let location = GridLocation {
        province: "Chiang Mai".to_string(),
        district: "Muang".to_string(),
        coordinates: GridCoordinates { lat: 18.7883, lng: 98.9853 },
        region: "North".to_string(),
        substation: "CM-Central".to_string(),
        grid_code: "CM-001".to_string(),
        meter_id: "METER-CM-001".to_string(),
    };
    
    assert_eq!(location.province, "Chiang Mai");
    assert_eq!(location.district, "Muang");
    assert_eq!(location.region, "North");
    assert_eq!(location.coordinates.lat, 18.7883);
    assert_eq!(location.coordinates.lng, 98.9853);
}

#[test]
fn test_carbon_offset() {
    let carbon_offset = CarbonOffset {
        offset_credits: 25.5,
        verified: true,
        certification_body: "carbon_authority".to_string(),
        timestamp: Utc::now(),
    };
    
    assert_eq!(carbon_offset.offset_credits, 25.5);
    assert!(carbon_offset.verified);
    assert_eq!(carbon_offset.certification_body, "carbon_authority");
}

#[test]
fn test_quality_metrics() {
    let metrics = QualityMetrics {
        power_quality: 0.98,
        reliability: 0.95,
        efficiency: 0.92,
        environmental_impact: 0.08,
    };
    
    assert_eq!(metrics.power_quality, 0.98);
    assert_eq!(metrics.reliability, 0.95);
    assert_eq!(metrics.efficiency, 0.92);
    assert_eq!(metrics.environmental_impact, 0.08);
}

#[test]
fn test_balance_and_amounts() {
    let balance: Balance = 1000000;
    let energy_amount: EnergyAmount = 500.5;
    let token_price: TokenPrice = 4750.25;
    
    assert_eq!(balance, 1000000);
    assert_eq!(energy_amount, 500.5);
    assert_eq!(token_price, 4750.25);
}

#[test]
fn test_account_id_and_hash() {
    let account_id: AccountId = "account_123".to_string();
    let hash: Hash = "hash_abc123".to_string();
    
    assert_eq!(account_id, "account_123");
    assert_eq!(hash, "hash_abc123");
    assert!(!account_id.is_empty());
    assert!(!hash.is_empty());
}
