--- ================================================
--- Affiliate System
--- ================================================
CREATE SCHEMA IF NOT EXISTS affiliate;

--- Affiliate Log
CREATE TABLE IF NOT EXISTS affiliate.log (
    id SERIAL PRIMARY KEY,
    parent UUID NOT NULL,
    child UUID NOT NULL,
    reward REAL NOT NULL,
    rate REAL NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--- Affiliate Statics
CREATE TABLE IF NOT EXISTS affiliate.statics (
    user_id UUID PRIMARY KEY;
    invite_code VARCHAR(32) NOT NULL,
    total REAL NOT NULL DEFAULT 0,
    withdraw REAL NOT NULL DEFAULT 0,
    count_referral INT NOT NULL DEFAULT 0,
    invited_by UUID NULLABLE,
    rate REAL NOT NULL,
);

CREATE INDEX IF NOT EXISTS affiliate_statics_user_id ON affiliate.statics (user_id);