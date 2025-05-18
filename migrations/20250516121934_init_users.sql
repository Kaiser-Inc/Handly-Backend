-- Add migration script here
-- up
CREATE TABLE users (
    cpf_cnpj  TEXT PRIMARY KEY,
    name      TEXT NOT NULL,
    email     TEXT UNIQUE NOT NULL,
    password  TEXT NOT NULL,
    role      TEXT NOT NULL  -- 'customer' | 'provider'
);

-- down
