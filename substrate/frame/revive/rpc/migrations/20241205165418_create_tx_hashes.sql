-- Create DB:
--  DATABASE_URL="sqlite:$(pwd)/tx_hashes.db" cargo sqlx database create
--
-- Run migration:
--  DATABASE_URL="sqlite:$(pwd)/tx_hashes.db" cargo sqlx migration run
--
-- Update compile time artifacts:
--  DATABASE_URL="sqlite:$(pwd)/tx_hashes.db" cargo sqlx prepare
CREATE TABLE tx_hashes (
  transaction_hash CHAR(64) NOT NULL PRIMARY KEY,
  transaction_index INTEGER NOT NULL,
  block_hash CHAR(64) NOT NULL
);

-- Index for queries involving block_hash and transaction_index
CREATE INDEX idx_block_hash_tx_index ON tx_hashes (block_hash, transaction_index);
