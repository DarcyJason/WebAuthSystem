-- Add up migration script here
CREATE TYPE verification_token_kind AS ENUM ('EmailVerification', 'PasswordReset');

CREATE TABLE verification_tokens (
    id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    value VARCHAR(255) NOT NULL UNIQUE,
    kind verification_token_kind NOT NULL,
    used BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_verification_tokens_user_id ON verification_tokens(user_id);
