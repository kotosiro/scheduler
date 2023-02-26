-- Add migration script here
CREATE TABLE IF NOT EXISTS run (
    id UUID PRIMARY KEY,
    state VARCHAR,
    priority VARCHAR NOT NULL,
    job_id UUID NOT NULL REFERENCES job(id),
    /*runner_id UUID REFERENCES runner(id),*/
    /*triggered_at TIMESTAMP NOT NULL,*/
    queued_at TIMESTAMP WITH TIME ZONE NOT NULL,
    started_at TIMESTAMP WITH TIME ZONE ,
    finished_at TIMESTAMP WITH TIME ZONE ,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL default CURRENT_TIMESTAMP
    /*created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP*/
);
