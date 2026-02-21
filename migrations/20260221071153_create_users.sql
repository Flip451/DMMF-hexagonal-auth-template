-- Create users table with business and common columns
CREATE TABLE users (
    -- Primary Key
    id UUID PRIMARY KEY,

    -- Business Columns
    email VARCHAR(255) NOT NULL,
    password_hash TEXT NOT NULL,

    -- Common Columns (Creation)
    created_at TIMESTAMPTZ NOT NULL,
    created_by VARCHAR(255) NOT NULL,
    created_pgm_cd VARCHAR(255) NOT NULL,
    created_tx_id VARCHAR(255) NOT NULL,

    -- Common Columns (Update)
    updated_at TIMESTAMPTZ NOT NULL,
    updated_by VARCHAR(255) NOT NULL,
    updated_pgm_cd VARCHAR(255) NOT NULL,
    updated_tx_id VARCHAR(255) NOT NULL,

    -- Common Columns (Optimistic Locking)
    lock_no INTEGER NOT NULL DEFAULT 1,

    -- Common Columns (Patch)
    patched_at TIMESTAMPTZ,
    patched_by VARCHAR(255),
    patched_id VARCHAR(255)
);

-- Use UNIQUE INDEX instead of UNIQUE CONSTRAINT for email
CREATE UNIQUE INDEX idx_users_email_unique ON users(email);
