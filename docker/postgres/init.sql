-- Thai Energy Trading Blockchain Database Initialization
-- This script sets up the initial database schema for production

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";

-- Create database schemas
CREATE SCHEMA IF NOT EXISTS blockchain;
CREATE SCHEMA IF NOT EXISTS trading;
CREATE SCHEMA IF NOT EXISTS grid;
CREATE SCHEMA IF NOT EXISTS governance;
CREATE SCHEMA IF NOT EXISTS monitoring;

-- Grant permissions
GRANT USAGE ON SCHEMA blockchain TO thai_energy;
GRANT USAGE ON SCHEMA trading TO thai_energy;
GRANT USAGE ON SCHEMA grid TO thai_energy;
GRANT USAGE ON SCHEMA governance TO thai_energy;
GRANT USAGE ON SCHEMA monitoring TO thai_energy;

-- Create initial tables
CREATE TABLE IF NOT EXISTS blockchain.blocks (
    id SERIAL PRIMARY KEY,
    hash VARCHAR(64) UNIQUE NOT NULL,
    parent_hash VARCHAR(64),
    block_number BIGINT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    merkle_root VARCHAR(64) NOT NULL,
    nonce BIGINT NOT NULL,
    difficulty BIGINT NOT NULL,
    gas_limit BIGINT NOT NULL,
    gas_used BIGINT NOT NULL,
    miner VARCHAR(66) NOT NULL,
    size INTEGER NOT NULL,
    transaction_count INTEGER NOT NULL,
    state_root VARCHAR(64) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blockchain.transactions (
    id SERIAL PRIMARY KEY,
    hash VARCHAR(64) UNIQUE NOT NULL,
    block_hash VARCHAR(64) REFERENCES blockchain.blocks(hash),
    block_number BIGINT,
    transaction_index INTEGER,
    from_address VARCHAR(66) NOT NULL,
    to_address VARCHAR(66),
    value DECIMAL(78, 0) NOT NULL,
    gas_price DECIMAL(78, 0) NOT NULL,
    gas_limit BIGINT NOT NULL,
    gas_used BIGINT,
    nonce BIGINT NOT NULL,
    input_data TEXT,
    status INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Account balances table for blockchain state management
CREATE TABLE IF NOT EXISTS blockchain.account_balances (
    id SERIAL PRIMARY KEY,
    account_id VARCHAR(100) UNIQUE NOT NULL,
    total_balance DECIMAL(78, 0) NOT NULL DEFAULT 0,
    available_balance DECIMAL(78, 0) NOT NULL DEFAULT 0,
    locked_balance DECIMAL(78, 0) NOT NULL DEFAULT 0,
    nonce BIGINT NOT NULL DEFAULT 0,
    energy_balances JSONB NOT NULL DEFAULT '{}', -- Balance per energy source
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Energy production records
CREATE TABLE IF NOT EXISTS grid.energy_production (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    producer_id VARCHAR(100) NOT NULL,
    energy_amount DECIMAL(15, 6) NOT NULL CHECK (energy_amount > 0),
    energy_source VARCHAR(20) NOT NULL,
    location_json JSONB NOT NULL,
    efficiency DECIMAL(5, 4) NOT NULL DEFAULT 1.0,
    quality_score DECIMAL(5, 4) NOT NULL DEFAULT 1.0,
    equipment_id VARCHAR(100),
    verified BOOLEAN NOT NULL DEFAULT false,
    verification_authority VARCHAR(100),
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Energy consumption records
CREATE TABLE IF NOT EXISTS grid.energy_consumption (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    consumer_id VARCHAR(100) NOT NULL,
    energy_amount DECIMAL(15, 6) NOT NULL CHECK (energy_amount > 0),
    consumer_type VARCHAR(20) NOT NULL CHECK (consumer_type IN ('residential', 'commercial', 'industrial', 'agricultural', 'municipal')),
    location_json JSONB NOT NULL,
    appliance_breakdown JSONB NOT NULL DEFAULT '{}',
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Smart contracts table
CREATE TABLE IF NOT EXISTS blockchain.smart_contracts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contract_address VARCHAR(66) UNIQUE NOT NULL,
    deployer VARCHAR(66) NOT NULL,
    contract_name VARCHAR(255) NOT NULL,
    contract_code TEXT NOT NULL,
    abi JSONB NOT NULL,
    deployed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'paused', 'destroyed')),
    gas_used BIGINT NOT NULL DEFAULT 0,
    storage_rent DECIMAL(20, 8) NOT NULL DEFAULT 0
);

-- Smart contract executions
CREATE TABLE IF NOT EXISTS blockchain.contract_executions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_hash VARCHAR(64) NOT NULL,
    contract_address VARCHAR(66) NOT NULL REFERENCES blockchain.smart_contracts(contract_address),
    caller VARCHAR(66) NOT NULL,
    method_name VARCHAR(255) NOT NULL,
    input_data JSONB NOT NULL,
    output_data JSONB,
    gas_used BIGINT NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('success', 'failed', 'reverted')),
    error_message TEXT,
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Carbon credits table
CREATE TABLE IF NOT EXISTS grid.carbon_credits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    credit_id VARCHAR(100) UNIQUE NOT NULL,
    issuer VARCHAR(100) NOT NULL,
    holder VARCHAR(100) NOT NULL,
    energy_source VARCHAR(20) NOT NULL,
    energy_amount DECIMAL(15, 6) NOT NULL CHECK (energy_amount > 0),
    offset_amount DECIMAL(15, 6) NOT NULL CHECK (offset_amount > 0),
    certification_body VARCHAR(100) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT false,
    issued_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'retired', 'cancelled', 'expired')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Energy storage systems
CREATE TABLE IF NOT EXISTS grid.energy_storage (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    storage_id VARCHAR(100) UNIQUE NOT NULL,
    storage_operator VARCHAR(100) NOT NULL,
    storage_type VARCHAR(50) NOT NULL, -- battery, pumped_hydro, compressed_air, etc.
    capacity_mwh DECIMAL(15, 6) NOT NULL CHECK (capacity_mwh > 0),
    current_charge DECIMAL(15, 6) NOT NULL DEFAULT 0,
    efficiency DECIMAL(5, 4) NOT NULL DEFAULT 0.95,
    location_json JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'maintenance', 'offline')),
    installed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Energy storage operations (charge/discharge)
CREATE TABLE IF NOT EXISTS grid.storage_operations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    storage_id VARCHAR(100) NOT NULL REFERENCES grid.energy_storage(storage_id),
    operation_type VARCHAR(20) NOT NULL CHECK (operation_type IN ('charge', 'discharge')),
    energy_amount DECIMAL(15, 6) NOT NULL CHECK (energy_amount > 0),
    price_per_unit DECIMAL(15, 6) NOT NULL CHECK (price_per_unit > 0),
    grid_location VARCHAR(100) NOT NULL,
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settlement_status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (settlement_status IN ('pending', 'settled', 'failed'))
);

CREATE TABLE IF NOT EXISTS trading.orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id VARCHAR(100) NOT NULL,
    order_type VARCHAR(10) NOT NULL CHECK (order_type IN ('buy', 'sell')),
    energy_amount DECIMAL(15, 6) NOT NULL CHECK (energy_amount > 0),
    price_per_unit DECIMAL(15, 6) NOT NULL CHECK (price_per_unit > 0),
    total_value DECIMAL(15, 6) NOT NULL CHECK (total_value > 0),
    location_json JSONB NOT NULL,
    energy_source VARCHAR(20),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'partially_filled', 'filled', 'cancelled', 'expired')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS trading.trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    buy_order_id UUID NOT NULL REFERENCES trading.orders(id),
    sell_order_id UUID NOT NULL REFERENCES trading.orders(id),
    buyer_id VARCHAR(100) NOT NULL,
    seller_id VARCHAR(100) NOT NULL,
    energy_amount DECIMAL(15, 6) NOT NULL CHECK (energy_amount > 0),
    price_per_unit DECIMAL(15, 6) NOT NULL CHECK (price_per_unit > 0),
    total_value DECIMAL(15, 6) NOT NULL CHECK (total_value > 0),
    grid_fee DECIMAL(15, 6) NOT NULL DEFAULT 0.0,
    carbon_offset DECIMAL(15, 6) NOT NULL DEFAULT 0.0,
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settlement_status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (settlement_status IN ('pending', 'settled', 'failed'))
);

CREATE TABLE IF NOT EXISTS grid.locations (
    id SERIAL PRIMARY KEY,
    province VARCHAR(100) NOT NULL,
    district VARCHAR(100) NOT NULL,
    grid_code VARCHAR(20) UNIQUE NOT NULL,
    substation VARCHAR(100) NOT NULL,
    coordinates POINT NOT NULL,
    capacity_mw DECIMAL(10, 2) NOT NULL,
    current_load_mw DECIMAL(10, 2) NOT NULL,
    congestion_level VARCHAR(20) NOT NULL DEFAULT 'low',
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS grid.status_history (
    id SERIAL PRIMARY KEY,
    grid_code VARCHAR(20) NOT NULL,
    capacity DECIMAL(10, 2) NOT NULL,
    current_load DECIMAL(10, 2) NOT NULL,
    congestion_level VARCHAR(20) NOT NULL,
    stability_score DECIMAL(5, 4) NOT NULL,
    outage_risk DECIMAL(5, 4) NOT NULL,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS governance.proposals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proposer VARCHAR(66) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    proposal_type VARCHAR(50) NOT NULL,
    voting_deadline TIMESTAMP WITH TIME ZONE NOT NULL,
    execution_deadline TIMESTAMP WITH TIME ZONE,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    voting_results JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS governance.votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    proposal_id UUID REFERENCES governance.proposals(id),
    voter VARCHAR(66) NOT NULL,
    choice VARCHAR(10) NOT NULL CHECK (choice IN ('for', 'against', 'abstain')),
    voting_power DECIMAL(20, 8) NOT NULL,
    signature TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(proposal_id, voter)
);

-- Monitoring tables for system health and metrics
CREATE TABLE IF NOT EXISTS monitoring.system_metrics (
    id SERIAL PRIMARY KEY,
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(20, 8) NOT NULL,
    metric_type VARCHAR(50) NOT NULL, -- counter, gauge, histogram
    labels JSONB NOT NULL DEFAULT '{}',
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring.transaction_metrics (
    id SERIAL PRIMARY KEY,
    block_number BIGINT NOT NULL,
    transaction_count INTEGER NOT NULL DEFAULT 0,
    total_gas_used BIGINT NOT NULL DEFAULT 0,
    total_fees DECIMAL(20, 8) NOT NULL DEFAULT 0,
    energy_traded DECIMAL(15, 6) NOT NULL DEFAULT 0,
    avg_confirmation_time DECIMAL(10, 3) NOT NULL DEFAULT 0,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS monitoring.grid_health (
    id SERIAL PRIMARY KEY,
    grid_code VARCHAR(20) NOT NULL,
    health_score DECIMAL(5, 4) NOT NULL,
    stability_index DECIMAL(5, 4) NOT NULL,
    congestion_level VARCHAR(20) NOT NULL,
    active_connections INTEGER NOT NULL DEFAULT 0,
    energy_flow_in DECIMAL(15, 6) NOT NULL DEFAULT 0,
    energy_flow_out DECIMAL(15, 6) NOT NULL DEFAULT 0,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Producer and consumer registrations
CREATE TABLE IF NOT EXISTS blockchain.registered_producers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    producer_id VARCHAR(100) UNIQUE NOT NULL,
    producer_name VARCHAR(255) NOT NULL,
    producer_type VARCHAR(50) NOT NULL CHECK (producer_type IN ('residential', 'commercial', 'industrial', 'utility', 'community')),
    energy_sources TEXT[] NOT NULL, -- Array of energy sources
    location_json JSONB NOT NULL,
    capacity_kw DECIMAL(15, 6) NOT NULL CHECK (capacity_kw > 0),
    certification VARCHAR(255),
    verified BOOLEAN NOT NULL DEFAULT false,
    registered_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blockchain.registered_consumers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    consumer_id VARCHAR(100) UNIQUE NOT NULL,
    consumer_name VARCHAR(255) NOT NULL,
    consumer_type VARCHAR(50) NOT NULL CHECK (consumer_type IN ('residential', 'commercial', 'industrial', 'agricultural', 'municipal')),
    location_json JSONB NOT NULL,
    avg_consumption_kwh DECIMAL(15, 6) NOT NULL DEFAULT 0,
    peak_demand_kw DECIMAL(15, 6) NOT NULL DEFAULT 0,
    verified BOOLEAN NOT NULL DEFAULT false,
    registered_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_blocks_hash ON blockchain.blocks(hash);
CREATE INDEX IF NOT EXISTS idx_blocks_number ON blockchain.blocks(block_number);
CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON blockchain.blocks(timestamp);

CREATE INDEX IF NOT EXISTS idx_transactions_hash ON blockchain.transactions(hash);
CREATE INDEX IF NOT EXISTS idx_transactions_block_hash ON blockchain.transactions(block_hash);
CREATE INDEX IF NOT EXISTS idx_transactions_from ON blockchain.transactions(from_address);
CREATE INDEX IF NOT EXISTS idx_transactions_to ON blockchain.transactions(to_address);

CREATE INDEX IF NOT EXISTS idx_orders_account ON trading.orders(account_id);
CREATE INDEX IF NOT EXISTS idx_orders_type ON trading.orders(order_type);
CREATE INDEX IF NOT EXISTS idx_orders_status ON trading.orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created ON trading.orders(created_at);

CREATE INDEX IF NOT EXISTS idx_trades_buy_order ON trading.trades(buy_order_id);
CREATE INDEX IF NOT EXISTS idx_trades_sell_order ON trading.trades(sell_order_id);
CREATE INDEX IF NOT EXISTS idx_trades_buyer ON trading.trades(buyer_id);
CREATE INDEX IF NOT EXISTS idx_trades_seller ON trading.trades(seller_id);
CREATE INDEX IF NOT EXISTS idx_trades_executed ON trading.trades(executed_at);

CREATE INDEX IF NOT EXISTS idx_grid_locations_code ON grid.locations(grid_code);
CREATE INDEX IF NOT EXISTS idx_grid_status_code ON grid.status_history(grid_code);
CREATE INDEX IF NOT EXISTS idx_grid_status_recorded ON grid.status_history(recorded_at);

CREATE INDEX IF NOT EXISTS idx_proposals_proposer ON governance.proposals(proposer);
CREATE INDEX IF NOT EXISTS idx_proposals_status ON governance.proposals(status);
CREATE INDEX IF NOT EXISTS idx_votes_proposal ON governance.votes(proposal_id);
CREATE INDEX IF NOT EXISTS idx_votes_voter ON governance.votes(voter);

-- Additional indexes for new tables
CREATE INDEX IF NOT EXISTS idx_account_balances_account ON blockchain.account_balances(account_id);
CREATE INDEX IF NOT EXISTS idx_energy_production_producer ON grid.energy_production(producer_id);
CREATE INDEX IF NOT EXISTS idx_energy_production_timestamp ON grid.energy_production(timestamp);
CREATE INDEX IF NOT EXISTS idx_energy_consumption_consumer ON grid.energy_consumption(consumer_id);
CREATE INDEX IF NOT EXISTS idx_energy_consumption_timestamp ON grid.energy_consumption(timestamp);

CREATE INDEX IF NOT EXISTS idx_smart_contracts_address ON blockchain.smart_contracts(contract_address);
CREATE INDEX IF NOT EXISTS idx_smart_contracts_deployer ON blockchain.smart_contracts(deployer);
CREATE INDEX IF NOT EXISTS idx_contract_executions_contract ON blockchain.contract_executions(contract_address);
CREATE INDEX IF NOT EXISTS idx_contract_executions_caller ON blockchain.contract_executions(caller);

CREATE INDEX IF NOT EXISTS idx_carbon_credits_holder ON grid.carbon_credits(holder);
CREATE INDEX IF NOT EXISTS idx_carbon_credits_issuer ON grid.carbon_credits(issuer);
CREATE INDEX IF NOT EXISTS idx_carbon_credits_status ON grid.carbon_credits(status);

CREATE INDEX IF NOT EXISTS idx_energy_storage_operator ON grid.energy_storage(storage_operator);
CREATE INDEX IF NOT EXISTS idx_storage_operations_storage ON grid.storage_operations(storage_id);
CREATE INDEX IF NOT EXISTS idx_storage_operations_executed ON grid.storage_operations(executed_at);

CREATE INDEX IF NOT EXISTS idx_system_metrics_name ON monitoring.system_metrics(metric_name);
CREATE INDEX IF NOT EXISTS idx_system_metrics_recorded ON monitoring.system_metrics(recorded_at);
CREATE INDEX IF NOT EXISTS idx_transaction_metrics_block ON monitoring.transaction_metrics(block_number);
CREATE INDEX IF NOT EXISTS idx_grid_health_code ON monitoring.grid_health(grid_code);
CREATE INDEX IF NOT EXISTS idx_grid_health_recorded ON monitoring.grid_health(recorded_at);

CREATE INDEX IF NOT EXISTS idx_registered_producers_id ON blockchain.registered_producers(producer_id);
CREATE INDEX IF NOT EXISTS idx_registered_consumers_id ON blockchain.registered_consumers(consumer_id);

-- Grant table permissions
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA blockchain TO thai_energy;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA trading TO thai_energy;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA grid TO thai_energy;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA governance TO thai_energy;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA monitoring TO thai_energy;

-- Grant sequence permissions
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA blockchain TO thai_energy;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA trading TO thai_energy;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA grid TO thai_energy;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA governance TO thai_energy;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA monitoring TO thai_energy;
