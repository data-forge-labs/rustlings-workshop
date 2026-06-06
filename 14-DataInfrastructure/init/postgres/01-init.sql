-- PostgreSQL initialization for Data Infrastructure section
-- This runs on first container startup

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS pg_stat_statements;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create a schema for our data engineering examples
CREATE SCHEMA IF NOT EXISTS dataeng;

-- Sample table for CDC demonstrations
CREATE TABLE IF NOT EXISTS dataeng.orders (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id     UUID NOT NULL,
    product_id      UUID NOT NULL,
    quantity        INT NOT NULL CHECK (quantity > 0),
    unit_price      NUMERIC(12,2) NOT NULL CHECK (unit_price >= 0),
    total_price     NUMERIC(12,2) GENERATED ALWAYS AS (quantity * unit_price) STORED,
    status          VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','paid','shipped','cancelled')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Index for common query patterns
CREATE INDEX IF NOT EXISTS idx_orders_customer_id ON dataeng.orders(customer_id);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON dataeng.orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_orders_status ON dataeng.orders(status);

-- Sample table for ClickHouse sync demonstrations
CREATE TABLE IF NOT EXISTS dataeng.events (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    event_type      VARCHAR(50) NOT NULL,
    payload         JSONB NOT NULL,
    source          VARCHAR(50) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_events_type ON dataeng.events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_created_at ON dataeng.events(created_at DESC);

-- Outbox table for transactional outbox pattern (Project 01)
CREATE TABLE IF NOT EXISTS dataeng.outbox (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    aggregate_type  VARCHAR(100) NOT NULL,
    aggregate_id    UUID NOT NULL,
    event_type      VARCHAR(100) NOT NULL,
    payload         JSONB NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at    TIMESTAMPTZ,
    retry_count     INT NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_outbox_unprocessed ON dataeng.outbox(processed_at) WHERE processed_at IS NULL;

-- DuckLake catalog tables (created by DuckLake automatically, but we pre-create the schema)
CREATE SCHEMA IF NOT EXISTS ducklake_catalog;

-- Grant permissions
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA dataeng TO dataeng;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA dataeng TO dataeng;
GRANT USAGE ON SCHEMA ducklake_catalog TO dataeng;

-- Insert sample data for testing
INSERT INTO dataeng.orders (customer_id, product_id, quantity, unit_price, status)
SELECT
    uuid_generate_v4(),
    uuid_generate_v4(),
    (random() * 10 + 1)::int,
    (random() * 100 + 10)::numeric(12,2),
    (ARRAY['pending','paid','shipped','cancelled'])[floor(random() * 4 + 1)]
FROM generate_series(1, 100);

INSERT INTO dataeng.events (event_type, payload, source)
SELECT
    (ARRAY['page_view','click','purchase','signup','login'])[floor(random() * 5 + 1)],
    jsonb_build_object(
        'user_id', uuid_generate_v4(),
        'session_id', uuid_generate_v4(),
        'value', (random() * 1000)::int
    ),
    (ARRAY['web','mobile','api'])[floor(random() * 3 + 1)]
FROM generate_series(1, 1000);