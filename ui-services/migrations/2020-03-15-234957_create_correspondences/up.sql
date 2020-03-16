-- Your SQL goes here
CREATE TABLE corrs (
  id SERIAL PRIMARY KEY,
  corr_name VARCHAR NOT NULL,
  CONSTRAINT uk_corr_name UNIQUE (corr_name)
)
