-- Add up migration script here
-- Roda para fazer a alteração que eu quero

CREATE TABLE IF NOT EXISTS assets (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    unit_value DOUBLE PRECISION NOT NULL
);