-- Add migration script here
ALTER TABLE services RENAME COLUMN category TO old_category;

ALTER TABLE services
  ADD COLUMN categories TEXT[] NOT NULL DEFAULT '{}';

UPDATE services
SET categories = ARRAY[old_category];

ALTER TABLE services DROP COLUMN old_category;

-- down