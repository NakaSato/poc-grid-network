use thai_energy_trading_blockchain::utils::*;

#[test]
fn test_transaction_id_generation() {
    let id1 = generate_transaction_id();
    let id2 = generate_transaction_id();
    
    assert!(id1.starts_with("tx_"));
    assert!(id2.starts_with("tx_"));
    assert_ne!(id1, id2);
}

#[test]
fn test_order_id_generation() {
    let id1 = generate_order_id();
    let id2 = generate_order_id();
    
    assert_ne!(id1, id2);
}

#[test]
fn test_account_id_conversion() {
    let account_id = "test_account_123".to_string();
    
    let converted = account_id_to_string(&account_id);
    assert_eq!(converted, account_id);
    
    let back_converted = string_to_account_id(&converted).unwrap();
    assert_eq!(back_converted, account_id);
}

#[test]
fn test_hash_calculation() {
    let data = b"test data";
    let hash1 = calculate_hash(data);
    let hash2 = calculate_hash(data);
    
    assert_eq!(hash1, hash2);
    assert!(!hash1.is_empty());
}

#[test]
fn test_secure_random_generation() {
    let random1 = generate_secure_random(32);
    let random2 = generate_secure_random(32);
    
    assert_eq!(random1.len(), 32);
    assert_eq!(random2.len(), 32);
    assert_ne!(random1, random2);
}

#[test]
fn test_time_utilities() {
    let now = time::now();
    let timestamp = time::to_unix_timestamp(now);
    let back_converted = time::from_unix_timestamp(timestamp);
    
    assert_eq!(now.timestamp(), back_converted.timestamp());
    
    let within_seconds = time::is_within_last_seconds(now, 60);
    assert!(within_seconds);
    
    let formatted = time::format_timestamp(now);
    assert!(formatted.contains("UTC"));
}

#[test]
fn test_validation_utilities() {
    // Test email validation
    assert!(validation::is_valid_email("test@example.com"));
    assert!(!validation::is_valid_email("invalid_email"));
    
    // Test energy amount validation
    assert!(validation::is_valid_energy_amount(100));
    assert!(!validation::is_valid_energy_amount(0));
    assert!(!validation::is_valid_energy_amount(2_000_000));
    
    // Test price validation
    assert!(validation::is_valid_price(1000));
    assert!(!validation::is_valid_price(0));
    assert!(!validation::is_valid_price(200_000_000));
    
    // Test account ID validation
    assert!(validation::is_valid_account_id("valid_account"));
    
    // Test grid location validation
    let valid_location = testing::create_test_grid_location();
    assert!(validation::is_valid_grid_location(&valid_location));
}

#[test]
fn test_conversion_utilities() {
    // Test kWh to tokens conversion
    let kwh = 100;
    let tokens = conversion::kwh_to_tokens(kwh);
    assert_eq!(tokens, 100);
    
    // Test tokens to kWh conversion
    let back_kwh = conversion::tokens_to_kwh(tokens);
    assert_eq!(back_kwh, kwh);
    
    // Test total price calculation
    let total_price = conversion::calculate_total_price(100, 50);
    assert_eq!(total_price, 5000);
    
    // Test congestion level conversion
    let low_congestion = conversion::utilization_to_congestion(30.0);
    assert_eq!(low_congestion, thai_energy_trading_blockchain::CongestionLevel::Low);
    
    let high_congestion = conversion::utilization_to_congestion(85.0);
    assert_eq!(high_congestion, thai_energy_trading_blockchain::CongestionLevel::High);
}

#[test]
fn test_formatting_utilities() {
    // Test balance formatting
    let balance = 123456;
    let formatted = formatting::format_balance(balance);
    assert!(formatted.contains("THB"));
    
    // Test energy formatting
    let energy = 1000;
    let formatted = formatting::format_energy(energy);
    assert!(formatted.contains("kWh"));
    
    // Test price formatting
    let price = 50000;
    let formatted = formatting::format_price(price);
    assert!(formatted.contains("THB/kWh"));
    
    // Test percentage formatting
    let percentage = 75.5;
    let formatted = formatting::format_percentage(percentage);
    assert!(formatted.contains("%"));
    
    // Test file size formatting
    let size = 1024 * 1024; // 1MB
    let formatted = formatting::format_file_size(size);
    assert!(formatted.contains("MB"));
}

#[test]
fn test_mathematical_utilities() {
    // Test percentage calculation
    let percentage = math::calculate_percentage(25.0, 100.0);
    assert_eq!(percentage, 25.0);
    
    // Test average calculation
    let values = vec![10.0, 20.0, 30.0];
    let average = math::calculate_average(&values);
    assert_eq!(average, 20.0);
    
    // Test median calculation
    let mut values = vec![10.0, 30.0, 20.0];
    let median = math::calculate_median(&mut values);
    assert_eq!(median, 20.0);
    
    // Test standard deviation calculation
    let values = vec![10.0, 20.0, 30.0];
    let std_dev = math::calculate_standard_deviation(&values);
    assert!(std_dev > 0.0);
    
    // Test compound interest calculation
    let compound = math::calculate_compound_interest(1000.0, 0.05, 1.0, 12.0);
    assert!(compound > 1000.0);
}

#[test]
fn test_network_utilities() {
    // Test IP validation
    assert!(network::is_valid_ip("192.168.1.1"));
    assert!(network::is_valid_ip("::1"));
    assert!(!network::is_valid_ip("invalid_ip"));
    
    // Test port validation
    assert!(network::is_valid_port(8080));
    assert!(!network::is_valid_port(0));
    // Test with max u16 value (65535 is valid, but we test boundary)
    assert!(network::is_valid_port(65535));
    
    // Test network address generation
    let address = network::generate_network_address("127.0.0.1", 8080);
    assert_eq!(address, "127.0.0.1:8080");
    
    // Test IP extraction
    let ip = network::extract_ip_from_address("127.0.0.1:8080");
    assert_eq!(ip, Some("127.0.0.1".to_string()));
    
    // Test port extraction
    let port = network::extract_port_from_address("127.0.0.1:8080");
    assert_eq!(port, Some(8080));
}

#[test]
fn test_cache_utilities() {
    let mut cache = cache::Cache::new(std::time::Duration::from_secs(1));
    
    // Test cache set and get
    cache.set("key1", "value1");
    assert_eq!(cache.get(&"key1"), Some("value1"));
    
    // Test cache miss
    assert_eq!(cache.get(&"nonexistent"), None);
    
    // Test cache remove
    let removed = cache.remove(&"key1");
    assert_eq!(removed, Some("value1"));
    assert_eq!(cache.get(&"key1"), None);
    
    // Test cache clear
    cache.set("key2", "value2");
    cache.clear();
    assert_eq!(cache.get(&"key2"), None);
}

#[test]
fn test_testing_utilities() {
    // Test test account creation
    let account = testing::create_test_account_id();
    assert!(!account.is_empty());
    
    // Test test grid location creation
    let location = testing::create_test_grid_location();
    assert_eq!(location.province, "Bangkok");
    assert_eq!(location.district, "Pathum Wan");
    
    // Test test energy order creation
    let order = testing::create_test_energy_order();
    assert_eq!(order.order_type, thai_energy_trading_blockchain::OrderType::Buy);
    assert_eq!(order.energy_amount, 100.0);
}

#[test]
fn test_error_handling() {
    // Test error conversion
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let system_error = error::to_system_error(io_error);
    assert!(system_error.to_string().contains("File not found"));
}
