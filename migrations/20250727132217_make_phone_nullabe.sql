-- Add migration script here
ALTER TABLE users
ALTER COLUMN phone DROP NOT NULL;

UPDATE users
   SET phone = NULL
 WHERE phone = '';
 
 CREATE UNIQUE INDEX IF NOT EXISTS uniq_users_phone_notnull
     ON users (phone)
  WHERE phone IS NOT NULL;

-- down