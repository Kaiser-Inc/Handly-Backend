-- Add migration script here
-- up
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE services (
    id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    provider_key TEXT        NOT NULL,
    category     TEXT        NOT NULL,
    name         TEXT        NOT NULL,
    description  TEXT        NOT NULL,
    image        TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    FOREIGN KEY (provider_key) REFERENCES users(cpf_cnpj)
);

-- down
