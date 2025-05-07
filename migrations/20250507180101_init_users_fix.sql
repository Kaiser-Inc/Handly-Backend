-- Add migration script here
-- up
CREATE TABLE users (
    id        UUID PRIMARY KEY,
    name      TEXT NOT NULL,
    email     TEXT UNIQUE NOT NULL,
    password  TEXT NOT NULL,
    role      TEXT NOT NULL DEFAULT 'customer',
    cpf_cnpj  TEXT UNIQUE
);

-- down
