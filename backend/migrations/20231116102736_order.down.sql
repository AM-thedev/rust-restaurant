-- Add down migration script here

DROP TABLE IF EXISTS "orders";

DROP EXTENSION IF EXISTS "uuid-ossp";