-- Add migration script here
CREATE TABLE IF NOT EXISTS job (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    workflow_id UUID NOT NULL REFERENCES workflow(id),
    threshold INT,
    image VARCHAR,
    args VARCHAR[],
    envs VARCHAR[],
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE(workflow_id, name) INCLUDE (id)
);
