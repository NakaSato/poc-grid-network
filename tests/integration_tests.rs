use thai_energy_trading_blockchain::*;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_system_creation() {
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await;
    assert!(system.is_ok());
}

#[tokio::test]
async fn test_system_lifecycle() {
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
    
    // Test start
    let start_result = system.start().await;
    assert!(start_result.is_ok());
    
    // Test stop
    let stop_result = system.stop().await;
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_system_components() {
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
    
    // Test blockchain component
    let blockchain_status = system.get_blockchain_status().await;
    assert!(blockchain_status.is_ok());
    
    // Test trading component
    let trading_status = system.get_trading_status().await;
    assert!(trading_status.is_ok());
}

#[tokio::test]
async fn test_energy_trading_operations() {
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
    
    // Create test order
    let test_order = thai_energy_trading_blockchain::utils::testing::create_test_energy_order();
    
    // Test order placement
    let place_result = system.place_energy_order(&test_order).await;
    assert!(place_result.is_ok());
    
    // Test order cancellation
    let cancel_result = system.cancel_energy_order(&test_order.id, &test_order.account_id).await;
    assert!(cancel_result.is_ok());
}

#[tokio::test]
async fn test_grid_management() {
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
    
    let test_location = thai_energy_trading_blockchain::utils::testing::create_test_grid_location();
    
    // Test grid status
    let grid_status = system.get_grid_status(&test_location).await;
    assert!(grid_status.is_ok());
    
    // Test load validation
    let load_result = system.validate_grid_load(&test_location, 1000.0).await;
    assert!(load_result.is_ok());
}

#[tokio::test]
async fn test_governance_operations() {
    let config = SystemConfig::default();
    let system = ThaiEnergyTradingSystem::new(config).await.unwrap();
    
    let test_account = thai_energy_trading_blockchain::utils::testing::create_test_account_id();
    
    // Test proposal creation
    let proposal = GovernanceProposal {
        id: Uuid::new_v4(),
        title: "Test Proposal".to_string(),
        description: "Test governance proposal".to_string(),
        proposer: test_account.clone(),
        proposal_type: ProposalType::GridUpgrade,
        voting_deadline: Utc::now() + chrono::Duration::days(7),
        minimum_voting_power: 1000,
        status: ProposalStatus::Active,
        created_at: Utc::now(),
        vote_results: VotingResults {
            yes_votes: 0,
            no_votes: 0,
            abstain_votes: 0,
            total_voting_power: 0,
            participation_rate: 0.0,
            total_eligible: 0,
            turnout_percentage: 0.0,
        },
    };
    
    let create_result = system.create_governance_proposal(&proposal).await;
    assert!(create_result.is_ok());
    
    // Test voting
    let vote_result = system.vote_on_proposal(&proposal.id, &test_account, VoteChoice::Yes, 1000).await;
    assert!(vote_result.is_ok());
}
