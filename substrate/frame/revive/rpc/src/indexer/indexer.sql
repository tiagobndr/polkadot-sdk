CREATE TABLE tx_hashes (
  transaction_hash CHAR(64) NOT NULL PRIMARY KEY,
  block_hash CHAR(64) NOT NULL,
  transaction_index INTEGER NOT NULL,
  block_number BIGINT NOT NULL
);
