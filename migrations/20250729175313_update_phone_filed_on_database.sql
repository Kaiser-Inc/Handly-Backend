-- Add migration script here
ALTER TABLE users
    ALTER COLUMN phone DROP DEFAULT;

UPDATE users
   SET phone = NULL
 WHERE phone = '';

DO $$
BEGIN
    IF EXISTS (SELECT 1
                 FROM pg_indexes
                WHERE schemaname = 'public'
                  AND indexname  = 'uniq_users_phone_notnull') THEN
        DROP INDEX uniq_users_phone_notnull;
    END IF;
END$$;

CREATE UNIQUE INDEX uniq_users_phone_notnull
    ON users(phone)
    WHERE phone IS NOT NULL;

-- down