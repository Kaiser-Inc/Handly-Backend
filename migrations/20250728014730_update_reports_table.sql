-- Add migration script here
ALTER TABLE reports
    ALTER COLUMN target_id TYPE VARCHAR(36)
    USING target_id::text;

-- down