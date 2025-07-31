-- Add migration script here
ALTER TABLE users
ADD COLUMN phone VARCHAR(20) NOT NULL DEFAULT '';

-- down