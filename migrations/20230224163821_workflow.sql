-- Add migration script here
CREATE TABLE IF NOT EXISTS workflow (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    project_id UUID NOT NULL REFERENCES project(id),
    description VARCHAR,
    paused BOOLEAN,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE(project_id, name) INCLUDE (id)
);
