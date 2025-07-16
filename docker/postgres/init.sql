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

CREATE TABLE IF NOT EXISTS trading.orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id VARCHAR(66) NOT NULL,
    order_type VARCHAR(10) NOT NULL CHECK (order_type IN ('buy', 'sell')),
    energy_amount DECIMAL(20, 8) NOT NULL,
    price_per_unit DECIMAL(20, 8) NOT NULL,
    total_price DECIMAL(20, 8) NOT NULL,
    energy_source VARCHAR(20),
    grid_location JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS trading.trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    buy_order_id UUID REFERENCES trading.orders(id),
    sell_order_id UUID REFERENCES trading.orders(id),
    buyer_id VARCHAR(66) NOT NULL,
    seller_id VARCHAR(66) NOT NULL,
    energy_amount DECIMAL(20, 8) NOT NULL,
    price_per_unit DECIMAL(20, 8) NOT NULL,
    total_price DECIMAL(20, 8) NOT NULL,
    grid_fee DECIMAL(20, 8) NOT NULL,
    energy_source VARCHAR(20),
    grid_location JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
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
