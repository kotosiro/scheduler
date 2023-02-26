-- Add migration script here
CREATE TABLE IF NOT EXISTS token (
    job_id UUID NOT NULL REFERENCES job(id),
    count INT,
    state VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE(job_id, created_at)
);
