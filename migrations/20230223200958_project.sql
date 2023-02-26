-- Add migration script here
CREATE TABLE IF NOT EXISTS project (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR,
    config JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
);
