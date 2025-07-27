-- Add migration script here
ALTER TABLE ratings
ADD time TIMESTAMPTZ NOT NULL;