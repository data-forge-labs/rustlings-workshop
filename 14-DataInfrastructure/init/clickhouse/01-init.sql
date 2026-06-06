-- ClickHouse initialization for Data Infrastructure section
-- Runs on first container startup

-- Create analytics schema
CREATE DATABASE IF NOT EXISTS analytics;

-- Orders table (mirror of PostgreSQL orders)
CREATE TABLE IF NOT EXISTS analytics.orders
(
    id              UUID,
    customer_id     UUID,
    product_id      UUID,
    quantity        UInt32,
    unit_price      Decimal(12, 2),
    total_price     Decimal(12, 2),
    status          LowCardinality(String),
    created_at      DateTime64(3, 'UTC'),
    updated_at      DateTime64(3, 'UTC'),
    inserted_at     DateTime DEFAULT now()
)
ENGINE = MergeTree()
PARTITION BY toYYYYMM(created_at)
ORDER BY (created_at, id)
SETTINGS index_granularity = 8192;

-- Events table for high-throughput ingestion
CREATE TABLE IF NOT EXISTS analytics.events
(
    id              UUID,
    event_type      LowCardinality(String),
    source          LowCardinality(String),
    payload         String,  -- JSON as string
    created_at      DateTime64(3, 'UTC'),
    inserted_at     DateTime DEFAULT now()
)
ENGINE = MergeTree()
PARTITION BY toYYYYMM(created_at)
ORDER BY (created_at, event_type)
SETTINGS index_granularity = 8192;

-- Aggregated view for real-time analytics
CREATE MATERIALIZED VIEW IF NOT EXISTS analytics.events_per_minute_mv
ENGINE = SummingMergeTree()
ORDER BY (event_type, minute)
POPULATE AS
SELECT
    event_type,
    toStartOfMinute(created_at) AS minute,
    count() AS event_count
FROM analytics.events
GROUP BY event_type, minute;

-- Kafka-style landing table (consumed via ClickHouse Kafka engine in some demos)
CREATE TABLE IF NOT EXISTS analytics.kafka_landing
(
    topic       String,
    partition   Int32,
    offset      Int64,
    key         String,
    value       String,
    timestamp   DateTime,
    consumed_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (topic, timestamp)
SETTINGS index_granularity = 8192;

-- Iggy-style landing table
CREATE TABLE IF NOT EXISTS analytics.iggy_landing
(
    stream_id   UInt32,
    topic_id    UInt32,
    partition_id UInt32,
    offset      UInt64,
    key         String,
    value       String,
    produced_at DateTime64(3, 'UTC'),
    consumed_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
ORDER BY (stream_id, topic_id, partition_id, produced_at)
SETTINGS index_granularity = 8192;

-- User grant
GRANT ALL ON analytics.* TO dataeng;