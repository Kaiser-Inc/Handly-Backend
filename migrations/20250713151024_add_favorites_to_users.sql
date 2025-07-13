-- Add migration script here
ALTER TABLE users
    ADD COLUMN favorites JSONB NOT NULL DEFAULT '[]';

    -- down