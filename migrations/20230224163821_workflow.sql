-- Add migration script here
CREATE TABLE IF NOT EXISTS workflow (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    project_id UUID NOT NULL REFERENCES project(id),
    description VARCHAR,
    paused BOOLEAN,
    UNIQUE(project_id, name) INCLUDE (id)
);
