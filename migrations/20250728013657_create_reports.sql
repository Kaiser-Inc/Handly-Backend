-- Add migration script here
-- up -------------------------------------------------------------------------
CREATE TABLE reports (
    id            UUID PRIMARY KEY        DEFAULT gen_random_uuid(),
    reporter_key  VARCHAR(14)  NOT NULL,
    target_type   VARCHAR(10)  NOT NULL CHECK (target_type IN ('service','user')),
    target_id     UUID         NOT NULL,
    reason_code   VARCHAR(20)  NOT NULL,
    description   TEXT,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX uniq_reporter_target
    ON reports (reporter_key, target_type, target_id);

ALTER TABLE reports
    ADD CONSTRAINT fk_reports_users FOREIGN KEY (reporter_key)
        REFERENCES users(cpf_cnpj) ON DELETE CASCADE;

-- down -----------------------------------------------------------------------
