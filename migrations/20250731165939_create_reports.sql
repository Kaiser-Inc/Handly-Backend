-- up
CREATE TABLE IF NOT EXISTS reports (
    id            UUID PRIMARY KEY        DEFAULT gen_random_uuid(),
    reporter_key  VARCHAR(14)  NOT NULL,
    target_type   VARCHAR(10)  NOT NULL CHECK (target_type IN ('service','user')),
    target_id     VARCHAR(36)  NOT NULL,
    reason_code   VARCHAR(20)  NOT NULL,
    description   TEXT,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_indexes WHERE indexname = 'uniq_reporter_target'
    ) THEN
        CREATE UNIQUE INDEX uniq_reporter_target
            ON reports (reporter_key, target_type, target_id);
    END IF;
END$$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.table_constraints
        WHERE constraint_name = 'fk_reports_users'
    ) THEN
        ALTER TABLE reports
            ADD CONSTRAINT fk_reports_users
                FOREIGN KEY (reporter_key) REFERENCES users (cpf_cnpj)
                ON DELETE CASCADE;
    END IF;
END$$;

-- down
