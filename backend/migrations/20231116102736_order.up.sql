-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS orders (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        table_number SMALLINT NOT NULL,
        item TEXT NOT NULL,
        cook_time SMALLINT NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
