-- Add migration script here
CREATE TABLE provider_ratings (
    id           UUID PRIMARY KEY,
    provider_id  TEXT  NOT NULL
        REFERENCES users(cpf_cnpj) ON DELETE CASCADE,
    user_id      TEXT  NOT NULL
        REFERENCES users(cpf_cnpj) ON DELETE CASCADE,
    stars        SMALLINT NOT NULL CHECK (stars BETWEEN 1 AND 5),
    comment      TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT one_provider_rating_per_user UNIQUE (provider_id, user_id)
);

-- down