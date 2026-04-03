-- Add up migration script here
CREATE TYPE user_status AS ENUM ('EmailNotVerified', 'Active', 'Banned');

CREATE TABLE "users"
(
    id          UUID         NOT NULL PRIMARY KEY,
    name        VARCHAR(31)  NOT NULL,
    email       VARCHAR(254) NOT NULL,
    credentials JSONB        NOT NULL,
    status      user_status  NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
