-- Add migration script here
-- up
ALTER TABLE users
ADD COLUMN profile_pic TEXT NULL;

-- down