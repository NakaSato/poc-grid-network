-- Initial schema for trading database
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS postgis;

-- Grid schema
CREATE SCHEMA IF NOT EXISTS grid;

-- Trading schema  
CREATE SCHEMA IF NOT EXISTS trading;

-- Grid locations table
CREATE TABLE IF NOT EXISTS grid.locations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    province VARCHAR(100) NOT NULL,
    district VARCHAR(100) NOT NULL,
    grid_code VARCHAR(50) NOT NULL UNIQUE,
    coordinates POINT NOT NULL,
    capacity_kw DECIMAL(10,2) NOT NULL DEFAULT 0.0,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Orders table
CREATE TABLE IF NOT EXISTS trading.orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id VARCHAR(100) NOT NULL,
    order_type VARCHAR(10) NOT NULL CHECK (order_type IN ('buy', 'sell')),
    energy_amount DECIMAL(15,6) NOT NULL CHECK (energy_amount > 0),
    price_per_unit DECIMAL(15,6) NOT NULL CHECK (price_per_unit > 0),
    total_value DECIMAL(15,6) NOT NULL CHECK (total_value > 0),
    location_json JSONB NOT NULL,
    energy_source VARCHAR(20),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'partially_filled', 'filled', 'cancelled', 'expired')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE
);

-- Trades table  
CREATE TABLE IF NOT EXISTS trading.trades (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    buy_order_id UUID NOT NULL REFERENCES trading.orders(id),
    sell_order_id UUID NOT NULL REFERENCES trading.orders(id),
    energy_amount DECIMAL(15,6) NOT NULL CHECK (energy_amount > 0),
    price_per_unit DECIMAL(15,6) NOT NULL CHECK (price_per_unit > 0),
    total_value DECIMAL(15,6) NOT NULL CHECK (total_value > 0),
    executed_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    settlement_status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (settlement_status IN ('pending', 'settled', 'failed'))
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_orders_account_id ON trading.orders(account_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON trading.orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_type ON trading.orders(order_type);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON trading.orders(created_at);
CREATE INDEX IF NOT EXISTS idx_trades_buy_order ON trading.trades(buy_order_id);
CREATE INDEX IF NOT EXISTS idx_trades_sell_order ON trading.trades(sell_order_id);
CREATE INDEX IF NOT EXISTS idx_trades_executed_at ON trading.trades(executed_at);
